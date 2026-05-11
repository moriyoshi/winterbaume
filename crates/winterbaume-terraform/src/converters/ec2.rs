//! Terraform converters for EC2 resources (VPC, Subnet, IGW, Route Table, Security Group, etc.).
//!
//! The 39 `*TfModel` structs imported from `crate::generated::ec2` are produced
//! from `specs/ec2.toml`. Each `do_inject` deserialises the model, then layers
//! on the hand-written ARN/URL templates, snapshot lookups (subnet -> vpc_id,
//! ipam_scope -> ipam_arn, etc.), tag merging, and nested-block / Vec<T>
//! parsing that the spec deliberately omits. Every `do_extract` is preserved
//! byte-identically with the pre-codegen converter.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_ec2::Ec2Service;
use winterbaume_ec2::views::{
    AwsNetworkPerformanceSubscriptionView, CapacityBlockView, CapacityReservationView,
    CarrierGatewayView, ClientVpnAssociationStatusView, ClientVpnAuthorizationRuleStatusView,
    ClientVpnAuthorizationRuleView, ClientVpnEndpointStatusView, ClientVpnEndpointView,
    ClientVpnRouteStatusView, ClientVpnRouteView, ClientVpnTargetNetworkAssociationView,
    CustomerGatewayView, DedicatedHostView, DhcpOptionsView, Ec2FleetView, Ec2StateView,
    EgressOnlyIgwView, ElasticIpView, EoigwAttachmentView, FlowLogView, IcmpTypeCodeView,
    IgwAttachmentView, ImageView, InstanceConnectEndpointView, InstanceStateView, InstanceView,
    InternetGatewayView, IpPermissionView, IpRangeView, IpamOperatingRegionView,
    IpamPoolAllocationView, IpamPoolCidrView, IpamPoolView, IpamResourceDiscoveryAssociationView,
    IpamResourceDiscoveryView, IpamScopeView, IpamView, Ipv6RangeView, KeyPairView,
    LaunchPermissionView, LaunchTemplateView, LocalGatewayRouteTableVpcAssociationView,
    LocalGatewayRouteView, ManagedPrefixListView, MulticastSubnetAssociationView, NatGatewayView,
    NetworkAclAssociationView, NetworkAclEntryView, NetworkAclView, NetworkInsightsAnalysisView,
    NetworkInsightsPathView, NetworkInterfacePermissionView, NetworkInterfaceView,
    PlacementGroupView, PortRangeView, PrefixListEntryView, RouteServerAssociationView,
    RouteTableAssociationView, RouteTableView, RouteView, SecurityGroupView,
    SecurityGroupVpcAssociationView, SpotFleetRequestView, SpotInstanceRequestView,
    SubnetCidrReservationView, SubnetIpv6CidrAssocView, SubnetView, TgwPeeringAttachmentView,
    TgwRouteTableView, TgwRouteView, TgwVpcAttachmentView, TrafficMirrorFilterRuleView,
    TrafficMirrorFilterView, TrafficMirrorPortRangeView, TrafficMirrorSessionView,
    TrafficMirrorTargetView, TransitGatewayConnectPeerView, TransitGatewayConnectView,
    TransitGatewayMulticastDomainAssociationView, TransitGatewayMulticastDomainView,
    TransitGatewayMulticastGroupMemberView, TransitGatewayMulticastGroupSourceView,
    TransitGatewayPolicyTableAssociationView, TransitGatewayPolicyTableView,
    TransitGatewayPrefixListReferenceView, TransitGatewayView, UserIdGroupPairView,
    VerifiedAccessEndpointView, VerifiedAccessGroupView, VerifiedAccessInstanceView,
    VerifiedAccessSseSpecificationView, VerifiedAccessTrustProviderAttachmentView,
    VerifiedAccessTrustProviderView, VgwVpcAttachmentView, VolumeAttachmentView,
    VpcBlockPublicAccessExclusionView, VpcBlockPublicAccessOptionsView,
    VpcEndpointConnectionNotificationView, VpcEndpointConnectionView, VpcEndpointServiceConfigView,
    VpcEndpointView, VpcPeeringConnectionOptionsView, VpcPeeringConnectionView, VpcView,
    VpnConnectionOptionsView, VpnConnectionView, VpnGatewayView, VpnStaticRouteView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::ec2 as ec2_gen;
use crate::util::{
    classify_deserialize_error, extract_region, extract_tags, optional_bool, optional_i64,
    optional_str,
};

// ---------------------------------------------------------------------------
// aws_vpc
// ---------------------------------------------------------------------------

pub struct AwsVpcConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpc", e))?;

        let cidr_block = model.cidr_block;
        let vpc_id = model
            .id
            .unwrap_or_else(|| format!("vpc-{}", &uuid::Uuid::new_v4().to_string()[..17]));
        let tags = extract_tags(attrs);

        // Parse secondary CIDR blocks from TF state (list of association_id+cidr pairs).
        let secondary_cidr_blocks: Vec<(String, String)> = attrs
            .get("secondary_cidr_blocks")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|entry| {
                        let assoc = entry
                            .get("association_id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let cidr = entry
                            .get("cidr_block")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        if cidr.is_empty() {
                            None
                        } else {
                            Some((assoc, cidr))
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        let vpc_view = VpcView {
            vpc_id: vpc_id.clone(),
            cidr_block,
            state: "available".to_string(),
            dhcp_options_id: model
                .dhcp_options_id
                .unwrap_or_else(|| "dopt-default".to_string()),
            instance_tenancy: model
                .instance_tenancy
                .unwrap_or_else(|| "default".to_string()),
            is_default: model.default_,
            enable_dns_hostnames: model.enable_dns_hostnames,
            enable_dns_support: model.enable_dns_support,
            tags,
            secondary_cidr_blocks,
            classic_link_enabled: false,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.vpcs.insert(vpc_id, vpc_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for vpc in view.vpcs.values() {
            // Build secondary_cidr_blocks array for TF state.
            let secondary_cidrs: Vec<serde_json::Value> = vpc
                .secondary_cidr_blocks
                .iter()
                .map(|(assoc_id, cidr)| {
                    serde_json::json!({
                        "association_id": assoc_id,
                        "cidr_block": cidr,
                    })
                })
                .collect();

            // Derive default NACL / route table / security group IDs from state
            // if available.
            let default_network_acl_id = view
                .network_acls
                .values()
                .find(|nacl| nacl.vpc_id == vpc.vpc_id && nacl.is_default)
                .map(|nacl| nacl.network_acl_id.as_str())
                .unwrap_or("");

            let default_route_table_id = view
                .route_tables
                .values()
                .find(|rtb| rtb.vpc_id == vpc.vpc_id && rtb.associations.iter().any(|a| a.main))
                .map(|rtb| rtb.route_table_id.as_str())
                .unwrap_or("");

            let default_security_group_id = view
                .security_groups
                .values()
                .find(|sg| sg.vpc_id == vpc.vpc_id && sg.group_name == "default")
                .map(|sg| sg.group_id.as_str())
                .unwrap_or("");

            let main_route_table_id = default_route_table_id;

            let attrs = serde_json::json!({
                "id": vpc.vpc_id,
                "cidr_block": vpc.cidr_block,
                "state": vpc.state,
                "instance_tenancy": vpc.instance_tenancy,
                "enable_dns_hostnames": vpc.enable_dns_hostnames,
                "enable_dns_support": vpc.enable_dns_support,
                "dhcp_options_id": vpc.dhcp_options_id,
                "default": vpc.is_default,
                "owner_id": ctx.default_account_id,
                "main_route_table_id": main_route_table_id,
                "default_network_acl_id": default_network_acl_id,
                "default_route_table_id": default_route_table_id,
                "default_security_group_id": default_security_group_id,
                "secondary_cidr_blocks": secondary_cidrs,
                "arn": format!("arn:aws:ec2:{}:{}:vpc/{}", ctx.default_region, ctx.default_account_id, vpc.vpc_id),
                "tags": vpc.tags,
            });
            results.push(ExtractedResource {
                name: vpc.vpc_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_subnet
// ---------------------------------------------------------------------------

pub struct AwsSubnetConverter {
    service: Arc<Ec2Service>,
}

impl AwsSubnetConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSubnetConverter {
    fn resource_type(&self) -> &str {
        "aws_subnet"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsSubnetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::SubnetTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_subnet", e))?;

        let vpc_id = model.vpc_id;
        let cidr_block = model.cidr_block;
        let subnet_id = model
            .id
            .unwrap_or_else(|| format!("subnet-{}", &uuid::Uuid::new_v4().to_string()[..17]));
        let az = model
            .availability_zone
            .unwrap_or_else(|| format!("{}a", region));
        let tags = extract_tags(attrs);

        // Parse IPv6 CIDR block associations from TF state.
        let ipv6_cidr_blocks: Vec<SubnetIpv6CidrAssocView> = attrs
            .get("ipv6_cidr_block_associations")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|entry| {
                        let cidr = entry
                            .get("ipv6_cidr_block")
                            .and_then(|v| v.as_str())?
                            .to_string();
                        let assoc_id = entry
                            .get("association_id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        Some(SubnetIpv6CidrAssocView {
                            association_id: assoc_id,
                            ipv6_cidr_block: cidr,
                            state: "associated".to_string(),
                        })
                    })
                    .collect()
            })
            .or_else(|| {
                // Fallback: single ipv6_cidr_block attribute
                optional_str(attrs, "ipv6_cidr_block").map(|cidr| {
                    vec![SubnetIpv6CidrAssocView {
                        association_id: String::new(),
                        ipv6_cidr_block: cidr,
                        state: "associated".to_string(),
                    }]
                })
            })
            .unwrap_or_default();

        let subnet_view = SubnetView {
            subnet_id: subnet_id.clone(),
            vpc_id,
            cidr_block,
            availability_zone: az,
            state: "available".to_string(),
            available_ip_address_count: 251,
            map_public_ip_on_launch: model.map_public_ip_on_launch,
            tags,
            ipv6_cidr_blocks,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.subnets.insert(subnet_id, subnet_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for subnet in view.subnets.values() {
            let ipv6_assocs: Vec<serde_json::Value> = subnet
                .ipv6_cidr_blocks
                .iter()
                .map(|a| {
                    serde_json::json!({
                        "association_id": a.association_id,
                        "ipv6_cidr_block": a.ipv6_cidr_block,
                        "state": a.state,
                    })
                })
                .collect();

            let assign_ipv6 = !subnet.ipv6_cidr_blocks.is_empty();

            let attrs = serde_json::json!({
                "id": subnet.subnet_id,
                "vpc_id": subnet.vpc_id,
                "cidr_block": subnet.cidr_block,
                "availability_zone": subnet.availability_zone,
                "availability_zone_id": format!("{}z", subnet.availability_zone),
                "state": subnet.state,
                "available_ip_address_count": subnet.available_ip_address_count,
                "map_public_ip_on_launch": subnet.map_public_ip_on_launch,
                "assign_ipv6_address_on_creation": assign_ipv6,
                "ipv6_cidr_block_associations": ipv6_assocs,
                "owner_id": ctx.default_account_id,
                "arn": format!("arn:aws:ec2:{}:{}:subnet/{}", ctx.default_region, ctx.default_account_id, subnet.subnet_id),
                "tags": subnet.tags,
                "tags_all": subnet.tags,
                "private_dns_hostname_type_on_launch": "ip-name",
            });
            results.push(ExtractedResource {
                name: subnet.subnet_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_internet_gateway
// ---------------------------------------------------------------------------

pub struct AwsInternetGatewayConverter {
    service: Arc<Ec2Service>,
}

impl AwsInternetGatewayConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsInternetGatewayConverter {
    fn resource_type(&self) -> &str {
        "aws_internet_gateway"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsInternetGatewayConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::InternetGatewayTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_internet_gateway", e))?;

        let igw_id = model
            .id
            .unwrap_or_else(|| format!("igw-{}", &uuid::Uuid::new_v4().to_string()[..17]));
        let vpc_id = model.vpc_id;
        let mut tags = extract_tags(attrs);
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let attachments = if let Some(vpc) = vpc_id {
            vec![IgwAttachmentView {
                vpc_id: vpc,
                state: "available".to_string(),
            }]
        } else {
            vec![]
        };

        let igw_view = InternetGatewayView {
            igw_id: igw_id.clone(),
            attachments,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.igws.insert(igw_id, igw_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for igw in view.igws.values() {
            let vpc_id = igw
                .attachments
                .first()
                .map(|a| a.vpc_id.as_str())
                .unwrap_or("");
            let attrs = serde_json::json!({
                "id": igw.igw_id,
                "vpc_id": vpc_id,
                "owner_id": ctx.default_account_id,
                "arn": format!("arn:aws:ec2:{}:{}:internet-gateway/{}", ctx.default_region, ctx.default_account_id, igw.igw_id),
                "tags": igw.tags,
                "tags_all": igw.tags,
            });
            results.push(ExtractedResource {
                name: igw.igw_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route_table
// ---------------------------------------------------------------------------

pub struct AwsRouteTableConverter {
    service: Arc<Ec2Service>,
}

impl AwsRouteTableConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRouteTableConverter {
    fn resource_type(&self) -> &str {
        "aws_route_table"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsRouteTableConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::RouteTableTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_route_table", e))?;

        let vpc_id = model.vpc_id;
        let rtb_id = model
            .id
            .unwrap_or_else(|| format!("rtb-{}", &uuid::Uuid::new_v4().to_string()[..17]));
        let tags = extract_tags(attrs);

        // Inline route blocks (if any)
        let routes = parse_route_blocks(attrs);

        // Parse associations if present in the TF state.
        let associations = parse_route_table_associations(attrs);

        let rtb_view = RouteTableView {
            route_table_id: rtb_id.clone(),
            vpc_id,
            routes,
            associations,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.route_tables.insert(rtb_id, rtb_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for rtb in view.route_tables.values() {
            let routes: Vec<serde_json::Value> = rtb
                .routes
                .iter()
                .map(|r| {
                    // Determine whether the target is an IGW or NAT GW by prefix.
                    let (gw_id, nat_gw_id) = match &r.gateway_id {
                        Some(id) if id.starts_with("nat-") => (
                            serde_json::Value::Null,
                            serde_json::Value::String(id.clone()),
                        ),
                        Some(id) => (
                            serde_json::Value::String(id.clone()),
                            serde_json::Value::Null,
                        ),
                        None => (serde_json::Value::Null, serde_json::Value::Null),
                    };
                    serde_json::json!({
                        "cidr_block": r.destination_cidr_block,
                        "ipv6_cidr_block": r.destination_ipv6_cidr_block,
                        "gateway_id": gw_id,
                        "nat_gateway_id": nat_gw_id,
                        "state": r.state,
                        "origin": r.origin,
                    })
                })
                .collect();

            let associations: Vec<serde_json::Value> = rtb
                .associations
                .iter()
                .map(|a| {
                    serde_json::json!({
                        "route_table_association_id": a.association_id,
                        "route_table_id": rtb.route_table_id,
                        "subnet_id": a.subnet_id,
                        "main": a.main,
                    })
                })
                .collect();

            let attrs = serde_json::json!({
                "id": rtb.route_table_id,
                "vpc_id": rtb.vpc_id,
                "owner_id": ctx.default_account_id,
                "route": routes,
                "associations": associations,
                "arn": format!("arn:aws:ec2:{}:{}:route-table/{}", ctx.default_region, ctx.default_account_id, rtb.route_table_id),
                "tags": rtb.tags,
            });
            results.push(ExtractedResource {
                name: rtb.route_table_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route
// ---------------------------------------------------------------------------

pub struct AwsRouteConverter {
    service: Arc<Ec2Service>,
}

impl AwsRouteConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRouteConverter {
    fn resource_type(&self) -> &str {
        "aws_route"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_route_table", "aws_internet_gateway", "aws_nat_gateway"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsRouteConverter {
    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for rtb in view.route_tables.values() {
            for route in &rtb.routes {
                // Determine gateway_id vs nat_gateway_id by prefix.
                let (gw_id, nat_gw_id) = match &route.gateway_id {
                    Some(id) if id.starts_with("nat-") => {
                        (serde_json::Value::Null, serde_json::json!(id))
                    }
                    Some(id) => (serde_json::json!(id), serde_json::Value::Null),
                    None => (serde_json::Value::Null, serde_json::Value::Null),
                };
                // Build a unique name from route_table_id + destination
                let dest = route
                    .destination_cidr_block
                    .as_deref()
                    .or(route.destination_ipv6_cidr_block.as_deref())
                    .unwrap_or("unknown");
                let name = format!("{}_{}", rtb.route_table_id, dest);
                let attrs = serde_json::json!({
                    "id": name,
                    "route_table_id": rtb.route_table_id,
                    "destination_cidr_block": route.destination_cidr_block,
                    "destination_ipv6_cidr_block": route.destination_ipv6_cidr_block,
                    "gateway_id": gw_id,
                    "nat_gateway_id": nat_gw_id,
                    "state": route.state,
                    "origin": route.origin,
                    "carrier_gateway_id": null,
                    "transit_gateway_id": null,
                    "vpc_endpoint_id": null,
                });
                results.push(ExtractedResource {
                    name,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }

    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::RouteTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_route", e))?;

        let route_table_id = model.route_table_id;
        let target = model.gateway_id.or(model.nat_gateway_id);

        let route_view = RouteView {
            destination_cidr_block: model.destination_cidr_block,
            destination_ipv6_cidr_block: model.destination_ipv6_cidr_block,
            gateway_id: target,
            state: "active".to_string(),
            origin: "CreateRoute".to_string(),
        };

        // We need to add this route to the existing route table.
        // Use a minimal state view with just this route table containing only the new route.
        // The merge will insert the route table, but we need the route table to exist first.
        // Since we depend on aws_route_table, we can snapshot and append.
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        let mut state_view = minimal_ec2_state_view();
        if let Some(mut rtb) = snapshot.route_tables.get(&route_table_id).cloned() {
            rtb.routes.push(route_view);
            state_view.route_tables.insert(route_table_id, rtb);
        } else {
            // Route table not yet in state; create a minimal one with just this route
            let rtb_view = RouteTableView {
                route_table_id: route_table_id.clone(),
                vpc_id: String::new(),
                routes: vec![route_view],
                associations: vec![],
                tags: HashMap::new(),
            };
            state_view.route_tables.insert(route_table_id, rtb_view);
        }

        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_security_group
// ---------------------------------------------------------------------------

pub struct AwsSecurityGroupConverter {
    service: Arc<Ec2Service>,
}

impl AwsSecurityGroupConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_security_group"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsSecurityGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::SecurityGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_security_group", e))?;

        let name = model.name;
        let sg_id = model
            .id
            .unwrap_or_else(|| format!("sg-{}", &uuid::Uuid::new_v4().to_string()[..17]));
        let vpc_id = model.vpc_id.unwrap_or_default();
        let description = model
            .description
            .unwrap_or_else(|| "Managed by Terraform".to_string());
        let tags = extract_tags(attrs);

        let ingress_rules = parse_sg_rules(attrs, "ingress");
        let egress_rules = parse_sg_rules(attrs, "egress");

        let sg_view = SecurityGroupView {
            group_id: sg_id.clone(),
            group_name: name,
            description,
            vpc_id,
            owner_id: ctx.default_account_id.clone(),
            ingress_rules,
            egress_rules,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.security_groups.insert(sg_id, sg_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for sg in view.security_groups.values() {
            let ingress: Vec<serde_json::Value> =
                sg.ingress_rules.iter().map(sg_rule_to_json).collect();
            let egress: Vec<serde_json::Value> =
                sg.egress_rules.iter().map(sg_rule_to_json).collect();
            let attrs = serde_json::json!({
                "id": sg.group_id,
                "name": sg.group_name,
                "description": sg.description,
                "vpc_id": sg.vpc_id,
                "owner_id": sg.owner_id,
                "ingress": ingress,
                "egress": egress,
                "arn": format!("arn:aws:ec2:{}:{}:security-group/{}", ctx.default_region, ctx.default_account_id, sg.group_id),
                "tags": sg.tags,
            });
            results.push(ExtractedResource {
                name: sg.group_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_key_pair
// ---------------------------------------------------------------------------

pub struct AwsKeyPairConverter {
    service: Arc<Ec2Service>,
}

impl AwsKeyPairConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKeyPairConverter {
    fn resource_type(&self) -> &str {
        "aws_key_pair"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsKeyPairConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::KeyPairTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_key_pair", e))?;

        let key_name = model.key_name;
        let key_pair_id = model
            .id
            .or(model.key_pair_id)
            .unwrap_or_else(|| format!("key-{}", &uuid::Uuid::new_v4().to_string()[..17]));
        let fingerprint = model.fingerprint.unwrap_or_default();
        let tags = extract_tags(attrs);

        let kp_view = KeyPairView {
            key_pair_id: key_pair_id.clone(),
            key_name: key_name.clone(),
            fingerprint,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.key_pairs.insert(key_name, kp_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for kp in view.key_pairs.values() {
            let attrs = serde_json::json!({
                "id": kp.key_pair_id,
                "key_pair_id": kp.key_pair_id,
                "key_name": kp.key_name,
                "fingerprint": kp.fingerprint,
                "arn": format!("arn:aws:ec2:{}:{}:key-pair/{}", ctx.default_region, ctx.default_account_id, kp.key_pair_id),
                "tags": kp.tags,
                "tags_all": kp.tags,
            });
            results.push(ExtractedResource {
                name: kp.key_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_eip
// ---------------------------------------------------------------------------

pub struct AwsEipConverter {
    service: Arc<Ec2Service>,
}

impl AwsEipConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEipConverter {
    fn resource_type(&self) -> &str {
        "aws_eip"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEipConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::EipTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_eip", e))?;

        let allocation_id = model
            .id
            .or(model.allocation_id)
            .unwrap_or_else(|| format!("eipalloc-{}", &uuid::Uuid::new_v4().to_string()[..17]));
        let public_ip = model.public_ip.unwrap_or_else(|| "0.0.0.0".to_string());
        let tags = extract_tags(attrs);

        let eip_view = ElasticIpView {
            allocation_id: allocation_id.clone(),
            public_ip,
            association_id: optional_str(attrs, "association_id"),
            instance_id: optional_str(attrs, "instance"),
            network_interface_id: optional_str(attrs, "network_interface"),
            private_ip_address: optional_str(attrs, "private_ip"),
            address_attribute_ptr_record: optional_str(attrs, "ptr_record"),
            domain: model.domain.unwrap_or_else(|| "vpc".to_string()),
            tags,
            pending_transfer: None,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.elastic_ips.insert(allocation_id, eip_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for eip in view.elastic_ips.values() {
            let domain = if eip.allocation_id.starts_with("eipalloc-") {
                "vpc"
            } else {
                "standard"
            };
            let attrs = serde_json::json!({
                "id": eip.allocation_id,
                "allocation_id": eip.allocation_id,
                "public_ip": eip.public_ip,
                "domain": domain,
                "association_id": eip.association_id,
                "instance": eip.instance_id,
                "network_interface": eip.network_interface_id,
                "private_ip": eip.private_ip_address,
                "public_dns": format!("ec2-{}.{}.compute.amazonaws.com", eip.public_ip.replace('.', "-"), ctx.default_region),
                "private_dns": format!("ip-{}.{}.compute.internal", eip.private_ip_address.as_deref().unwrap_or("0-0-0-0").replace('.', "-"), ctx.default_region),
                "tags": eip.tags,
                "tags_all": eip.tags,
                "carrier_ip": "",
            });
            results.push(ExtractedResource {
                name: eip.allocation_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_nat_gateway
// ---------------------------------------------------------------------------

pub struct AwsNatGatewayConverter {
    service: Arc<Ec2Service>,
}

impl AwsNatGatewayConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNatGatewayConverter {
    fn resource_type(&self) -> &str {
        "aws_nat_gateway"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_subnet", "aws_eip"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsNatGatewayConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::NatGatewayTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_nat_gateway", e))?;

        let subnet_id = model.subnet_id;
        let nat_gw_id = model
            .id
            .unwrap_or_else(|| format!("nat-{}", &uuid::Uuid::new_v4().to_string()[..17]));
        let connectivity_type = model
            .connectivity_type
            .unwrap_or_else(|| "public".to_string());
        let allocation_id = model.allocation_id;
        let public_ip = model.public_ip;
        let tags = extract_tags(attrs);

        // Resolve the VPC ID from the subnet if possible.
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let vpc_id = snapshot
            .subnets
            .get(&subnet_id)
            .map(|s| s.vpc_id.clone())
            .unwrap_or_default();

        let ng_view = NatGatewayView {
            nat_gateway_id: nat_gw_id.clone(),
            vpc_id,
            subnet_id,
            state: "available".to_string(),
            connectivity_type,
            allocation_id,
            public_ip,
            secondary_addresses: vec![],
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.nat_gateways.insert(nat_gw_id, ng_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for ng in view.nat_gateways.values() {
            let attrs = serde_json::json!({
                "id": ng.nat_gateway_id,
                "nat_gateway_id": ng.nat_gateway_id,
                "vpc_id": ng.vpc_id,
                "subnet_id": ng.subnet_id,
                "state": ng.state,
                "connectivity_type": ng.connectivity_type,
                "allocation_id": ng.allocation_id,
                "public_ip": ng.public_ip,
                "association_id": "",
                "tags": ng.tags,
                "tags_all": ng.tags,
            });
            results.push(ExtractedResource {
                name: ng.nat_gateway_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_placement_group
// ---------------------------------------------------------------------------

pub struct AwsPlacementGroupConverter {
    service: Arc<Ec2Service>,
}

impl AwsPlacementGroupConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsPlacementGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_placement_group"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsPlacementGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::PlacementGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_placement_group", e))?;

        let name = model.name;
        let strategy = model.strategy;
        let partition_count = attrs
            .get("partition_count")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);
        let spread_level = model.spread_level;
        let tags = extract_tags(attrs);

        // Extract emits `id` as the group name and `placement_group_id` as the
        // `pg-...` identifier, so prefer `placement_group_id` when constructing
        // the GroupId. The `id` attribute is treated as a fallback name source
        // and is not used for the GroupId. Synthesise a `pg-...` identifier
        // only when no `placement_group_id` is present in tfstate.
        let group_id = model
            .placement_group_id
            .unwrap_or_else(|| format!("pg-{}", &uuid::Uuid::new_v4().to_string()[..8]));
        let group_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ec2:{}:{}:placement-group/{}",
                region, ctx.default_account_id, name
            )
        });

        let pg_view = PlacementGroupView {
            group_id: group_id.clone(),
            group_name: name,
            group_arn,
            strategy,
            state: "available".to_string(),
            partition_count,
            spread_level,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.placement_groups.insert(group_id, pg_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for pg in view.placement_groups.values() {
            let attrs = serde_json::json!({
                "id": pg.group_name,
                "name": pg.group_name,
                "arn": pg.group_arn,
                "placement_group_id": pg.group_id,
                "strategy": pg.strategy,
                "partition_count": pg.partition_count.unwrap_or(0),
                "spread_level": pg.spread_level.clone().unwrap_or_default(),
                "tags": pg.tags,
                "tags_all": pg.tags,
            });
            results.push(ExtractedResource {
                name: pg.group_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_network_acl
// ---------------------------------------------------------------------------

pub struct AwsNetworkAclConverter {
    service: Arc<Ec2Service>,
}

impl AwsNetworkAclConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkAclConverter {
    fn resource_type(&self) -> &str {
        "aws_network_acl"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc", "aws_subnet"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsNetworkAclConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::NetworkAclTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_network_acl", e))?;

        let vpc_id = model.vpc_id;
        let nacl_id = model
            .id
            .unwrap_or_else(|| format!("acl-{}", &uuid::Uuid::new_v4().to_string()[..17]));
        let tags = extract_tags(attrs);

        let mut entries = parse_nacl_entries(attrs, "ingress", false);
        entries.extend(parse_nacl_entries(attrs, "egress", true));

        let subnet_ids: Vec<String> = attrs
            .get("subnet_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|s| s.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let associations: Vec<NetworkAclAssociationView> = subnet_ids
            .iter()
            .enumerate()
            .map(|(idx, subnet_id)| NetworkAclAssociationView {
                network_acl_association_id: format!(
                    "aclassoc-{}-{}",
                    &nacl_id[std::cmp::min(4, nacl_id.len())..],
                    idx
                ),
                network_acl_id: nacl_id.clone(),
                subnet_id: subnet_id.clone(),
            })
            .collect();

        let nacl_view = NetworkAclView {
            network_acl_id: nacl_id.clone(),
            vpc_id,
            is_default: false,
            entries,
            associations,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.network_acls.insert(nacl_id, nacl_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for nacl in view.network_acls.values() {
            let ingress: Vec<serde_json::Value> = nacl
                .entries
                .iter()
                .filter(|e| !e.egress)
                .map(nacl_entry_to_json)
                .collect();
            let egress: Vec<serde_json::Value> = nacl
                .entries
                .iter()
                .filter(|e| e.egress)
                .map(nacl_entry_to_json)
                .collect();
            let subnet_ids: Vec<&str> = nacl
                .associations
                .iter()
                .map(|a| a.subnet_id.as_str())
                .collect();
            let attrs = serde_json::json!({
                "id": nacl.network_acl_id,
                "vpc_id": nacl.vpc_id,
                "owner_id": ctx.default_account_id,
                "arn": format!(
                    "arn:aws:ec2:{}:{}:network-acl/{}",
                    ctx.default_region, ctx.default_account_id, nacl.network_acl_id
                ),
                "subnet_ids": subnet_ids,
                "ingress": ingress,
                "egress": egress,
                "tags": nacl.tags,
                "tags_all": nacl.tags,
            });
            results.push(ExtractedResource {
                name: nacl.network_acl_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_network_acl_rule
// ---------------------------------------------------------------------------

pub struct AwsNetworkAclRuleConverter {
    service: Arc<Ec2Service>,
}

impl AwsNetworkAclRuleConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkAclRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_network_acl_rule"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_network_acl"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsNetworkAclRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::NetworkAclRuleTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_network_acl_rule", e))?;

        let nacl_id = model.network_acl_id;
        let rule_number = attrs
            .get("rule_number")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32)
            .unwrap_or(0);
        let protocol = model.protocol.unwrap_or_else(|| "-1".to_string());
        let rule_action = model.rule_action.unwrap_or_else(|| "allow".to_string());
        let egress = attrs
            .get("egress")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let cidr_block = optional_str(attrs, "cidr_block");
        let ipv6_cidr_block = optional_str(attrs, "ipv6_cidr_block");
        let from_port = attrs
            .get("from_port")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);
        let to_port = attrs
            .get("to_port")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);
        let icmp_type = attrs
            .get("icmp_type")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);
        let icmp_code = attrs
            .get("icmp_code")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);

        let entry = NetworkAclEntryView {
            rule_number,
            protocol,
            rule_action,
            egress,
            cidr_block,
            ipv6_cidr_block,
            port_range: match (from_port, to_port) {
                (Some(f), Some(t)) => Some(PortRangeView { from: f, to: t }),
                _ => None,
            },
            icmp_type_code: match (icmp_type, icmp_code) {
                (Some(t), Some(c)) => Some(IcmpTypeCodeView {
                    type_num: t,
                    code: c,
                }),
                _ => None,
            },
        };

        // Merge the entry into the existing NACL view.
        let existing = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut nacl_view = existing
            .network_acls
            .get(&nacl_id)
            .cloned()
            .unwrap_or_else(|| NetworkAclView {
                network_acl_id: nacl_id.clone(),
                vpc_id: String::new(),
                is_default: false,
                entries: Vec::new(),
                associations: Vec::new(),
                tags: HashMap::new(),
            });
        nacl_view
            .entries
            .retain(|e| !(e.rule_number == rule_number && e.egress == egress));
        nacl_view.entries.push(entry);

        let mut state_view = minimal_ec2_state_view();
        state_view.network_acls.insert(nacl_id, nacl_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Extraction for individual rules is not separately tracked; rule data
        // round-trips through the `aws_network_acl` resource's `ingress`/`egress`
        // blocks instead.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_egress_only_internet_gateway
// ---------------------------------------------------------------------------

pub struct AwsEgressOnlyInternetGatewayConverter {
    service: Arc<Ec2Service>,
}

impl AwsEgressOnlyInternetGatewayConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEgressOnlyInternetGatewayConverter {
    fn resource_type(&self) -> &str {
        "aws_egress_only_internet_gateway"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEgressOnlyInternetGatewayConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::EgressOnlyInternetGatewayTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_egress_only_internet_gateway", e))?;

        let vpc_id = model.vpc_id;
        let eigw_id = model
            .id
            .unwrap_or_else(|| format!("eigw-{}", &uuid::Uuid::new_v4().to_string()[..17]));
        let mut tags = extract_tags(attrs);
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let eigw_view = EgressOnlyIgwView {
            eigw_id: eigw_id.clone(),
            state: "attached".to_string(),
            attachments: vec![EoigwAttachmentView {
                vpc_id,
                state: "attached".to_string(),
            }],
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.egress_only_igws.insert(eigw_id, eigw_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for eigw in view.egress_only_igws.values() {
            let vpc_id = eigw
                .attachments
                .first()
                .map(|a| a.vpc_id.as_str())
                .unwrap_or("");
            let attrs = serde_json::json!({
                "id": eigw.eigw_id,
                "vpc_id": vpc_id,
                "tags": eigw.tags,
                "tags_all": eigw.tags,
            });
            results.push(ExtractedResource {
                name: eigw.eigw_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_customer_gateway
// ---------------------------------------------------------------------------

pub struct AwsCustomerGatewayConverter {
    service: Arc<Ec2Service>,
}

impl AwsCustomerGatewayConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCustomerGatewayConverter {
    fn resource_type(&self) -> &str {
        "aws_customer_gateway"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsCustomerGatewayConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::CustomerGatewayTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_customer_gateway", e))?;

        // bgp_asn may be encoded as a string or an integer in TF state.
        let bgp_asn = optional_str(attrs, "bgp_asn")
            .or_else(|| {
                attrs
                    .get("bgp_asn")
                    .and_then(|v| v.as_i64())
                    .map(|n| n.to_string())
            })
            .unwrap_or_else(|| "65000".to_string());
        let ip_address = model.ip_address;
        let gateway_type = model.gateway_type.unwrap_or_else(|| "ipsec.1".to_string());
        let cgw_id = model
            .id
            .unwrap_or_else(|| format!("cgw-{}", &uuid::Uuid::new_v4().to_string()[..17]));
        let tags = extract_tags(attrs);

        let cgw_view = CustomerGatewayView {
            customer_gateway_id: cgw_id.clone(),
            bgp_asn,
            ip_address,
            gateway_type,
            state: "available".to_string(),
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.customer_gateways.insert(cgw_id, cgw_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for cgw in view.customer_gateways.values() {
            let attrs = serde_json::json!({
                "id": cgw.customer_gateway_id,
                "bgp_asn": cgw.bgp_asn,
                "ip_address": cgw.ip_address,
                "type": cgw.gateway_type,
                "arn": format!(
                    "arn:aws:ec2:{}:{}:customer-gateway/{}",
                    ctx.default_region, ctx.default_account_id, cgw.customer_gateway_id
                ),
                "tags": cgw.tags,
                "tags_all": cgw.tags,
            });
            results.push(ExtractedResource {
                name: cgw.customer_gateway_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_capacity_reservation
// ---------------------------------------------------------------------------

pub struct AwsEc2CapacityReservationConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2CapacityReservationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2CapacityReservationConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_capacity_reservation"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2CapacityReservationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::CapacityReservationTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_capacity_reservation", e))?;

        let instance_type = model.instance_type;
        let instance_platform = model.instance_platform;
        let availability_zone = model.availability_zone;
        let instance_count = optional_i64(attrs, "instance_count")
            .map(|n| n as i32)
            .ok_or_else(|| ConversionError::MissingAttribute {
                resource_type: "aws_ec2_capacity_reservation".to_string(),
                attribute: "instance_count".to_string(),
            })?;

        let tenancy = model.tenancy.unwrap_or_else(|| "default".to_string());
        let ebs_optimized = model.ebs_optimized;
        let ephemeral_storage = model.ephemeral_storage;
        let end_date = model.end_date;
        let end_date_type = model
            .end_date_type
            .unwrap_or_else(|| "unlimited".to_string());
        let instance_match_criteria = model
            .instance_match_criteria
            .unwrap_or_else(|| "open".to_string());
        let outpost_arn = model.outpost_arn;
        let placement_group_arn = model.placement_group_arn;
        let tags = extract_tags(attrs);

        let cr_id = model.id.unwrap_or_else(|| {
            format!(
                "cr-{}",
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..17]
            )
        });
        let cr_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ec2:{}:{}:capacity-reservation/{}",
                region, ctx.default_account_id, cr_id
            )
        });

        let owner_id = model
            .owner_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let now = chrono::Utc::now().to_rfc3339();
        let start_date = model.start_date.unwrap_or_else(|| now.clone());
        let create_date = model.create_date.unwrap_or(now);

        let cr_view = CapacityReservationView {
            capacity_reservation_id: cr_id.clone(),
            capacity_reservation_arn: cr_arn,
            owner_id,
            instance_type,
            instance_platform,
            availability_zone,
            tenancy,
            total_instance_count: instance_count,
            available_instance_count: instance_count,
            ebs_optimized,
            ephemeral_storage,
            state: "active".to_string(),
            start_date,
            end_date,
            end_date_type,
            instance_match_criteria,
            create_date,
            outpost_arn,
            placement_group_arn,
            tags,
            pending_billing_owner_account_id: None,
            billing_owner_account_id: None,
            target_capacity_reservation_id: None,
            reservation_type: None,
            commitment_info: None,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.capacity_reservations.insert(cr_id, cr_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for cr in view.capacity_reservations.values() {
            let attrs = serde_json::json!({
                "id": cr.capacity_reservation_id,
                "arn": cr.capacity_reservation_arn,
                "capacity_reservation_arn": cr.capacity_reservation_arn,
                "owner_id": cr.owner_id,
                "instance_type": cr.instance_type,
                "instance_platform": cr.instance_platform,
                "availability_zone": cr.availability_zone,
                "tenancy": cr.tenancy,
                "instance_count": cr.total_instance_count,
                "ebs_optimized": cr.ebs_optimized,
                "ephemeral_storage": cr.ephemeral_storage,
                "end_date": cr.end_date.clone().unwrap_or_default(),
                "end_date_type": cr.end_date_type,
                "instance_match_criteria": cr.instance_match_criteria,
                "outpost_arn": cr.outpost_arn.clone().unwrap_or_default(),
                "placement_group_arn": cr.placement_group_arn.clone().unwrap_or_default(),
                "tags": cr.tags,
                "tags_all": cr.tags,
            });
            results.push(ExtractedResource {
                name: cr.capacity_reservation_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_network_interface_permission
// ---------------------------------------------------------------------------

pub struct AwsNetworkInterfacePermissionConverter {
    service: Arc<Ec2Service>,
}

impl AwsNetworkInterfacePermissionConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkInterfacePermissionConverter {
    fn resource_type(&self) -> &str {
        "aws_network_interface_permission"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsNetworkInterfacePermissionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::NetworkInterfacePermissionTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_network_interface_permission", e))?;

        let network_interface_id = model.network_interface_id;
        let aws_account_id = model.aws_account_id;
        let permission = model.permission;

        let permission_id = model.id.unwrap_or_else(|| {
            format!(
                "eni-perm-{}",
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..17]
            )
        });

        let view = NetworkInterfacePermissionView {
            network_interface_permission_id: permission_id.clone(),
            network_interface_id,
            aws_account_id: Some(aws_account_id),
            aws_service: None,
            permission,
            permission_state: "GRANTED".to_string(),
        };

        let mut state_view = minimal_ec2_state_view();
        state_view
            .network_interface_permissions
            .insert(permission_id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for perm in view.network_interface_permissions.values() {
            let attrs = serde_json::json!({
                "id": perm.network_interface_permission_id,
                "network_interface_id": perm.network_interface_id,
                "aws_account_id": perm.aws_account_id.clone().unwrap_or_default(),
                "permission": perm.permission,
            });
            results.push(ExtractedResource {
                name: perm.network_interface_permission_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_instance_connect_endpoint
// ---------------------------------------------------------------------------

pub struct AwsEc2InstanceConnectEndpointConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2InstanceConnectEndpointConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2InstanceConnectEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_instance_connect_endpoint"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2InstanceConnectEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::InstanceConnectEndpointTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_instance_connect_endpoint", e))?;

        let subnet_id = model.subnet_id;
        let preserve_client_ip = model.preserve_client_ip;
        let security_group_ids: Vec<String> = attrs
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let tags = extract_tags(attrs);

        let endpoint_id = model.id.unwrap_or_else(|| {
            format!(
                "eice-{}",
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..17]
            )
        });
        let endpoint_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ec2:{}:{}:instance-connect-endpoint/{}",
                region, ctx.default_account_id, endpoint_id
            )
        });

        // Look up subnet to determine VPC and AZ if state already has the subnet.
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let (vpc_id, availability_zone) = snapshot
            .subnets
            .get(&subnet_id)
            .map(|s| (s.vpc_id.clone(), s.availability_zone.clone()))
            .unwrap_or_else(|| (String::new(), region.clone() + "a"));

        let owner_id = model
            .owner_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = model.created_at.unwrap_or(now);
        let dns_name = model.dns_name.unwrap_or_else(|| {
            format!(
                "{}.{}.ec2-instance-connect-endpoint.aws",
                endpoint_id, region
            )
        });
        let fips_dns_name = model.fips_dns_name.unwrap_or_default();

        let view = InstanceConnectEndpointView {
            instance_connect_endpoint_id: endpoint_id.clone(),
            instance_connect_endpoint_arn: endpoint_arn,
            subnet_id,
            vpc_id,
            availability_zone,
            state: "create-complete".to_string(),
            created_at,
            preserve_client_ip,
            security_group_ids,
            network_interface_ids: vec![],
            dns_name,
            fips_dns_name,
            ip_address_type: "ipv4".to_string(),
            owner_id,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view
            .instance_connect_endpoints
            .insert(endpoint_id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for ep in view.instance_connect_endpoints.values() {
            let attrs = serde_json::json!({
                "id": ep.instance_connect_endpoint_id,
                "arn": ep.instance_connect_endpoint_arn,
                "subnet_id": ep.subnet_id,
                "vpc_id": ep.vpc_id,
                "availability_zone": ep.availability_zone,
                "preserve_client_ip": ep.preserve_client_ip,
                "security_group_ids": ep.security_group_ids,
                "network_interface_ids": ep.network_interface_ids,
                "dns_name": ep.dns_name,
                "fips_dns_name": ep.fips_dns_name,
                "owner_id": ep.owner_id,
                "tags": ep.tags,
                "tags_all": ep.tags,
            });
            results.push(ExtractedResource {
                name: ep.instance_connect_endpoint_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ===========================================================================
// IPAM converters
// ===========================================================================

// ---------------------------------------------------------------------------
// aws_vpc_ipam
// ---------------------------------------------------------------------------

pub struct AwsVpcIpamConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcIpamConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcIpamConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_ipam"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcIpamConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcIpamTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpc_ipam", e))?;

        let description = model.description;
        let tier = model.tier.unwrap_or_else(|| "advanced".to_string());
        let enable_private_gua = model.enable_private_gua;

        let operating_regions: Vec<IpamOperatingRegionView> = attrs
            .get("operating_regions")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|entry| {
                        entry.get("region_name").and_then(|v| v.as_str()).map(|s| {
                            IpamOperatingRegionView {
                                region_name: s.to_string(),
                            }
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let tags = extract_tags(attrs);

        let ipam_id = model.id.unwrap_or_else(|| random_short_id("ipam"));
        let ipam_arn = model
            .arn
            .unwrap_or_else(|| format!("arn:aws:ec2::{}:ipam/{}", ctx.default_account_id, ipam_id));
        let public_default_scope_id = model.public_default_scope_id.unwrap_or_else(|| {
            format!(
                "ipam-scope-{}",
                random_short_id("public").trim_start_matches("public-")
            )
        });
        let private_default_scope_id = model.private_default_scope_id.unwrap_or_else(|| {
            format!(
                "ipam-scope-{}",
                random_short_id("priv").trim_start_matches("priv-")
            )
        });
        let owner_id = model
            .owner_id
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let view = IpamView {
            ipam_id: ipam_id.clone(),
            ipam_arn,
            ipam_region: region.clone(),
            public_default_scope_id,
            private_default_scope_id,
            scope_count: 2,
            description,
            operating_regions,
            state: "create-complete".to_string(),
            owner_id,
            default_resource_discovery_id: model.default_resource_discovery_id,
            default_resource_discovery_association_id: model
                .default_resource_discovery_association_id,
            resource_discovery_association_count: 0,
            tier,
            enable_private_gua,
            metered_account: model
                .metered_account
                .unwrap_or_else(|| "ipam-owner".to_string()),
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.ipams.insert(ipam_id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for ipam in view.ipams.values() {
            let operating_regions: Vec<serde_json::Value> = ipam
                .operating_regions
                .iter()
                .map(|r| serde_json::json!({ "region_name": r.region_name }))
                .collect();
            let attrs = serde_json::json!({
                "id": ipam.ipam_id,
                "arn": ipam.ipam_arn,
                "ipam_region": ipam.ipam_region,
                "public_default_scope_id": ipam.public_default_scope_id,
                "private_default_scope_id": ipam.private_default_scope_id,
                "scope_count": ipam.scope_count,
                "description": ipam.description.clone().unwrap_or_default(),
                "operating_regions": operating_regions,
                "state": ipam.state,
                "owner_id": ipam.owner_id,
                "default_resource_discovery_id": ipam.default_resource_discovery_id.clone().unwrap_or_default(),
                "default_resource_discovery_association_id": ipam.default_resource_discovery_association_id.clone().unwrap_or_default(),
                "resource_discovery_association_count": ipam.resource_discovery_association_count,
                "tier": ipam.tier,
                "enable_private_gua": ipam.enable_private_gua,
                "tags": ipam.tags,
                "tags_all": ipam.tags,
            });
            results.push(ExtractedResource {
                name: ipam.ipam_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_ipam_scope
// ---------------------------------------------------------------------------

pub struct AwsVpcIpamScopeConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcIpamScopeConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcIpamScopeConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_ipam_scope"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_ipam"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcIpamScopeConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcIpamScopeTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpc_ipam_scope", e))?;

        let ipam_id = model.ipam_id;
        let description = model.description;
        let tags = extract_tags(attrs);

        // Look up the IPAM to derive the ipam_arn / region.
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let ipam_arn = snapshot
            .ipams
            .get(&ipam_id)
            .map(|i| i.ipam_arn.clone())
            .unwrap_or_else(|| format!("arn:aws:ec2::{}:ipam/{}", ctx.default_account_id, ipam_id));

        let scope_id = model.id.unwrap_or_else(|| random_short_id("ipam-scope"));
        let scope_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ec2::{}:ipam-scope/{}",
                ctx.default_account_id, scope_id
            )
        });
        let owner_id = model
            .owner_id
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let view = IpamScopeView {
            ipam_scope_id: scope_id.clone(),
            ipam_scope_arn: scope_arn,
            ipam_arn,
            ipam_region: region.clone(),
            ipam_scope_type: model
                .ipam_scope_type
                .unwrap_or_else(|| "private".to_string()),
            is_default: model.is_default,
            description,
            pool_count: 0,
            state: "create-complete".to_string(),
            tags,
            owner_id,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.ipam_scopes.insert(scope_id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for scope in view.ipam_scopes.values() {
            let attrs = serde_json::json!({
                "id": scope.ipam_scope_id,
                "arn": scope.ipam_scope_arn,
                "ipam_arn": scope.ipam_arn,
                "ipam_scope_type": scope.ipam_scope_type,
                "is_default": scope.is_default,
                "description": scope.description.clone().unwrap_or_default(),
                "pool_count": scope.pool_count,
                "owner_id": scope.owner_id,
                "tags": scope.tags,
                "tags_all": scope.tags,
            });
            results.push(ExtractedResource {
                name: scope.ipam_scope_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_ipam_pool
// ---------------------------------------------------------------------------

pub struct AwsVpcIpamPoolConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcIpamPoolConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcIpamPoolConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_ipam_pool"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_ipam_scope"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcIpamPoolConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcIpamPoolTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpc_ipam_pool", e))?;

        let address_family = model.address_family;
        let ipam_scope_id = model.ipam_scope_id;
        let description = model.description;
        let auto_import = model.auto_import;
        let publicly_advertisable = model.publicly_advertisable;
        let aws_service = model.aws_service;
        let public_ip_source = model.public_ip_source;
        let locale = model.locale.unwrap_or_else(|| "None".to_string());
        let allocation_min_netmask_length =
            optional_i64(attrs, "allocation_min_netmask_length").map(|n| n as i32);
        let allocation_max_netmask_length =
            optional_i64(attrs, "allocation_max_netmask_length").map(|n| n as i32);
        let allocation_default_netmask_length =
            optional_i64(attrs, "allocation_default_netmask_length").map(|n| n as i32);
        let allocation_resource_tags: Vec<(String, String)> = attrs
            .get("allocation_resource_tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();
        let tags = extract_tags(attrs);

        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let (ipam_scope_arn, ipam_scope_type, ipam_arn) = snapshot
            .ipam_scopes
            .get(&ipam_scope_id)
            .map(|s| {
                (
                    s.ipam_scope_arn.clone(),
                    s.ipam_scope_type.clone(),
                    s.ipam_arn.clone(),
                )
            })
            .unwrap_or_else(|| {
                (
                    format!(
                        "arn:aws:ec2::{}:ipam-scope/{}",
                        ctx.default_account_id, ipam_scope_id
                    ),
                    "private".to_string(),
                    String::new(),
                )
            });

        let pool_id = model.id.unwrap_or_else(|| random_short_id("ipam-pool"));
        let pool_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ec2::{}:ipam-pool/{}",
                ctx.default_account_id, pool_id
            )
        });
        let owner_id = model
            .owner_id
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let view = IpamPoolView {
            ipam_pool_id: pool_id.clone(),
            source_ipam_pool_id: model.source_ipam_pool_id,
            ipam_pool_arn: pool_arn,
            ipam_scope_arn,
            ipam_scope_type,
            ipam_arn,
            ipam_region: region.clone(),
            locale,
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
            source_resource_id: model.source_resource_id,
            source_resource_type: model.source_resource_type,
            source_resource_region: model.source_resource_region,
            source_resource_owner: model.source_resource_owner,
            tags,
            owner_id,
            allocation_count: 0,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.ipam_pools.insert(pool_id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for pool in view.ipam_pools.values() {
            let alloc_tags: serde_json::Map<String, serde_json::Value> = pool
                .allocation_resource_tags
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            let attrs = serde_json::json!({
                "id": pool.ipam_pool_id,
                "arn": pool.ipam_pool_arn,
                "ipam_scope_id": pool.ipam_scope_arn.split('/').next_back().unwrap_or(""),
                "ipam_scope_arn": pool.ipam_scope_arn,
                "ipam_scope_type": pool.ipam_scope_type,
                "address_family": pool.address_family,
                "auto_import": pool.auto_import,
                "publicly_advertisable": pool.publicly_advertisable,
                "aws_service": pool.aws_service.clone().unwrap_or_default(),
                "public_ip_source": pool.public_ip_source.clone().unwrap_or_default(),
                "description": pool.description.clone().unwrap_or_default(),
                "locale": pool.locale,
                "pool_depth": pool.pool_depth,
                "state": pool.state,
                "source_ipam_pool_id": pool.source_ipam_pool_id.clone().unwrap_or_default(),
                "allocation_min_netmask_length": pool.allocation_min_netmask_length.unwrap_or(0),
                "allocation_max_netmask_length": pool.allocation_max_netmask_length.unwrap_or(0),
                "allocation_default_netmask_length": pool.allocation_default_netmask_length.unwrap_or(0),
                "allocation_resource_tags": alloc_tags,
                "owner_id": pool.owner_id,
                "tags": pool.tags,
                "tags_all": pool.tags,
            });
            results.push(ExtractedResource {
                name: pool.ipam_pool_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_ipam_pool_cidr
// ---------------------------------------------------------------------------

pub struct AwsVpcIpamPoolCidrConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcIpamPoolCidrConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcIpamPoolCidrConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_ipam_pool_cidr"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_ipam_pool"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcIpamPoolCidrConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcIpamPoolCidrTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpc_ipam_pool_cidr", e))?;

        let ipam_pool_id = model.ipam_pool_id;
        let cidr = model.cidr.unwrap_or_default();
        let netmask_length = optional_i64(attrs, "netmask_length").map(|n| n as i32);

        let cidr_id = model
            .id
            .or(model.ipam_pool_cidr_id)
            .unwrap_or_else(|| random_short_id("ipam-pool-cidr"));

        let view = IpamPoolCidrView {
            ipam_pool_id,
            cidr,
            state: "provisioned".to_string(),
            failure_reason: None,
            ipam_pool_cidr_id: cidr_id,
            netmask_length,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.ipam_pool_cidrs.push(view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for c in view.ipam_pool_cidrs.iter() {
            let attrs = serde_json::json!({
                "id": format!("{}_{}", c.ipam_pool_cidr_id, c.ipam_pool_id),
                "ipam_pool_id": c.ipam_pool_id,
                "ipam_pool_cidr_id": c.ipam_pool_cidr_id,
                "cidr": c.cidr,
                "state": c.state,
                "netmask_length": c.netmask_length.unwrap_or(0),
            });
            results.push(ExtractedResource {
                name: c.ipam_pool_cidr_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_ipam_resource_discovery
// ---------------------------------------------------------------------------

pub struct AwsVpcIpamResourceDiscoveryConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcIpamResourceDiscoveryConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcIpamResourceDiscoveryConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_ipam_resource_discovery"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcIpamResourceDiscoveryConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcIpamResourceDiscoveryTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpc_ipam_resource_discovery", e))?;

        let description = model.description;
        let tags = extract_tags(attrs);

        let operating_regions: Vec<IpamOperatingRegionView> = attrs
            .get("operating_regions")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|entry| {
                        entry.get("region_name").and_then(|v| v.as_str()).map(|s| {
                            IpamOperatingRegionView {
                                region_name: s.to_string(),
                            }
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let id = model
            .id
            .unwrap_or_else(|| random_short_id("ipam-res-disco"));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ec2::{}:ipam-resource-discovery/{}",
                ctx.default_account_id, id
            )
        });
        let owner_id = model
            .owner_id
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let view = IpamResourceDiscoveryView {
            ipam_resource_discovery_id: id.clone(),
            ipam_resource_discovery_arn: arn,
            ipam_resource_discovery_region: region.clone(),
            description,
            operating_regions,
            is_default: model.is_default,
            state: "create-complete".to_string(),
            owner_id,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.ipam_resource_discoveries.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for r in view.ipam_resource_discoveries.values() {
            let operating_regions: Vec<serde_json::Value> = r
                .operating_regions
                .iter()
                .map(|o| serde_json::json!({ "region_name": o.region_name }))
                .collect();
            let attrs = serde_json::json!({
                "id": r.ipam_resource_discovery_id,
                "arn": r.ipam_resource_discovery_arn,
                "ipam_resource_discovery_region": r.ipam_resource_discovery_region,
                "description": r.description.clone().unwrap_or_default(),
                "operating_regions": operating_regions,
                "is_default": r.is_default,
                "owner_id": r.owner_id,
                "tags": r.tags,
                "tags_all": r.tags,
            });
            results.push(ExtractedResource {
                name: r.ipam_resource_discovery_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ===========================================================================
// Verified Access converters
// ===========================================================================

// ---------------------------------------------------------------------------
// aws_verifiedaccess_instance
// ---------------------------------------------------------------------------

pub struct AwsVerifiedaccessInstanceConverter {
    service: Arc<Ec2Service>,
}

impl AwsVerifiedaccessInstanceConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVerifiedaccessInstanceConverter {
    fn resource_type(&self) -> &str {
        "aws_verifiedaccess_instance"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVerifiedaccessInstanceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VerifiedAccessInstanceTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_verifiedaccess_instance", e))?;

        let description = model.description;
        let fips_enabled = model.fips_enabled;
        let cidr_endpoints_custom_subdomain = model.cidr_endpoints_custom_subdomain;
        let name = model.name;
        let trust_provider_ids = parse_string_array(attrs, "verified_access_trust_provider_ids");
        let tags = extract_tags(attrs);

        let id = model.id.unwrap_or_else(|| random_short_id("vai"));
        let now = chrono::Utc::now().to_rfc3339();
        let creation_time = model.creation_time.unwrap_or_else(|| now.clone());
        let last_updated_time = model.last_updated_time.unwrap_or(now);

        let view = VerifiedAccessInstanceView {
            verified_access_instance_id: id.clone(),
            description,
            creation_time,
            last_updated_time,
            fips_enabled,
            cidr_endpoints_custom_subdomain,
            name,
            trust_provider_ids,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.verified_access_instances.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for inst in view.verified_access_instances.values() {
            let attrs = serde_json::json!({
                "id": inst.verified_access_instance_id,
                "description": inst.description.clone().unwrap_or_default(),
                "creation_time": inst.creation_time,
                "last_updated_time": inst.last_updated_time,
                "fips_enabled": inst.fips_enabled,
                "cidr_endpoints_custom_subdomain": inst.cidr_endpoints_custom_subdomain.clone().unwrap_or_default(),
                "name": inst.name.clone().unwrap_or_default(),
                "verified_access_trust_provider_ids": inst.trust_provider_ids,
                "tags": inst.tags,
                "tags_all": inst.tags,
            });
            results.push(ExtractedResource {
                name: inst.verified_access_instance_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_verifiedaccess_trust_provider
// ---------------------------------------------------------------------------

pub struct AwsVerifiedaccessTrustProviderConverter {
    service: Arc<Ec2Service>,
}

impl AwsVerifiedaccessTrustProviderConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVerifiedaccessTrustProviderConverter {
    fn resource_type(&self) -> &str {
        "aws_verifiedaccess_trust_provider"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVerifiedaccessTrustProviderConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VerifiedAccessTrustProviderTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_verifiedaccess_trust_provider", e))?;

        let trust_provider_type = model.trust_provider_type;
        let policy_reference_name = model.policy_reference_name;
        let description = model.description;
        let user_trust_provider_type = model.user_trust_provider_type;
        let device_trust_provider_type = model.device_trust_provider_type;
        let tags = extract_tags(attrs);

        let id = model.id.unwrap_or_else(|| random_short_id("vatp"));
        let now = chrono::Utc::now().to_rfc3339();
        let creation_time = model.creation_time.unwrap_or_else(|| now.clone());
        let last_updated_time = model.last_updated_time.unwrap_or(now);

        let view = VerifiedAccessTrustProviderView {
            verified_access_trust_provider_id: id.clone(),
            description,
            trust_provider_type,
            user_trust_provider_type,
            device_trust_provider_type,
            oidc_options: None,
            device_options: None,
            native_application_oidc_options: None,
            policy_reference_name,
            creation_time,
            last_updated_time,
            sse_specification: VerifiedAccessSseSpecificationView::default(),
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.verified_access_trust_providers.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for tp in view.verified_access_trust_providers.values() {
            let attrs = serde_json::json!({
                "id": tp.verified_access_trust_provider_id,
                "trust_provider_type": tp.trust_provider_type,
                "user_trust_provider_type": tp.user_trust_provider_type.clone().unwrap_or_default(),
                "device_trust_provider_type": tp.device_trust_provider_type.clone().unwrap_or_default(),
                "policy_reference_name": tp.policy_reference_name,
                "description": tp.description.clone().unwrap_or_default(),
                "creation_time": tp.creation_time,
                "last_updated_time": tp.last_updated_time,
                "tags": tp.tags,
                "tags_all": tp.tags,
            });
            results.push(ExtractedResource {
                name: tp.verified_access_trust_provider_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_verifiedaccess_group
// ---------------------------------------------------------------------------

pub struct AwsVerifiedaccessGroupConverter {
    service: Arc<Ec2Service>,
}

impl AwsVerifiedaccessGroupConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVerifiedaccessGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_verifiedaccess_group"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_verifiedaccess_instance"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVerifiedaccessGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VerifiedAccessGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_verifiedaccess_group", e))?;

        // verifiedaccess_instance_id has both a "verifiedaccess_instance_id"
        // and "verified_access_instance_id" spelling depending on TF state.
        let verified_access_instance_id = optional_str(attrs, "verifiedaccess_instance_id")
            .or_else(|| optional_str(attrs, "verified_access_instance_id"))
            .ok_or_else(|| ConversionError::MissingAttribute {
                resource_type: "aws_verifiedaccess_group".to_string(),
                attribute: "verifiedaccess_instance_id".to_string(),
            })?;
        let description = model.description;
        let policy_document = model.policy_document;
        let policy_enabled = policy_document.is_some();
        let tags = extract_tags(attrs);

        let id = model.id.unwrap_or_else(|| random_short_id("vagr"));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ec2:{}:{}:verified-access-group/{}",
                region, ctx.default_account_id, id
            )
        });
        let now = chrono::Utc::now().to_rfc3339();
        let creation_time = model.creation_time.unwrap_or_else(|| now.clone());
        let last_updated_time = model.last_updated_time.unwrap_or(now);
        let owner = model
            .owner
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let view = VerifiedAccessGroupView {
            verified_access_group_id: id.clone(),
            verified_access_group_arn: arn,
            verified_access_instance_id,
            owner,
            description,
            creation_time,
            last_updated_time,
            deletion_time: None,
            sse_specification: VerifiedAccessSseSpecificationView::default(),
            policy_document,
            policy_enabled,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.verified_access_groups.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for g in view.verified_access_groups.values() {
            let attrs = serde_json::json!({
                "id": g.verified_access_group_id,
                "verifiedaccess_group_arn": g.verified_access_group_arn,
                "verifiedaccess_instance_id": g.verified_access_instance_id,
                "owner": g.owner,
                "description": g.description.clone().unwrap_or_default(),
                "creation_time": g.creation_time,
                "last_updated_time": g.last_updated_time,
                "policy_document": g.policy_document.clone().unwrap_or_default(),
                "tags": g.tags,
                "tags_all": g.tags,
            });
            results.push(ExtractedResource {
                name: g.verified_access_group_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_verifiedaccess_endpoint
// ---------------------------------------------------------------------------

pub struct AwsVerifiedaccessEndpointConverter {
    service: Arc<Ec2Service>,
}

impl AwsVerifiedaccessEndpointConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVerifiedaccessEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_verifiedaccess_endpoint"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_verifiedaccess_group"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVerifiedaccessEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VerifiedAccessEndpointTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_verifiedaccess_endpoint", e))?;

        let endpoint_type = model.endpoint_type;
        let attachment_type = model.attachment_type;
        // group_id has both "verifiedaccess_group_id" and
        // "verified_access_group_id" spellings depending on TF state.
        let group_id = optional_str(attrs, "verifiedaccess_group_id")
            .or_else(|| optional_str(attrs, "verified_access_group_id"))
            .ok_or_else(|| ConversionError::MissingAttribute {
                resource_type: "aws_verifiedaccess_endpoint".to_string(),
                attribute: "verifiedaccess_group_id".to_string(),
            })?;
        let application_domain = model.application_domain;
        let domain_certificate_arn = model.domain_certificate_arn;
        let endpoint_domain_prefix = model.endpoint_domain_prefix;
        let endpoint_domain = model.endpoint_domain.or(endpoint_domain_prefix);
        let device_validation_domain = model.device_validation_domain;
        let security_group_ids = parse_string_array(attrs, "security_group_ids");
        let description = model.description;
        let policy_document = model.policy_document;
        let policy_enabled = policy_document.is_some();
        let tags = extract_tags(attrs);

        // Look up group to derive instance id.
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let verified_access_instance_id = snapshot
            .verified_access_groups
            .get(&group_id)
            .map(|g| g.verified_access_instance_id.clone())
            .unwrap_or_default();

        let id = model.id.unwrap_or_else(|| random_short_id("vae"));
        let now = chrono::Utc::now().to_rfc3339();
        let creation_time = model.creation_time.unwrap_or_else(|| now.clone());
        let last_updated_time = model.last_updated_time.unwrap_or(now);

        let view = VerifiedAccessEndpointView {
            verified_access_endpoint_id: id.clone(),
            verified_access_instance_id,
            verified_access_group_id: group_id,
            application_domain,
            endpoint_type,
            attachment_type,
            domain_certificate_arn,
            endpoint_domain,
            device_validation_domain,
            security_group_ids,
            load_balancer_options: None,
            network_interface_options: None,
            cidr_options: None,
            rds_options: None,
            status_code: "active".to_string(),
            status_message: None,
            description,
            creation_time,
            last_updated_time,
            deletion_time: None,
            sse_specification: VerifiedAccessSseSpecificationView::default(),
            policy_document,
            policy_enabled,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.verified_access_endpoints.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for ep in view.verified_access_endpoints.values() {
            let attrs = serde_json::json!({
                "id": ep.verified_access_endpoint_id,
                "verifiedaccess_instance_id": ep.verified_access_instance_id,
                "verifiedaccess_group_id": ep.verified_access_group_id,
                "application_domain": ep.application_domain.clone().unwrap_or_default(),
                "endpoint_type": ep.endpoint_type,
                "attachment_type": ep.attachment_type,
                "domain_certificate_arn": ep.domain_certificate_arn.clone().unwrap_or_default(),
                "endpoint_domain": ep.endpoint_domain.clone().unwrap_or_default(),
                "device_validation_domain": ep.device_validation_domain.clone().unwrap_or_default(),
                "security_group_ids": ep.security_group_ids,
                "status_code": ep.status_code,
                "description": ep.description.clone().unwrap_or_default(),
                "creation_time": ep.creation_time,
                "last_updated_time": ep.last_updated_time,
                "policy_document": ep.policy_document.clone().unwrap_or_default(),
                "tags": ep.tags,
                "tags_all": ep.tags,
            });
            results.push(ExtractedResource {
                name: ep.verified_access_endpoint_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ===========================================================================
// Traffic Mirror converters
// ===========================================================================

// ---------------------------------------------------------------------------
// aws_ec2_traffic_mirror_target
// ---------------------------------------------------------------------------

pub struct AwsEc2TrafficMirrorTargetConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TrafficMirrorTargetConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TrafficMirrorTargetConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_traffic_mirror_target"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TrafficMirrorTargetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TrafficMirrorTargetTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_traffic_mirror_target", e))?;

        let network_interface_id = model.network_interface_id;
        let network_load_balancer_arn = model.network_load_balancer_arn;
        let gateway_load_balancer_endpoint_id = model.gateway_load_balancer_endpoint_id;
        let description = model.description;
        let tags = extract_tags(attrs);

        let target_type = if network_interface_id.is_some() {
            "network-interface".to_string()
        } else if network_load_balancer_arn.is_some() {
            "network-load-balancer".to_string()
        } else {
            "gateway-load-balancer-endpoint".to_string()
        };

        let id = model.id.unwrap_or_else(|| random_short_id("tmt"));
        let owner_id = model
            .owner_id
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let view = TrafficMirrorTargetView {
            traffic_mirror_target_id: id.clone(),
            network_interface_id,
            network_load_balancer_arn,
            gateway_load_balancer_endpoint_id,
            r#type: target_type,
            description,
            owner_id,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.traffic_mirror_targets.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for t in view.traffic_mirror_targets.values() {
            let attrs = serde_json::json!({
                "id": t.traffic_mirror_target_id,
                "network_interface_id": t.network_interface_id.clone().unwrap_or_default(),
                "network_load_balancer_arn": t.network_load_balancer_arn.clone().unwrap_or_default(),
                "gateway_load_balancer_endpoint_id": t.gateway_load_balancer_endpoint_id.clone().unwrap_or_default(),
                "type": t.r#type,
                "description": t.description.clone().unwrap_or_default(),
                "owner_id": t.owner_id,
                "tags": t.tags,
                "tags_all": t.tags,
            });
            results.push(ExtractedResource {
                name: t.traffic_mirror_target_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_traffic_mirror_filter
// ---------------------------------------------------------------------------

pub struct AwsEc2TrafficMirrorFilterConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TrafficMirrorFilterConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TrafficMirrorFilterConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_traffic_mirror_filter"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TrafficMirrorFilterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TrafficMirrorFilterTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_traffic_mirror_filter", e))?;

        let description = model.description;
        let network_services = parse_string_array(attrs, "network_services");
        let tags = extract_tags(attrs);

        let id = model.id.unwrap_or_else(|| random_short_id("tmf"));

        let view = TrafficMirrorFilterView {
            traffic_mirror_filter_id: id.clone(),
            description,
            ingress_filter_rules: vec![],
            egress_filter_rules: vec![],
            network_services,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.traffic_mirror_filters.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for f in view.traffic_mirror_filters.values() {
            let attrs = serde_json::json!({
                "id": f.traffic_mirror_filter_id,
                "description": f.description.clone().unwrap_or_default(),
                "network_services": f.network_services,
                "tags": f.tags,
                "tags_all": f.tags,
            });
            results.push(ExtractedResource {
                name: f.traffic_mirror_filter_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_traffic_mirror_filter_rule
// ---------------------------------------------------------------------------

pub struct AwsEc2TrafficMirrorFilterRuleConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TrafficMirrorFilterRuleConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TrafficMirrorFilterRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_traffic_mirror_filter_rule"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_traffic_mirror_filter"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

fn parse_tm_port_range(attrs: &serde_json::Value, key: &str) -> Option<TrafficMirrorPortRangeView> {
    attrs
        .get(key)
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .map(|block| TrafficMirrorPortRangeView {
            from_port: block
                .get("from_port")
                .and_then(|v| v.as_i64())
                .map(|n| n as i32),
            to_port: block
                .get("to_port")
                .and_then(|v| v.as_i64())
                .map(|n| n as i32),
        })
}

impl AwsEc2TrafficMirrorFilterRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TrafficMirrorFilterRuleTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_traffic_mirror_filter_rule", e))?;

        let traffic_mirror_filter_id = model.traffic_mirror_filter_id;
        let traffic_direction = model.traffic_direction;
        let rule_action = model.rule_action;
        let rule_number = optional_i64(attrs, "rule_number")
            .ok_or_else(|| ConversionError::MissingAttribute {
                resource_type: "aws_ec2_traffic_mirror_filter_rule".to_string(),
                attribute: "rule_number".to_string(),
            })?
            .try_into()
            .unwrap_or(0_i32);
        let destination_cidr_block = model.destination_cidr_block;
        let source_cidr_block = model.source_cidr_block;
        let protocol = optional_i64(attrs, "protocol").map(|n| n as i32);
        let description = model.description;
        let destination_port_range = parse_tm_port_range(attrs, "destination_port_range");
        let source_port_range = parse_tm_port_range(attrs, "source_port_range");
        let tags = extract_tags(attrs);

        let id = model.id.unwrap_or_else(|| random_short_id("tmfr"));

        let rule = TrafficMirrorFilterRuleView {
            traffic_mirror_filter_rule_id: id.clone(),
            traffic_mirror_filter_id: traffic_mirror_filter_id.clone(),
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

        // Read-modify-write the parent filter so we add this rule to its rule list.
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if let Some(filter) = snapshot
            .traffic_mirror_filters
            .get(&traffic_mirror_filter_id)
        {
            let mut filter = filter.clone();
            if traffic_direction == "ingress" {
                filter.ingress_filter_rules.push(rule);
            } else {
                filter.egress_filter_rules.push(rule);
            }
            let mut state_view = minimal_ec2_state_view();
            state_view
                .traffic_mirror_filters
                .insert(traffic_mirror_filter_id, filter);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;
        }

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for filter in view.traffic_mirror_filters.values() {
            for rule in filter
                .ingress_filter_rules
                .iter()
                .chain(filter.egress_filter_rules.iter())
            {
                let dest_pr = rule.destination_port_range.as_ref().map(|pr| {
                    serde_json::json!([{"from_port": pr.from_port.unwrap_or(0), "to_port": pr.to_port.unwrap_or(0)}])
                });
                let src_pr = rule.source_port_range.as_ref().map(|pr| {
                    serde_json::json!([{"from_port": pr.from_port.unwrap_or(0), "to_port": pr.to_port.unwrap_or(0)}])
                });
                let attrs = serde_json::json!({
                    "id": rule.traffic_mirror_filter_rule_id,
                    "traffic_mirror_filter_id": rule.traffic_mirror_filter_id,
                    "traffic_direction": rule.traffic_direction,
                    "rule_number": rule.rule_number,
                    "rule_action": rule.rule_action,
                    "protocol": rule.protocol.unwrap_or(0),
                    "destination_cidr_block": rule.destination_cidr_block,
                    "source_cidr_block": rule.source_cidr_block,
                    "description": rule.description.clone().unwrap_or_default(),
                    "destination_port_range": dest_pr,
                    "source_port_range": src_pr,
                });
                results.push(ExtractedResource {
                    name: rule.traffic_mirror_filter_rule_id.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_traffic_mirror_session
// ---------------------------------------------------------------------------

pub struct AwsEc2TrafficMirrorSessionConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TrafficMirrorSessionConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TrafficMirrorSessionConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_traffic_mirror_session"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_ec2_traffic_mirror_target",
            "aws_ec2_traffic_mirror_filter",
        ]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TrafficMirrorSessionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TrafficMirrorSessionTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_traffic_mirror_session", e))?;

        let network_interface_id = model.network_interface_id;
        let traffic_mirror_target_id = model.traffic_mirror_target_id;
        let traffic_mirror_filter_id = model.traffic_mirror_filter_id;
        let session_number = optional_i64(attrs, "session_number").ok_or_else(|| {
            ConversionError::MissingAttribute {
                resource_type: "aws_ec2_traffic_mirror_session".to_string(),
                attribute: "session_number".to_string(),
            }
        })? as i32;
        let packet_length = optional_i64(attrs, "packet_length").map(|n| n as i32);
        let virtual_network_id = optional_i64(attrs, "virtual_network_id").map(|n| n as i32);
        let description = model.description;
        let tags = extract_tags(attrs);

        let id = model.id.unwrap_or_else(|| random_short_id("tms"));
        let owner_id = model
            .owner_id
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let view = TrafficMirrorSessionView {
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

        let mut state_view = minimal_ec2_state_view();
        state_view.traffic_mirror_sessions.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for s in view.traffic_mirror_sessions.values() {
            let attrs = serde_json::json!({
                "id": s.traffic_mirror_session_id,
                "network_interface_id": s.network_interface_id,
                "traffic_mirror_target_id": s.traffic_mirror_target_id,
                "traffic_mirror_filter_id": s.traffic_mirror_filter_id,
                "session_number": s.session_number,
                "packet_length": s.packet_length.unwrap_or(0),
                "virtual_network_id": s.virtual_network_id.unwrap_or(0),
                "description": s.description.clone().unwrap_or_default(),
                "owner_id": s.owner_id,
                "tags": s.tags,
                "tags_all": s.tags,
            });
            results.push(ExtractedResource {
                name: s.traffic_mirror_session_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ===========================================================================
// Transit Gateway extension converters
// ===========================================================================

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_multicast_domain
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayMulticastDomainConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayMulticastDomainConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayMulticastDomainConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_multicast_domain"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TransitGatewayMulticastDomainConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayMulticastDomainTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_transit_gateway_multicast_domain", e)
            })?;

        let transit_gateway_id = model.transit_gateway_id;
        let igmpv2_support = model
            .igmpv2_support
            .unwrap_or_else(|| "disable".to_string());
        let static_sources_support = model
            .static_sources_support
            .unwrap_or_else(|| "disable".to_string());
        let auto_accept_shared_associations = model
            .auto_accept_shared_associations
            .unwrap_or_else(|| "disable".to_string());
        let tags = extract_tags(attrs);

        let id = model
            .id
            .unwrap_or_else(|| random_short_id("tgw-mcast-domain"));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ec2:{}:{}:transit-gateway-multicast-domain/{}",
                region, ctx.default_account_id, id
            )
        });
        let owner_id = model
            .owner_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

        let view = TransitGatewayMulticastDomainView {
            transit_gateway_multicast_domain_id: id.clone(),
            transit_gateway_id,
            transit_gateway_multicast_domain_arn: arn,
            owner_id,
            igmpv2_support,
            static_sources_support,
            auto_accept_shared_associations,
            state: "available".to_string(),
            creation_time,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.tgw_multicast_domains.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for d in view.tgw_multicast_domains.values() {
            let attrs = serde_json::json!({
                "id": d.transit_gateway_multicast_domain_id,
                "arn": d.transit_gateway_multicast_domain_arn,
                "transit_gateway_id": d.transit_gateway_id,
                "owner_id": d.owner_id,
                "igmpv2_support": d.igmpv2_support,
                "static_sources_support": d.static_sources_support,
                "auto_accept_shared_associations": d.auto_accept_shared_associations,
                "state": d.state,
                "creation_time": d.creation_time,
                "tags": d.tags,
                "tags_all": d.tags,
            });
            results.push(ExtractedResource {
                name: d.transit_gateway_multicast_domain_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_connect
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayConnectConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayConnectConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayConnectConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_connect"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TransitGatewayConnectConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayConnectTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_transit_gateway_connect", e))?;

        let transport_attachment_id = model.transport_attachment_id;
        let protocol = model.protocol.unwrap_or_else(|| "gre".to_string());
        let tags = extract_tags(attrs);

        // Look up the transport attachment to find the TGW id.
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let transit_gateway_id = snapshot
            .tgw_vpc_attachments
            .get(&transport_attachment_id)
            .map(|a| a.transit_gateway_id.clone())
            .unwrap_or_else(|| model.transit_gateway_id.unwrap_or_default());

        let id = model.id.unwrap_or_else(|| random_short_id("tgw-attach"));
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

        let view = TransitGatewayConnectView {
            transit_gateway_attachment_id: id.clone(),
            transport_transit_gateway_attachment_id: transport_attachment_id,
            transit_gateway_id,
            state: "available".to_string(),
            creation_time,
            protocol,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.tgw_connects.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for c in view.tgw_connects.values() {
            let attrs = serde_json::json!({
                "id": c.transit_gateway_attachment_id,
                "transit_gateway_id": c.transit_gateway_id,
                "transport_attachment_id": c.transport_transit_gateway_attachment_id,
                "protocol": c.protocol,
                "state": c.state,
                "creation_time": c.creation_time,
                "tags": c.tags,
                "tags_all": c.tags,
            });
            results.push(ExtractedResource {
                name: c.transit_gateway_attachment_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_policy_table
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayPolicyTableConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayPolicyTableConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayPolicyTableConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_policy_table"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TransitGatewayPolicyTableConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayPolicyTableTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_transit_gateway_policy_table", e)
            })?;

        let transit_gateway_id = model.transit_gateway_id;
        let tags = extract_tags(attrs);

        let id = model.id.unwrap_or_else(|| random_short_id("tgw-ptb"));
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

        let view = TransitGatewayPolicyTableView {
            transit_gateway_policy_table_id: id.clone(),
            transit_gateway_id,
            state: "available".to_string(),
            creation_time,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.tgw_policy_tables.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for t in view.tgw_policy_tables.values() {
            let attrs = serde_json::json!({
                "id": t.transit_gateway_policy_table_id,
                "transit_gateway_id": t.transit_gateway_id,
                "state": t.state,
                "creation_time": t.creation_time,
                "tags": t.tags,
                "tags_all": t.tags,
            });
            results.push(ExtractedResource {
                name: t.transit_gateway_policy_table_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ===========================================================================
// Network Insights converters
// ===========================================================================

// ---------------------------------------------------------------------------
// aws_ec2_network_insights_path
// ---------------------------------------------------------------------------

pub struct AwsEc2NetworkInsightsPathConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2NetworkInsightsPathConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2NetworkInsightsPathConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_network_insights_path"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2NetworkInsightsPathConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::NetworkInsightsPathTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_network_insights_path", e))?;

        let protocol = model.protocol;
        let source = model.source;
        let destination = model.destination;
        let source_ip = model.source_ip;
        let destination_ip = model.destination_ip;
        let destination_port = optional_i64(attrs, "destination_port").map(|n| n as i32);
        let tags = extract_tags(attrs);

        let id = model.id.unwrap_or_else(|| random_short_id("nip"));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ec2:{}:{}:network-insights-path/{}",
                region, ctx.default_account_id, id
            )
        });
        let created_date = model
            .created_date
            .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

        let view = NetworkInsightsPathView {
            network_insights_path_id: id.clone(),
            network_insights_path_arn: arn,
            created_date,
            source: Some(source),
            destination,
            source_arn: model.source_arn,
            destination_arn: model.destination_arn,
            source_ip,
            destination_ip,
            protocol,
            destination_port,
            tags,
            filter_at_source: Default::default(),
            filter_at_destination: Default::default(),
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.network_insights_paths.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for p in view.network_insights_paths.values() {
            let attrs = serde_json::json!({
                "id": p.network_insights_path_id,
                "arn": p.network_insights_path_arn,
                "protocol": p.protocol,
                "source": p.source.clone().unwrap_or_default(),
                "destination": p.destination.clone().unwrap_or_default(),
                "source_arn": p.source_arn.clone().unwrap_or_default(),
                "destination_arn": p.destination_arn.clone().unwrap_or_default(),
                "source_ip": p.source_ip.clone().unwrap_or_default(),
                "destination_ip": p.destination_ip.clone().unwrap_or_default(),
                "destination_port": p.destination_port.unwrap_or(0),
                "created_date": p.created_date,
                "tags": p.tags,
                "tags_all": p.tags,
            });
            results.push(ExtractedResource {
                name: p.network_insights_path_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_network_insights_analysis
// ---------------------------------------------------------------------------

pub struct AwsEc2NetworkInsightsAnalysisConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2NetworkInsightsAnalysisConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2NetworkInsightsAnalysisConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_network_insights_analysis"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_network_insights_path"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2NetworkInsightsAnalysisConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::NetworkInsightsAnalysisTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_network_insights_analysis", e))?;

        let network_insights_path_id = model.network_insights_path_id;
        let additional_accounts = parse_string_array(attrs, "additional_accounts");
        let filter_in_arns = parse_string_array(attrs, "filter_in_arns");
        let tags = extract_tags(attrs);

        let id = model.id.unwrap_or_else(|| random_short_id("nia"));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ec2:{}:{}:network-insights-analysis/{}",
                region, ctx.default_account_id, id
            )
        });
        let start_date = model
            .start_date
            .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

        let view = NetworkInsightsAnalysisView {
            network_insights_analysis_id: id.clone(),
            network_insights_analysis_arn: arn,
            network_insights_path_id,
            additional_accounts,
            filter_in_arns,
            start_date,
            end_date: None,
            status: "succeeded".to_string(),
            status_message: None,
            warning_message: None,
            network_path_found: optional_bool(attrs, "path_found").unwrap_or(true),
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.network_insights_analyses.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for a in view.network_insights_analyses.values() {
            let attrs = serde_json::json!({
                "id": a.network_insights_analysis_id,
                "arn": a.network_insights_analysis_arn,
                "network_insights_path_id": a.network_insights_path_id,
                "additional_accounts": a.additional_accounts,
                "filter_in_arns": a.filter_in_arns,
                "start_date": a.start_date,
                "status": a.status,
                "path_found": a.network_path_found,
                "tags": a.tags,
                "tags_all": a.tags,
            });
            results.push(ExtractedResource {
                name: a.network_insights_analysis_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ===========================================================================
// Batch C converters: resources promoted from Default::default() stubs to
// real state-backed implementations in EC2 handler Batches A and B.
// ===========================================================================

// ---------------------------------------------------------------------------
// aws_ec2_local_gateway_route
// ---------------------------------------------------------------------------

pub struct AwsEc2LocalGatewayRouteConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2LocalGatewayRouteConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2LocalGatewayRouteConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_local_gateway_route"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2LocalGatewayRouteConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::LocalGatewayRouteTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_local_gateway_route", e))?;

        let local_gateway_route_table_id = model.local_gateway_route_table_id;
        let destination_cidr_block = model.destination_cidr_block;
        let local_gateway_virtual_interface_group_id =
            model.local_gateway_virtual_interface_group_id;

        let view = LocalGatewayRouteView {
            destination_cidr_block,
            local_gateway_route_table_id,
            r#type: "static".to_string(),
            state: "active".to_string(),
            local_gateway_route_table_arn: model.local_gateway_route_table_arn,
            owner_id: ctx.default_account_id.clone(),
            subnet_id: model.subnet_id,
            network_interface_id: model.network_interface_id,
            destination_prefix_list_id: model.destination_prefix_list_id,
            coip_pool_id: model.coip_pool_id,
            local_gateway_virtual_interface_group_id,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.local_gateway_routes.push(view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for r in view.local_gateway_routes.iter() {
            let attrs = serde_json::json!({
                "id": format!(
                    "{}_{}",
                    r.local_gateway_route_table_id,
                    r.destination_cidr_block,
                ),
                "local_gateway_route_table_id": r.local_gateway_route_table_id,
                "destination_cidr_block": r.destination_cidr_block,
                "local_gateway_virtual_interface_group_id":
                    r.local_gateway_virtual_interface_group_id.clone().unwrap_or_default(),
                "network_interface_id": r.network_interface_id.clone().unwrap_or_default(),
                "state": r.state,
                "type": r.r#type,
            });
            results.push(ExtractedResource {
                name: format!(
                    "{}_{}",
                    r.local_gateway_route_table_id, r.destination_cidr_block,
                ),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_local_gateway_route_table_vpc_association
// ---------------------------------------------------------------------------

pub struct AwsEc2LocalGatewayRouteTableVpcAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2LocalGatewayRouteTableVpcAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2LocalGatewayRouteTableVpcAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_local_gateway_route_table_vpc_association"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2LocalGatewayRouteTableVpcAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::LocalGatewayRouteTableVpcAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_local_gateway_route_table_vpc_association", e)
            })?;

        let local_gateway_route_table_id = model.local_gateway_route_table_id;
        let vpc_id = model.vpc_id;
        let tags = extract_tags(attrs);

        let id = model.id.unwrap_or_else(|| random_short_id("lgw-vpc-assoc"));
        let local_gateway_route_table_arn =
            model.local_gateway_route_table_arn.unwrap_or_else(|| {
                format!(
                    "arn:aws:ec2:{}:{}:local-gateway-route-table/{}",
                    region, ctx.default_account_id, local_gateway_route_table_id,
                )
            });
        let local_gateway_id = model.local_gateway_id.unwrap_or_default();

        let view = LocalGatewayRouteTableVpcAssociationView {
            local_gateway_route_table_vpc_association_id: id.clone(),
            local_gateway_route_table_id,
            local_gateway_route_table_arn,
            local_gateway_id,
            vpc_id,
            owner_id: ctx.default_account_id.clone(),
            state: "associated".to_string(),
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view
            .local_gateway_route_table_vpc_associations
            .insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for a in view.local_gateway_route_table_vpc_associations.values() {
            let attrs = serde_json::json!({
                "id": a.local_gateway_route_table_vpc_association_id,
                "local_gateway_route_table_id": a.local_gateway_route_table_id,
                "local_gateway_route_table_arn": a.local_gateway_route_table_arn,
                "local_gateway_id": a.local_gateway_id,
                "vpc_id": a.vpc_id,
                "owner_id": a.owner_id,
                "state": a.state,
                "tags": a.tags,
                "tags_all": a.tags,
            });
            results.push(ExtractedResource {
                name: a.local_gateway_route_table_vpc_association_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_ipam_pool_cidr_allocation
// ---------------------------------------------------------------------------

pub struct AwsVpcIpamPoolCidrAllocationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcIpamPoolCidrAllocationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcIpamPoolCidrAllocationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_ipam_pool_cidr_allocation"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_ipam_pool"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcIpamPoolCidrAllocationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcIpamPoolCidrAllocationTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_vpc_ipam_pool_cidr_allocation", e))?;

        let ipam_pool_id = model.ipam_pool_id;
        let cidr = model.cidr.unwrap_or_default();
        let description = model.description;

        let allocation_id = model
            .id
            .or(model.ipam_pool_allocation_id)
            .unwrap_or_else(|| random_short_id("ipam-pool-alloc"));

        let view = IpamPoolAllocationView {
            ipam_pool_allocation_id: allocation_id.clone(),
            cidr,
            ipam_pool_id,
            description,
            resource_id: model.resource_id,
            resource_type: model.resource_type.unwrap_or_else(|| "custom".to_string()),
            resource_region: model.resource_region,
            resource_owner: model
                .resource_owner
                .or_else(|| Some(ctx.default_account_id.clone())),
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.ipam_pool_allocations.push(view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for a in view.ipam_pool_allocations.iter() {
            let attrs = serde_json::json!({
                "id": format!("{}_{}", a.ipam_pool_allocation_id, a.ipam_pool_id),
                "ipam_pool_allocation_id": a.ipam_pool_allocation_id,
                "ipam_pool_id": a.ipam_pool_id,
                "cidr": a.cidr,
                "description": a.description.clone().unwrap_or_default(),
                "resource_id": a.resource_id.clone().unwrap_or_default(),
                "resource_type": a.resource_type,
                "resource_region": a.resource_region.clone().unwrap_or_default(),
                "resource_owner": a.resource_owner.clone().unwrap_or_default(),
            });
            results.push(ExtractedResource {
                name: a.ipam_pool_allocation_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_capacity_block_reservation
// ---------------------------------------------------------------------------
//
// Maps to `state.capacity_blocks` (purchased Capacity Block reservations,
// surfaced via the `DescribeCapacityBlocks` handler wired in Batch A).

pub struct AwsEc2CapacityBlockReservationConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2CapacityBlockReservationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2CapacityBlockReservationConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_capacity_block_reservation"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2CapacityBlockReservationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::CapacityBlockReservationTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_capacity_block_reservation", e))?;

        let capacity_block_offering_id = model.capacity_block_offering_id;
        let tags = extract_tags(attrs);

        let id = model
            .id
            .or(model.capacity_block_id)
            .unwrap_or_else(|| random_short_id("cb"));
        let capacity_reservation_id = model
            .capacity_reservation_id
            .unwrap_or_else(|| random_short_id("cr"));
        let capacity_reservation_arn = model.capacity_reservation_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ec2:{}:{}:capacity-reservation/{}",
                region, ctx.default_account_id, capacity_reservation_id,
            )
        });

        let view = CapacityBlockView {
            capacity_block_id: id.clone(),
            capacity_reservation_id,
            capacity_block_offering_id,
            instance_type: model.instance_type.unwrap_or_default(),
            instance_count: optional_i64(attrs, "instance_count").unwrap_or(0) as i32,
            availability_zone: model.availability_zone.unwrap_or_default(),
            start_date: model.start_date.unwrap_or_default(),
            end_date: model.end_date.unwrap_or_default(),
            tenancy: model.tenancy.unwrap_or_else(|| "default".to_string()),
            currency_code: model.currency_code.unwrap_or_else(|| "USD".to_string()),
            upfront_fee: model.upfront_fee.unwrap_or_default(),
            commitment_duration_in_seconds: 0,
            capacity_reservation_arn,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.capacity_blocks.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for b in view.capacity_blocks.values() {
            let attrs = serde_json::json!({
                "id": b.capacity_block_id,
                "capacity_block_id": b.capacity_block_id,
                "capacity_block_offering_id": b.capacity_block_offering_id,
                "capacity_reservation_id": b.capacity_reservation_id,
                "capacity_reservation_arn": b.capacity_reservation_arn,
                "instance_type": b.instance_type,
                "instance_count": b.instance_count,
                "availability_zone": b.availability_zone,
                "start_date": b.start_date,
                "end_date": b.end_date,
                "tenancy": b.tenancy,
                "currency_code": b.currency_code,
                "upfront_fee": b.upfront_fee,
                "tags": b.tags,
                "tags_all": b.tags,
            });
            results.push(ExtractedResource {
                name: b.capacity_block_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_instance
// ---------------------------------------------------------------------------

pub struct AwsInstanceConverter {
    service: Arc<Ec2Service>,
}

impl AwsInstanceConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsInstanceConverter {
    fn resource_type(&self) -> &str {
        "aws_instance"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_subnet", "aws_security_group", "aws_key_pair"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsInstanceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::InstanceTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_instance", e))?;

        let instance_id = model
            .id
            .unwrap_or_else(|| format!("i-{}", &uuid::Uuid::new_v4().to_string()[..17]));
        let tags = extract_tags(attrs);

        // security_groups (classic) and vpc_security_group_ids (vpc) are both
        // pulled into a unified set on the view.
        let mut security_groups: Vec<String> = parse_string_array(attrs, "security_groups");
        let vpc_sg_ids = parse_string_array(attrs, "vpc_security_group_ids");
        for sg in vpc_sg_ids {
            if !security_groups.contains(&sg) {
                security_groups.push(sg);
            }
        }

        // Resolve vpc_id from subnet snapshot if a subnet is given.
        let subnet_id = model.subnet_id.clone();
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let vpc_id = subnet_id
            .as_ref()
            .and_then(|s| snapshot.subnets.get(s).map(|sn| sn.vpc_id.clone()));

        let placement_az = model
            .availability_zone
            .clone()
            .or_else(|| {
                subnet_id.as_ref().and_then(|s| {
                    snapshot
                        .subnets
                        .get(s)
                        .map(|sn| sn.availability_zone.clone())
                })
            })
            .unwrap_or_else(|| format!("{}a", region));

        let placement_partition_number = if model.placement_partition_number > 0 {
            Some(model.placement_partition_number as i32)
        } else {
            None
        };

        let iam_instance_profile_arn = model.iam_instance_profile.filter(|s| !s.is_empty());

        let instance_view = InstanceView {
            instance_id: instance_id.clone(),
            image_id: model.ami.unwrap_or_default(),
            instance_type: model
                .instance_type
                .unwrap_or_else(|| "t2.micro".to_string()),
            state: InstanceStateView {
                code: 16,
                name: "running".to_string(),
            },
            private_ip_address: model.private_ip,
            public_ip_address: model.public_ip,
            subnet_id,
            vpc_id,
            key_name: model.key_name,
            security_groups,
            launch_time: "1970-01-01T00:00:00Z".to_string(),
            tags,
            iam_instance_profile_arn,
            monitoring_state: if model.monitoring {
                "enabled".to_string()
            } else {
                "disabled".to_string()
            },
            placement_az,
            placement_group_name: model.placement_group.filter(|s| !s.is_empty()),
            placement_tenancy: model.tenancy,
            placement_host_id: model.host_id.filter(|s| !s.is_empty()),
            placement_affinity: None,
            placement_partition_number,
            owner_id: ctx.default_account_id.clone(),
            classic_link_vpc: None,
            private_dns_hostname_type: None,
            enable_resource_name_dns_a_record: None,
            enable_resource_name_dns_aaaa_record: None,
            credit_specification: None,
            cpu_options: None,
            maintenance_options: None,
            network_bandwidth_weighting: None,
            lifecycle: model.instance_lifecycle,
            product_codes: vec![],
            capacity_reservation_specification: None,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.instances.insert(instance_id, instance_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for inst in view.instances.values() {
            let arn = format!(
                "arn:aws:ec2:{}:{}:instance/{}",
                ctx.default_region, ctx.default_account_id, inst.instance_id
            );
            let attrs = serde_json::json!({
                "id": inst.instance_id,
                "ami": inst.image_id,
                "arn": arn,
                "instance_type": inst.instance_type,
                "instance_state": inst.state.name,
                "subnet_id": inst.subnet_id,
                "key_name": inst.key_name,
                "availability_zone": inst.placement_az,
                "private_ip": inst.private_ip_address,
                "public_ip": inst.public_ip_address,
                "iam_instance_profile": inst.iam_instance_profile_arn,
                "monitoring": inst.monitoring_state == "enabled",
                "tenancy": inst.placement_tenancy,
                "host_id": inst.placement_host_id,
                "placement_group": inst.placement_group_name,
                "placement_partition_number": inst.placement_partition_number.unwrap_or(0),
                "security_groups": inst.security_groups,
                "vpc_security_group_ids": inst.security_groups,
                "outpost_arn": "",
                "instance_lifecycle": inst.lifecycle,
                "tags": inst.tags,
                "tags_all": inst.tags,
            });
            results.push(ExtractedResource {
                name: inst.instance_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_launch_template
// ---------------------------------------------------------------------------

pub struct AwsLaunchTemplateConverter {
    service: Arc<Ec2Service>,
}

impl AwsLaunchTemplateConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLaunchTemplateConverter {
    fn resource_type(&self) -> &str {
        "aws_launch_template"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsLaunchTemplateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::LaunchTemplateTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_launch_template", e))?;

        let template_id = model
            .id
            .unwrap_or_else(|| format!("lt-{}", &uuid::Uuid::new_v4().to_string()[..17]));
        let template_name = model
            .name
            .or(model.name_prefix)
            .unwrap_or_else(|| template_id.clone());
        let default_version = if model.default_version > 0 {
            model.default_version
        } else {
            1
        };
        let latest_version = if model.latest_version > 0 {
            model.latest_version
        } else {
            default_version
        };

        let lt_view = LaunchTemplateView {
            launch_template_id: template_id.clone(),
            launch_template_name: template_name,
            default_version_number: default_version,
            latest_version_number: latest_version,
            tags: extract_tags(attrs),
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.launch_templates.insert(template_id, lt_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for lt in view.launch_templates.values() {
            let arn = format!(
                "arn:aws:ec2:{}:{}:launch-template/{}",
                ctx.default_region, ctx.default_account_id, lt.launch_template_id
            );
            let attrs = serde_json::json!({
                "id": lt.launch_template_id,
                "arn": arn,
                "name": lt.launch_template_name,
                "default_version": lt.default_version_number,
                "latest_version": lt.latest_version_number,
                "tags": lt.tags,
                "tags_all": lt.tags,
            });
            results.push(ExtractedResource {
                name: lt.launch_template_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_volume_attachment
// ---------------------------------------------------------------------------

pub struct AwsVolumeAttachmentConverter {
    service: Arc<Ec2Service>,
}

impl AwsVolumeAttachmentConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVolumeAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_volume_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_instance"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVolumeAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VolumeAttachmentTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_volume_attachment", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(volume) = view.volumes.get_mut(&model.volume_id) {
            let already = volume
                .attachments
                .iter()
                .any(|a| a.instance_id == model.instance_id);
            if !already {
                volume.attachments.push(VolumeAttachmentView {
                    volume_id: model.volume_id.clone(),
                    instance_id: model.instance_id.clone(),
                    device: model.device_name.clone(),
                    state: "attached".to_string(),
                    attach_time: "1970-01-01T00:00:00Z".to_string(),
                    delete_on_termination: false,
                });
                volume.state = "in-use".to_string();
            }
        } else {
            warnings.push(format!(
                "volume '{}' not found in state; attachment skipped",
                model.volume_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for volume in view.volumes.values() {
            for att in &volume.attachments {
                let id = format!("{}-{}-{}", att.device, att.volume_id, att.instance_id);
                let attrs = serde_json::json!({
                    "id": id,
                    "volume_id": att.volume_id,
                    "instance_id": att.instance_id,
                    "device_name": att.device,
                    "force_detach": false,
                    "skip_destroy": false,
                    "stop_instance_before_detaching": false,
                });
                results.push(ExtractedResource {
                    name: format!("{}-{}", att.volume_id, att.instance_id),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_network_interface
// ---------------------------------------------------------------------------

pub struct AwsNetworkInterfaceConverter {
    service: Arc<Ec2Service>,
}

impl AwsNetworkInterfaceConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkInterfaceConverter {
    fn resource_type(&self) -> &str {
        "aws_network_interface"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_subnet", "aws_security_group"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsNetworkInterfaceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::NetworkInterfaceTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_network_interface", e))?;

        let eni_id = model
            .id
            .unwrap_or_else(|| format!("eni-{}", &uuid::Uuid::new_v4().to_string()[..17]));
        let subnet_id = model.subnet_id;
        let security_groups = parse_string_array(attrs, "security_groups");

        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let vpc_id = snapshot
            .subnets
            .get(&subnet_id)
            .map(|s| s.vpc_id.clone())
            .unwrap_or_default();

        let eni_view = NetworkInterfaceView {
            network_interface_id: eni_id.clone(),
            subnet_id,
            vpc_id,
            description: model.description.unwrap_or_default(),
            private_ip_address: model.private_ip.unwrap_or_default(),
            status: "available".to_string(),
            attachment_id: None,
            instance_id: None,
            device_index: None,
            security_groups,
            source_dest_check: attrs
                .get("source_dest_check")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            tags: extract_tags(attrs),
            public_ip_dns_hostname_type: None,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.network_interfaces.insert(eni_id, eni_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for eni in view.network_interfaces.values() {
            let arn = format!(
                "arn:aws:ec2:{}:{}:network-interface/{}",
                ctx.default_region, ctx.default_account_id, eni.network_interface_id
            );
            let attrs = serde_json::json!({
                "id": eni.network_interface_id,
                "arn": arn,
                "subnet_id": eni.subnet_id,
                "description": eni.description,
                "private_ip": eni.private_ip_address,
                "private_ips": [eni.private_ip_address],
                "security_groups": eni.security_groups,
                "source_dest_check": eni.source_dest_check,
                "interface_type": "interface",
                "owner_id": ctx.default_account_id,
                "tags": eni.tags,
                "tags_all": eni.tags,
            });
            results.push(ExtractedResource {
                name: eni.network_interface_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_network_interface_attachment
// ---------------------------------------------------------------------------

pub struct AwsNetworkInterfaceAttachmentConverter {
    service: Arc<Ec2Service>,
}

impl AwsNetworkInterfaceAttachmentConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkInterfaceAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_network_interface_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_network_interface", "aws_instance"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsNetworkInterfaceAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::NetworkInterfaceAttachmentTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_network_interface_attachment", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(eni) = view.network_interfaces.get_mut(&model.network_interface_id) {
            let attachment_id = model.attachment_id.clone().unwrap_or_else(|| {
                format!("eni-attach-{}", &uuid::Uuid::new_v4().to_string()[..17])
            });
            eni.attachment_id = Some(attachment_id);
            eni.instance_id = Some(model.instance_id.clone());
            eni.device_index = Some(model.device_index as i32);
            eni.status = "in-use".to_string();
        } else {
            warnings.push(format!(
                "network interface '{}' not found in state; attachment skipped",
                model.network_interface_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for eni in view.network_interfaces.values() {
            if let (Some(att_id), Some(inst_id)) =
                (eni.attachment_id.as_ref(), eni.instance_id.as_ref())
            {
                let attrs = serde_json::json!({
                    "id": att_id,
                    "attachment_id": att_id,
                    "network_interface_id": eni.network_interface_id,
                    "instance_id": inst_id,
                    "device_index": eni.device_index.unwrap_or(0),
                    "status": eni.status,
                });
                results.push(ExtractedResource {
                    name: att_id.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_network_interface_sg_attachment
// ---------------------------------------------------------------------------

pub struct AwsNetworkInterfaceSgAttachmentConverter {
    service: Arc<Ec2Service>,
}

impl AwsNetworkInterfaceSgAttachmentConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkInterfaceSgAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_network_interface_sg_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_network_interface", "aws_security_group"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsNetworkInterfaceSgAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::NetworkInterfaceSgAttachmentTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_network_interface_sg_attachment", e)
            })?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(eni) = view.network_interfaces.get_mut(&model.network_interface_id) {
            if !eni.security_groups.contains(&model.security_group_id) {
                eni.security_groups.push(model.security_group_id.clone());
            }
        } else {
            warnings.push(format!(
                "network interface '{}' not found in state; sg attachment skipped",
                model.network_interface_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for eni in view.network_interfaces.values() {
            for sg_id in &eni.security_groups {
                let id = format!("{}_{}", eni.network_interface_id, sg_id);
                let attrs = serde_json::json!({
                    "id": id,
                    "network_interface_id": eni.network_interface_id,
                    "security_group_id": sg_id,
                });
                results.push(ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_eip_association
// ---------------------------------------------------------------------------

pub struct AwsEipAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsEipAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEipAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_eip_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_eip", "aws_instance", "aws_network_interface"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEipAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::EipAssociationTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_eip_association", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];

        // Locate the EIP by allocation_id, falling back to public_ip.
        let eip_key: Option<String> = if let Some(aid) = model.allocation_id.as_ref() {
            view.elastic_ips.contains_key(aid).then(|| aid.clone())
        } else if let Some(pip) = model.public_ip.as_ref() {
            view.elastic_ips
                .iter()
                .find(|(_, eip)| eip.public_ip == *pip)
                .map(|(k, _)| k.clone())
        } else {
            None
        };

        if let Some(key) = eip_key {
            if let Some(eip) = view.elastic_ips.get_mut(&key) {
                let association_id = model.id.clone().unwrap_or_else(|| {
                    format!("eipassoc-{}", &uuid::Uuid::new_v4().to_string()[..17])
                });
                eip.association_id = Some(association_id);
                eip.instance_id = model.instance_id.clone();
                eip.network_interface_id = model.network_interface_id.clone();
                eip.private_ip_address = model.private_ip_address.clone();
            }
        } else {
            warnings.push(format!(
                "eip not found in state for allocation_id={:?}/public_ip={:?}; association skipped",
                model.allocation_id, model.public_ip
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for eip in view.elastic_ips.values() {
            if let Some(assoc_id) = eip.association_id.as_ref() {
                let attrs = serde_json::json!({
                    "id": assoc_id,
                    "allocation_id": eip.allocation_id,
                    "instance_id": eip.instance_id,
                    "network_interface_id": eip.network_interface_id,
                    "private_ip_address": eip.private_ip_address,
                    "public_ip": eip.public_ip,
                    "allow_reassociation": false,
                });
                results.push(ExtractedResource {
                    name: assoc_id.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_internet_gateway_attachment
// ---------------------------------------------------------------------------

pub struct AwsInternetGatewayAttachmentConverter {
    service: Arc<Ec2Service>,
}

impl AwsInternetGatewayAttachmentConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsInternetGatewayAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_internet_gateway_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_internet_gateway", "aws_vpc"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsInternetGatewayAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::InternetGatewayAttachmentTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_internet_gateway_attachment", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(igw) = view.igws.get_mut(&model.internet_gateway_id) {
            if !igw.attachments.iter().any(|a| a.vpc_id == model.vpc_id) {
                igw.attachments.push(IgwAttachmentView {
                    vpc_id: model.vpc_id.clone(),
                    state: "available".to_string(),
                });
            }
        } else {
            warnings.push(format!(
                "internet gateway '{}' not found in state; attachment skipped",
                model.internet_gateway_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for igw in view.igws.values() {
            for att in &igw.attachments {
                let id = format!("{}:{}", igw.igw_id, att.vpc_id);
                let attrs = serde_json::json!({
                    "id": id,
                    "internet_gateway_id": igw.igw_id,
                    "vpc_id": att.vpc_id,
                });
                results.push(ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_flow_log
// ---------------------------------------------------------------------------

pub struct AwsFlowLogConverter {
    service: Arc<Ec2Service>,
}

impl AwsFlowLogConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsFlowLogConverter {
    fn resource_type(&self) -> &str {
        "aws_flow_log"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsFlowLogConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::FlowLogTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_flow_log", e))?;

        let flow_log_id = model
            .id
            .unwrap_or_else(|| format!("fl-{}", &uuid::Uuid::new_v4().to_string()[..17]));

        // Pick the most specific resource id available; aws_flow_log applies
        // to exactly one of vpc/subnet/eni/tgw/tgw_attachment.
        let resource_id = model
            .eni_id
            .clone()
            .or(model.subnet_id.clone())
            .or(model.vpc_id.clone())
            .or(model.transit_gateway_attachment_id.clone())
            .or(model.transit_gateway_id.clone())
            .unwrap_or_default();

        let log_destination_type = model
            .log_destination_type
            .unwrap_or_else(|| "cloud-watch-logs".to_string());

        let fl_view = FlowLogView {
            flow_log_id: flow_log_id.clone(),
            resource_id,
            traffic_type: model.traffic_type.unwrap_or_else(|| "ALL".to_string()),
            log_destination_type,
            log_destination: model.log_destination,
            log_group_name: model.log_group_name,
            deliver_logs_status: "SUCCESS".to_string(),
            flow_log_status: "ACTIVE".to_string(),
            tags: extract_tags(attrs),
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.flow_logs.insert(flow_log_id, fl_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for fl in view.flow_logs.values() {
            let arn = format!(
                "arn:aws:ec2:{}:{}:vpc-flow-log/{}",
                ctx.default_region, ctx.default_account_id, fl.flow_log_id
            );
            // Classify the resource_id by prefix so we can populate the right
            // TF attribute (eni_id / subnet_id / vpc_id / transit_gateway_id).
            let mut vpc_id = serde_json::Value::Null;
            let mut subnet_id = serde_json::Value::Null;
            let mut eni_id = serde_json::Value::Null;
            let mut tgw_id = serde_json::Value::Null;
            let mut tgw_attachment_id = serde_json::Value::Null;
            if fl.resource_id.starts_with("vpc-") {
                vpc_id = serde_json::Value::String(fl.resource_id.clone());
            } else if fl.resource_id.starts_with("subnet-") {
                subnet_id = serde_json::Value::String(fl.resource_id.clone());
            } else if fl.resource_id.starts_with("eni-") {
                eni_id = serde_json::Value::String(fl.resource_id.clone());
            } else if fl.resource_id.starts_with("tgw-attach-") {
                tgw_attachment_id = serde_json::Value::String(fl.resource_id.clone());
            } else if fl.resource_id.starts_with("tgw-") {
                tgw_id = serde_json::Value::String(fl.resource_id.clone());
            }
            let attrs = serde_json::json!({
                "id": fl.flow_log_id,
                "arn": arn,
                "traffic_type": fl.traffic_type,
                "log_destination_type": fl.log_destination_type,
                "log_destination": fl.log_destination,
                "log_group_name": fl.log_group_name,
                "vpc_id": vpc_id,
                "subnet_id": subnet_id,
                "eni_id": eni_id,
                "transit_gateway_id": tgw_id,
                "transit_gateway_attachment_id": tgw_attachment_id,
                "tags": fl.tags,
                "tags_all": fl.tags,
            });
            results.push(ExtractedResource {
                name: fl.flow_log_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_dhcp_options
// ---------------------------------------------------------------------------

pub struct AwsVpcDhcpOptionsConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcDhcpOptionsConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcDhcpOptionsConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_dhcp_options"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcDhcpOptionsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcDhcpOptionsTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpc_dhcp_options", e))?;

        let dhcp_options_id = model
            .id
            .unwrap_or_else(|| format!("dopt-{}", &uuid::Uuid::new_v4().to_string()[..17]));

        let mut configurations = vec![];
        if let Some(s) = model.domain_name.filter(|s| !s.is_empty()) {
            configurations.push(winterbaume_ec2::views::DhcpConfigurationView {
                key: "domain-name".to_string(),
                values: vec![s],
            });
        }
        for (tf_key, view_key) in [
            ("domain_name_servers", "domain-name-servers"),
            ("ntp_servers", "ntp-servers"),
            ("netbios_name_servers", "netbios-name-servers"),
        ] {
            let values = parse_string_array(attrs, tf_key);
            if !values.is_empty() {
                configurations.push(winterbaume_ec2::views::DhcpConfigurationView {
                    key: view_key.to_string(),
                    values,
                });
            }
        }
        if let Some(s) = model.netbios_node_type.filter(|s| !s.is_empty()) {
            configurations.push(winterbaume_ec2::views::DhcpConfigurationView {
                key: "netbios-node-type".to_string(),
                values: vec![s],
            });
        }
        if let Some(s) = model
            .ipv6_address_preferred_lease_time
            .filter(|s| !s.is_empty())
        {
            configurations.push(winterbaume_ec2::views::DhcpConfigurationView {
                key: "ipv6-address-preferred-lease-time".to_string(),
                values: vec![s],
            });
        }

        let dhcp_view = DhcpOptionsView {
            dhcp_options_id: dhcp_options_id.clone(),
            configurations,
            tags: extract_tags(attrs),
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.dhcp_options.insert(dhcp_options_id, dhcp_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for dopt in view.dhcp_options.values() {
            let arn = format!(
                "arn:aws:ec2:{}:{}:dhcp-options/{}",
                ctx.default_region, ctx.default_account_id, dopt.dhcp_options_id
            );
            let mut domain_name = String::new();
            let mut domain_name_servers: Vec<String> = vec![];
            let mut ntp_servers: Vec<String> = vec![];
            let mut netbios_name_servers: Vec<String> = vec![];
            let mut netbios_node_type = String::new();
            let mut ipv6_lease = String::new();
            for cfg in &dopt.configurations {
                match cfg.key.as_str() {
                    "domain-name" => domain_name = cfg.values.first().cloned().unwrap_or_default(),
                    "domain-name-servers" => domain_name_servers = cfg.values.clone(),
                    "ntp-servers" => ntp_servers = cfg.values.clone(),
                    "netbios-name-servers" => netbios_name_servers = cfg.values.clone(),
                    "netbios-node-type" => {
                        netbios_node_type = cfg.values.first().cloned().unwrap_or_default();
                    }
                    "ipv6-address-preferred-lease-time" => {
                        ipv6_lease = cfg.values.first().cloned().unwrap_or_default();
                    }
                    _ => {}
                }
            }
            let attrs = serde_json::json!({
                "id": dopt.dhcp_options_id,
                "arn": arn,
                "domain_name": domain_name,
                "domain_name_servers": domain_name_servers,
                "ntp_servers": ntp_servers,
                "netbios_name_servers": netbios_name_servers,
                "netbios_node_type": netbios_node_type,
                "ipv6_address_preferred_lease_time": ipv6_lease,
                "owner_id": ctx.default_account_id,
                "tags": dopt.tags,
                "tags_all": dopt.tags,
            });
            results.push(ExtractedResource {
                name: dopt.dhcp_options_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_dhcp_options_association
// ---------------------------------------------------------------------------

pub struct AwsVpcDhcpOptionsAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcDhcpOptionsAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcDhcpOptionsAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_dhcp_options_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc", "aws_vpc_dhcp_options"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcDhcpOptionsAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcDhcpOptionsAssociationTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_vpc_dhcp_options_association", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(vpc) = view.vpcs.get_mut(&model.vpc_id) {
            vpc.dhcp_options_id = model.dhcp_options_id.clone();
        } else {
            warnings.push(format!(
                "vpc '{}' not found in state; dhcp options association skipped",
                model.vpc_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for vpc in view.vpcs.values() {
            // Filter out the implicit "dopt-default" baseline.
            if vpc.dhcp_options_id.is_empty() || vpc.dhcp_options_id == "dopt-default" {
                continue;
            }
            let id = format!("dopt-assoc-{}-{}", vpc.dhcp_options_id, vpc.vpc_id);
            let attrs = serde_json::json!({
                "id": id,
                "dhcp_options_id": vpc.dhcp_options_id,
                "vpc_id": vpc.vpc_id,
            });
            results.push(ExtractedResource {
                name: id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_security_group_rule (legacy single-rule resource)
// ---------------------------------------------------------------------------

pub struct AwsSecurityGroupRuleConverter {
    service: Arc<Ec2Service>,
}

impl AwsSecurityGroupRuleConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityGroupRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_security_group_rule"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_security_group"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsSecurityGroupRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::SecurityGroupRuleTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_security_group_rule", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];

        let cidr_blocks: Vec<IpRangeView> = parse_string_array(attrs, "cidr_blocks")
            .into_iter()
            .map(|s| IpRangeView {
                cidr_ip: s,
                description: model.description.clone(),
            })
            .collect();
        let ipv6_cidr_blocks: Vec<Ipv6RangeView> = parse_string_array(attrs, "ipv6_cidr_blocks")
            .into_iter()
            .map(|s| Ipv6RangeView {
                cidr_ipv6: s,
                description: model.description.clone(),
            })
            .collect();
        let mut sg_refs: Vec<UserIdGroupPairView> =
            parse_string_array(attrs, "source_security_group_ids")
                .into_iter()
                .map(|gid| UserIdGroupPairView {
                    group_id: gid,
                    user_id: None,
                })
                .collect();
        if let Some(gid) = model.source_security_group_id.as_ref() {
            if !gid.is_empty() && !sg_refs.iter().any(|p| p.group_id == *gid) {
                sg_refs.push(UserIdGroupPairView {
                    group_id: gid.clone(),
                    user_id: None,
                });
            }
        }
        if model.self_
            && !sg_refs
                .iter()
                .any(|p| p.group_id == model.security_group_id)
        {
            sg_refs.push(UserIdGroupPairView {
                group_id: model.security_group_id.clone(),
                user_id: None,
            });
        }

        let from_port = if model.from_port == 0 && model.to_port == 0 && model.protocol == "-1" {
            None
        } else {
            Some(model.from_port)
        };
        let to_port = if model.from_port == 0 && model.to_port == 0 && model.protocol == "-1" {
            None
        } else {
            Some(model.to_port)
        };

        let permission = IpPermissionView {
            from_port,
            to_port,
            ip_protocol: model.protocol.clone(),
            ip_ranges: cidr_blocks,
            ipv6_ranges: ipv6_cidr_blocks,
            user_id_group_pairs: sg_refs,
        };

        if let Some(sg) = view.security_groups.get_mut(&model.security_group_id) {
            match model.rule_type.as_str() {
                "ingress" => sg.ingress_rules.push(permission),
                "egress" => sg.egress_rules.push(permission),
                other => warnings.push(format!(
                    "aws_security_group_rule: unknown type '{other}'; ignored"
                )),
            }
        } else {
            warnings.push(format!(
                "security group '{}' not found in state; rule skipped",
                model.security_group_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Rules are already extracted inline with their parent aws_security_group;
        // emitting them again as discrete aws_security_group_rule rows would
        // produce duplicate state on a round-trip. Leave empty.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_security_group_ingress_rule
// ---------------------------------------------------------------------------

pub struct AwsVpcSecurityGroupIngressRuleConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcSecurityGroupIngressRuleConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcSecurityGroupIngressRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_security_group_ingress_rule"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_security_group"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx, false).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_security_group_egress_rule
// ---------------------------------------------------------------------------

pub struct AwsVpcSecurityGroupEgressRuleConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcSecurityGroupEgressRuleConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcSecurityGroupEgressRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_security_group_egress_rule"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_security_group"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            AwsVpcSecurityGroupIngressRuleConverter {
                service: Arc::clone(&self.service),
            }
            .do_inject(instance, ctx, true)
            .await
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcSecurityGroupIngressRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
        egress: bool,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let resource_label = if egress {
            "aws_vpc_security_group_egress_rule"
        } else {
            "aws_vpc_security_group_ingress_rule"
        };
        // Both ingress/egress models share the same field set.
        let model: ec2_gen::VpcSecurityGroupIngressRuleTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error(resource_label, e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];

        let mut ip_ranges = vec![];
        if let Some(cidr) = model.cidr_ipv4.as_ref() {
            ip_ranges.push(IpRangeView {
                cidr_ip: cidr.clone(),
                description: model.description.clone(),
            });
        }
        let mut ipv6_ranges = vec![];
        if let Some(cidr) = model.cidr_ipv6.as_ref() {
            ipv6_ranges.push(Ipv6RangeView {
                cidr_ipv6: cidr.clone(),
                description: model.description.clone(),
            });
        }
        let mut sg_refs = vec![];
        if let Some(gid) = model.referenced_security_group_id.as_ref() {
            sg_refs.push(UserIdGroupPairView {
                group_id: gid.clone(),
                user_id: None,
            });
        }

        let from_port = if model.from_port == 0 && model.to_port == 0 && model.ip_protocol == "-1" {
            None
        } else {
            Some(model.from_port)
        };
        let to_port = if model.from_port == 0 && model.to_port == 0 && model.ip_protocol == "-1" {
            None
        } else {
            Some(model.to_port)
        };

        let permission = IpPermissionView {
            from_port,
            to_port,
            ip_protocol: model.ip_protocol.clone(),
            ip_ranges,
            ipv6_ranges,
            user_id_group_pairs: sg_refs,
        };

        if let Some(sg) = view.security_groups.get_mut(&model.security_group_id) {
            if egress {
                sg.egress_rules.push(permission);
            } else {
                sg.ingress_rules.push(permission);
            }
        } else {
            warnings.push(format!(
                "security group '{}' not found in state; rule skipped",
                model.security_group_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_security_group_vpc_association
// ---------------------------------------------------------------------------

pub struct AwsVpcSecurityGroupVpcAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcSecurityGroupVpcAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcSecurityGroupVpcAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_security_group_vpc_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_security_group", "aws_vpc"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcSecurityGroupVpcAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcSecurityGroupVpcAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_security_group_vpc_association", e)
            })?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let already = view
            .security_group_vpc_associations
            .iter()
            .any(|a| a.group_id == model.security_group_id && a.vpc_id == model.vpc_id);
        if !already {
            view.security_group_vpc_associations
                .push(SecurityGroupVpcAssociationView {
                    group_id: model.security_group_id.clone(),
                    vpc_id: model.vpc_id.clone(),
                    vpc_owner_id: ctx.default_account_id.clone(),
                    state: model
                        .state
                        .clone()
                        .unwrap_or_else(|| "associated".to_string()),
                });
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for assoc in &view.security_group_vpc_associations {
            let id = format!("{}_{}", assoc.group_id, assoc.vpc_id);
            let attrs = serde_json::json!({
                "id": id,
                "security_group_id": assoc.group_id,
                "vpc_id": assoc.vpc_id,
                "state": assoc.state,
            });
            results.push(ExtractedResource {
                name: id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_eip_domain_name
// ---------------------------------------------------------------------------

pub struct AwsEipDomainNameConverter {
    service: Arc<Ec2Service>,
}

impl AwsEipDomainNameConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEipDomainNameConverter {
    fn resource_type(&self) -> &str {
        "aws_eip_domain_name"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_eip"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEipDomainNameConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::EipDomainNameTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_eip_domain_name", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(eip) = view.elastic_ips.get_mut(&model.allocation_id) {
            eip.address_attribute_ptr_record =
                Some(model.ptr_record.clone().unwrap_or_else(|| {
                    let mut s = model.domain_name.clone();
                    if !s.ends_with('.') {
                        s.push('.');
                    }
                    s
                }));
        } else {
            warnings.push(format!(
                "eip '{}' not found in state; domain name skipped",
                model.allocation_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for eip in view.elastic_ips.values() {
            if let Some(ptr) = eip.address_attribute_ptr_record.as_ref() {
                if ptr.is_empty() {
                    continue;
                }
                let domain_name = ptr.trim_end_matches('.').to_string();
                let attrs = serde_json::json!({
                    "id": eip.allocation_id,
                    "allocation_id": eip.allocation_id,
                    "domain_name": domain_name,
                    "ptr_record": ptr,
                });
                results.push(ExtractedResource {
                    name: eip.allocation_id.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_main_route_table_association
// ---------------------------------------------------------------------------

pub struct AwsMainRouteTableAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsMainRouteTableAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMainRouteTableAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_main_route_table_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc", "aws_route_table"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsMainRouteTableAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::MainRouteTableAssociationTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_main_route_table_association", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];

        // Clear `main` from any other route tables in this VPC, then set on
        // the target one. Each RT can have only one association, so we use the
        // existing associations list (or push a new one if empty).
        for rt in view.route_tables.values_mut() {
            if rt.vpc_id != model.vpc_id {
                continue;
            }
            for assoc in rt.associations.iter_mut() {
                if assoc.main {
                    assoc.main = false;
                }
            }
        }
        let assoc_id = model
            .id
            .clone()
            .unwrap_or_else(|| random_short_id("rtbassoc"));
        if let Some(rt) = view.route_tables.get_mut(&model.route_table_id) {
            if let Some(existing) = rt.associations.iter_mut().find(|a| a.subnet_id.is_none()) {
                existing.main = true;
                existing.association_id = assoc_id;
            } else {
                rt.associations.push(RouteTableAssociationView {
                    association_id: assoc_id,
                    subnet_id: None,
                    main: true,
                    state: "associated".to_string(),
                });
            }
        } else {
            warnings.push(format!(
                "route table '{}' not found in state; main association skipped",
                model.route_table_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for rt in view.route_tables.values() {
            for assoc in &rt.associations {
                if assoc.main {
                    let attrs = serde_json::json!({
                        "id": assoc.association_id,
                        "vpc_id": rt.vpc_id,
                        "route_table_id": rt.route_table_id,
                    });
                    results.push(ExtractedResource {
                        name: assoc.association_id.clone(),
                        account_id: ctx.default_account_id.clone(),
                        region: ctx.default_region.clone(),
                        attributes: attrs,
                    });
                }
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route_table_association
// ---------------------------------------------------------------------------

pub struct AwsRouteTableAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsRouteTableAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRouteTableAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_route_table_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_route_table", "aws_subnet", "aws_internet_gateway"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsRouteTableAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::RouteTableAssociationTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_route_table_association", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];

        let assoc_id = model
            .id
            .clone()
            .unwrap_or_else(|| random_short_id("rtbassoc"));
        if let Some(rt) = view.route_tables.get_mut(&model.route_table_id) {
            // The view doesn't track gateway_id on RouteTableAssociationView, so
            // gateway-side associations are recorded as a subnet-less entry.
            let already = rt
                .associations
                .iter()
                .any(|a| a.subnet_id == model.subnet_id && !a.main);
            if !already {
                rt.associations.push(RouteTableAssociationView {
                    association_id: assoc_id,
                    subnet_id: model.subnet_id.clone(),
                    main: false,
                    state: "associated".to_string(),
                });
            }
        } else {
            warnings.push(format!(
                "route table '{}' not found in state; association skipped",
                model.route_table_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for rt in view.route_tables.values() {
            for assoc in &rt.associations {
                if assoc.main {
                    continue;
                }
                let attrs = serde_json::json!({
                    "id": assoc.association_id,
                    "route_table_id": rt.route_table_id,
                    "subnet_id": assoc.subnet_id,
                });
                results.push(ExtractedResource {
                    name: assoc.association_id.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_network_acl_association
// ---------------------------------------------------------------------------

pub struct AwsNetworkAclAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsNetworkAclAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNetworkAclAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_network_acl_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_network_acl", "aws_subnet"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsNetworkAclAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::NetworkAclAssociationTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_network_acl_association", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];

        let assoc_id = model
            .id
            .clone()
            .unwrap_or_else(|| random_short_id("aclassoc"));

        if let Some(nacl) = view.network_acls.get_mut(&model.network_acl_id) {
            let already = nacl
                .associations
                .iter()
                .any(|a| a.subnet_id == model.subnet_id);
            if !already {
                nacl.associations.push(NetworkAclAssociationView {
                    network_acl_association_id: assoc_id,
                    network_acl_id: model.network_acl_id.clone(),
                    subnet_id: model.subnet_id.clone(),
                });
            }
        } else {
            warnings.push(format!(
                "network acl '{}' not found in state; association skipped",
                model.network_acl_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for nacl in view.network_acls.values() {
            for assoc in &nacl.associations {
                let attrs = serde_json::json!({
                    "id": assoc.network_acl_association_id,
                    "network_acl_id": assoc.network_acl_id,
                    "subnet_id": assoc.subnet_id,
                });
                results.push(ExtractedResource {
                    name: assoc.network_acl_association_id.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpn_gateway_attachment
// ---------------------------------------------------------------------------

pub struct AwsVpnGatewayAttachmentConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpnGatewayAttachmentConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpnGatewayAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_vpn_gateway_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpn_gateway", "aws_vpc"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpnGatewayAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpnGatewayAttachmentTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpn_gateway_attachment", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(vgw) = view.vpn_gateways.get_mut(&model.vpn_gateway_id) {
            if !vgw.vpc_attachments.iter().any(|a| a.vpc_id == model.vpc_id) {
                vgw.vpc_attachments.push(VgwVpcAttachmentView {
                    vpc_id: model.vpc_id.clone(),
                    state: "attached".to_string(),
                });
            }
        } else {
            warnings.push(format!(
                "vpn gateway '{}' not found in state; attachment skipped",
                model.vpn_gateway_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for vgw in view.vpn_gateways.values() {
            for att in &vgw.vpc_attachments {
                let id = format!("{}:{}", vgw.vpn_gateway_id, att.vpc_id);
                let attrs = serde_json::json!({
                    "id": id,
                    "vpn_gateway_id": vgw.vpn_gateway_id,
                    "vpc_id": att.vpc_id,
                });
                results.push(ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ami
// ---------------------------------------------------------------------------

pub struct AwsAmiConverter {
    service: Arc<Ec2Service>,
}

impl AwsAmiConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAmiConverter {
    fn resource_type(&self) -> &str {
        "aws_ami"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsAmiConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::AmiTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ami", e))?;

        let image_id = model
            .id
            .unwrap_or_else(|| format!("ami-{}", &uuid::Uuid::new_v4().to_string()[..17]));

        let image_view = ImageView {
            image_id: image_id.clone(),
            name: model.name,
            description: model.description.unwrap_or_default(),
            state: "available".to_string(),
            owner_id: ctx.default_account_id.clone(),
            architecture: model.architecture.unwrap_or_else(|| "x86_64".to_string()),
            image_type: "machine".to_string(),
            platform: None,
            virtualization_type: model
                .virtualization_type
                .unwrap_or_else(|| "hvm".to_string()),
            root_device_type: "ebs".to_string(),
            root_device_name: model
                .root_device_name
                .unwrap_or_else(|| "/dev/xvda".to_string()),
            public: model.public_,
            tags: extract_tags(attrs),
            source_instance_id: None,
            source_instance_type: String::new(),
            launch_permissions: vec![],
            recycle_bin_state: None,
            deprecation_time: model.deprecation_time,
            recycle_bin_enter_time: None,
            product_codes: vec![],
            fast_launch_state: None,
            deregistration_protection: None,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.images.insert(image_id, image_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for image in view.images.values() {
            // Only emit images that are not the product of an instance/copy,
            // to roughly match the aws_ami resource (vs aws_ami_copy /
            // aws_ami_from_instance). Source-less images are aws_ami.
            if image.source_instance_id.is_some() {
                continue;
            }
            let arn = format!(
                "arn:aws:ec2:{}::image/{}",
                ctx.default_region, image.image_id
            );
            let attrs = serde_json::json!({
                "id": image.image_id,
                "arn": arn,
                "name": image.name,
                "description": image.description,
                "architecture": image.architecture,
                "virtualization_type": image.virtualization_type,
                "root_device_name": image.root_device_name,
                "public": image.public,
                "owner_id": image.owner_id,
                "platform": image.platform,
                "deprecation_time": image.deprecation_time,
                "tags": image.tags,
                "tags_all": image.tags,
            });
            results.push(ExtractedResource {
                name: image.image_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ami_copy
// ---------------------------------------------------------------------------

pub struct AwsAmiCopyConverter {
    service: Arc<Ec2Service>,
}

impl AwsAmiCopyConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAmiCopyConverter {
    fn resource_type(&self) -> &str {
        "aws_ami_copy"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Copies are indistinguishable from aws_ami in the view; aws_ami's
        // extract emits them. Skip here to avoid duplication.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsAmiCopyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::AmiCopyTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ami_copy", e))?;

        let image_id = model
            .id
            .unwrap_or_else(|| format!("ami-{}", &uuid::Uuid::new_v4().to_string()[..17]));

        let image_view = ImageView {
            image_id: image_id.clone(),
            name: model.name,
            description: model.description.unwrap_or_default(),
            state: "available".to_string(),
            owner_id: ctx.default_account_id.clone(),
            architecture: "x86_64".to_string(),
            image_type: "machine".to_string(),
            platform: None,
            virtualization_type: "hvm".to_string(),
            root_device_type: "ebs".to_string(),
            root_device_name: "/dev/xvda".to_string(),
            public: false,
            tags: extract_tags(attrs),
            source_instance_id: None,
            source_instance_type: String::new(),
            launch_permissions: vec![],
            recycle_bin_state: None,
            deprecation_time: model.deprecation_time,
            recycle_bin_enter_time: None,
            product_codes: vec![],
            fast_launch_state: None,
            deregistration_protection: None,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.images.insert(image_id, image_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_ami_from_instance
// ---------------------------------------------------------------------------

pub struct AwsAmiFromInstanceConverter {
    service: Arc<Ec2Service>,
}

impl AwsAmiFromInstanceConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAmiFromInstanceConverter {
    fn resource_type(&self) -> &str {
        "aws_ami_from_instance"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_instance"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsAmiFromInstanceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::AmiFromInstanceTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ami_from_instance", e))?;

        let image_id = model
            .id
            .unwrap_or_else(|| format!("ami-{}", &uuid::Uuid::new_v4().to_string()[..17]));

        // Pull source instance properties from state so the AMI inherits
        // architecture / instance_type sensibly.
        let snap = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let source = snap.instances.get(&model.source_instance_id);

        let image_view = ImageView {
            image_id: image_id.clone(),
            name: model.name,
            description: model.description.unwrap_or_default(),
            state: "available".to_string(),
            owner_id: ctx.default_account_id.clone(),
            architecture: "x86_64".to_string(),
            image_type: "machine".to_string(),
            platform: None,
            virtualization_type: "hvm".to_string(),
            root_device_type: "ebs".to_string(),
            root_device_name: "/dev/xvda".to_string(),
            public: false,
            tags: extract_tags(attrs),
            source_instance_id: Some(model.source_instance_id.clone()),
            source_instance_type: source.map(|i| i.instance_type.clone()).unwrap_or_default(),
            launch_permissions: vec![],
            recycle_bin_state: None,
            deprecation_time: model.deprecation_time,
            recycle_bin_enter_time: None,
            product_codes: vec![],
            fast_launch_state: None,
            deregistration_protection: None,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.images.insert(image_id, image_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for image in view.images.values() {
            let Some(src) = image.source_instance_id.as_ref() else {
                continue;
            };
            let arn = format!(
                "arn:aws:ec2:{}::image/{}",
                ctx.default_region, image.image_id
            );
            let attrs = serde_json::json!({
                "id": image.image_id,
                "arn": arn,
                "name": image.name,
                "description": image.description,
                "source_instance_id": src,
                "deprecation_time": image.deprecation_time,
                "tags": image.tags,
                "tags_all": image.tags,
            });
            results.push(ExtractedResource {
                name: image.image_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ami_launch_permission
// ---------------------------------------------------------------------------

pub struct AwsAmiLaunchPermissionConverter {
    service: Arc<Ec2Service>,
}

impl AwsAmiLaunchPermissionConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAmiLaunchPermissionConverter {
    fn resource_type(&self) -> &str {
        "aws_ami_launch_permission"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ami", "aws_ami_copy", "aws_ami_from_instance"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsAmiLaunchPermissionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::AmiLaunchPermissionTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ami_launch_permission", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(image) = view.images.get_mut(&model.image_id) {
            let perm = LaunchPermissionView {
                user_id: model.account_id.clone(),
                group: model.group.clone(),
            };
            let already = image
                .launch_permissions
                .iter()
                .any(|p| p.user_id == perm.user_id && p.group == perm.group);
            if !already {
                image.launch_permissions.push(perm);
            }
        } else {
            warnings.push(format!(
                "image '{}' not found in state; launch permission skipped",
                model.image_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for image in view.images.values() {
            for perm in &image.launch_permissions {
                // Terraform-style composite id: "<account_or_group>-<image_id>"
                let key = perm
                    .user_id
                    .clone()
                    .or_else(|| perm.group.clone())
                    .unwrap_or_default();
                let id = format!("{}-{}", key, image.image_id);
                let attrs = serde_json::json!({
                    "id": id,
                    "image_id": image.image_id,
                    "account_id": perm.user_id,
                    "group": perm.group,
                });
                results.push(ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_endpoint
// ---------------------------------------------------------------------------

pub struct AwsVpcEndpointConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcEndpointConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_endpoint"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcEndpointTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpc_endpoint", e))?;

        let endpoint_id = model.id.unwrap_or_else(|| random_short_id("vpce"));
        let tags = extract_tags(attrs);

        let subnet_ids = parse_string_array(attrs, "subnet_ids");
        let route_table_ids = parse_string_array(attrs, "route_table_ids");
        let security_group_ids = parse_string_array(attrs, "security_group_ids");

        let endpoint = VpcEndpointView {
            endpoint_id: endpoint_id.clone(),
            vpc_id: model.vpc_id,
            service_name: model.service_name,
            endpoint_type: model
                .vpc_endpoint_type
                .unwrap_or_else(|| "Gateway".to_string()),
            state: model.state.unwrap_or_else(|| "available".to_string()),
            policy_document: model.policy,
            route_table_ids,
            subnet_ids,
            security_group_ids,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.vpc_endpoints.insert(endpoint_id, endpoint);
        let mut warnings = vec![];
        if model.private_dns_enabled {
            warnings.push(
                "aws_vpc_endpoint: private_dns_enabled not modelled in VpcEndpointView state; \
                 field ignored"
                    .to_string(),
            );
        }
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for ep in view.vpc_endpoints.values() {
            let arn = format!(
                "arn:aws:ec2:{}:{}:vpc-endpoint/{}",
                ctx.default_region, ctx.default_account_id, ep.endpoint_id
            );
            let attrs = serde_json::json!({
                "id": ep.endpoint_id,
                "arn": arn,
                "vpc_id": ep.vpc_id,
                "service_name": ep.service_name,
                "vpc_endpoint_type": ep.endpoint_type,
                "state": ep.state,
                "policy": ep.policy_document,
                "route_table_ids": ep.route_table_ids,
                "subnet_ids": ep.subnet_ids,
                "security_group_ids": ep.security_group_ids,
                "private_dns_enabled": false,
                "auto_accept": false,
                "owner_id": ctx.default_account_id,
                "tags": ep.tags,
                "tags_all": ep.tags,
            });
            results.push(ExtractedResource {
                name: ep.endpoint_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_endpoint_service
// ---------------------------------------------------------------------------

pub struct AwsVpcEndpointServiceConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcEndpointServiceConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcEndpointServiceConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_endpoint_service"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcEndpointServiceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcEndpointServiceTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpc_endpoint_service", e))?;

        let service_id = model.id.unwrap_or_else(|| random_short_id("vpce-svc"));
        let nlb_arns = parse_string_array(attrs, "network_load_balancer_arns");
        let gwlb_arns = parse_string_array(attrs, "gateway_load_balancer_arns");
        let allowed_principals = parse_string_array(attrs, "allowed_principals");
        let tags = extract_tags(attrs);

        let service_name = format!("com.amazonaws.vpce.{}.{}", region, service_id);

        let svc = VpcEndpointServiceConfigView {
            service_id: service_id.clone(),
            service_name,
            service_type: model
                .service_type
                .unwrap_or_else(|| "Interface".to_string()),
            acceptance_required: model.acceptance_required,
            state: model.state.unwrap_or_else(|| "Available".to_string()),
            network_load_balancer_arns: nlb_arns,
            gateway_load_balancer_arns: gwlb_arns,
            allowed_principals,
            tags,
            payer_responsibility: model.payer_responsibility,
            private_dns_state: model
                .private_dns_name
                .as_ref()
                .map(|_| "pendingVerification".to_string()),
        };

        let mut state_view = minimal_ec2_state_view();
        state_view
            .vpc_endpoint_service_configs
            .insert(service_id, svc);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for svc in view.vpc_endpoint_service_configs.values() {
            let arn = format!(
                "arn:aws:ec2:{}:{}:vpc-endpoint-service/{}",
                ctx.default_region, ctx.default_account_id, svc.service_id
            );
            let attrs = serde_json::json!({
                "id": svc.service_id,
                "arn": arn,
                "service_name": svc.service_name,
                "service_type": svc.service_type,
                "state": svc.state,
                "acceptance_required": svc.acceptance_required,
                "network_load_balancer_arns": svc.network_load_balancer_arns,
                "gateway_load_balancer_arns": svc.gateway_load_balancer_arns,
                "allowed_principals": svc.allowed_principals,
                "payer_responsibility": svc.payer_responsibility,
                "tags": svc.tags,
                "tags_all": svc.tags,
            });
            results.push(ExtractedResource {
                name: svc.service_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_endpoint_service_allowed_principal
// ---------------------------------------------------------------------------

pub struct AwsVpcEndpointServiceAllowedPrincipalConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcEndpointServiceAllowedPrincipalConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcEndpointServiceAllowedPrincipalConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_endpoint_service_allowed_principal"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_endpoint_service"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Allowed principals are emitted as part of aws_vpc_endpoint_service.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcEndpointServiceAllowedPrincipalConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcEndpointServiceAllowedPrincipalTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_endpoint_service_allowed_principal", e)
            })?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(svc) = view
            .vpc_endpoint_service_configs
            .get_mut(&model.vpc_endpoint_service_id)
        {
            if !svc.allowed_principals.contains(&model.principal_arn) {
                svc.allowed_principals.push(model.principal_arn);
            }
        } else {
            warnings.push(format!(
                "vpc endpoint service '{}' not found in state; allowed principal skipped",
                model.vpc_endpoint_service_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_endpoint_service_private_dns_verification
// ---------------------------------------------------------------------------

pub struct AwsVpcEndpointServicePrivateDnsVerificationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcEndpointServicePrivateDnsVerificationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcEndpointServicePrivateDnsVerificationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_endpoint_service_private_dns_verification"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_endpoint_service"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Verification state is part of the service config; no standalone extract.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcEndpointServicePrivateDnsVerificationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcEndpointServicePrivateDnsVerificationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_endpoint_service_private_dns_verification", e)
            })?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(svc) = view.vpc_endpoint_service_configs.get_mut(&model.service_id) {
            svc.private_dns_state = Some("verified".to_string());
        } else {
            warnings.push(format!(
                "vpc endpoint service '{}' not found in state; private DNS verification skipped",
                model.service_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_endpoint_subnet_association
// ---------------------------------------------------------------------------

pub struct AwsVpcEndpointSubnetAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcEndpointSubnetAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcEndpointSubnetAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_endpoint_subnet_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_endpoint", "aws_subnet"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Subnet associations are emitted as part of aws_vpc_endpoint.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcEndpointSubnetAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcEndpointSubnetAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_endpoint_subnet_association", e)
            })?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(ep) = view.vpc_endpoints.get_mut(&model.vpc_endpoint_id) {
            if !ep.subnet_ids.contains(&model.subnet_id) {
                ep.subnet_ids.push(model.subnet_id);
            }
        } else {
            warnings.push(format!(
                "vpc endpoint '{}' not found in state; subnet association skipped",
                model.vpc_endpoint_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_endpoint_route_table_association
// ---------------------------------------------------------------------------

pub struct AwsVpcEndpointRouteTableAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcEndpointRouteTableAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcEndpointRouteTableAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_endpoint_route_table_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_endpoint", "aws_route_table"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Route table associations are emitted as part of aws_vpc_endpoint.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcEndpointRouteTableAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcEndpointRouteTableAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_endpoint_route_table_association", e)
            })?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(ep) = view.vpc_endpoints.get_mut(&model.vpc_endpoint_id) {
            if !ep.route_table_ids.contains(&model.route_table_id) {
                ep.route_table_ids.push(model.route_table_id);
            }
        } else {
            warnings.push(format!(
                "vpc endpoint '{}' not found in state; route table association skipped",
                model.vpc_endpoint_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_endpoint_security_group_association
// ---------------------------------------------------------------------------

pub struct AwsVpcEndpointSecurityGroupAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcEndpointSecurityGroupAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcEndpointSecurityGroupAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_endpoint_security_group_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_endpoint", "aws_security_group"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Security group associations are emitted as part of aws_vpc_endpoint.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcEndpointSecurityGroupAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcEndpointSecurityGroupAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_endpoint_security_group_association", e)
            })?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(ep) = view.vpc_endpoints.get_mut(&model.vpc_endpoint_id) {
            if !ep.security_group_ids.contains(&model.security_group_id) {
                ep.security_group_ids.push(model.security_group_id);
            }
        } else {
            warnings.push(format!(
                "vpc endpoint '{}' not found in state; sg association skipped",
                model.vpc_endpoint_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_endpoint_connection_accepter
// ---------------------------------------------------------------------------

pub struct AwsVpcEndpointConnectionAccepterConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcEndpointConnectionAccepterConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcEndpointConnectionAccepterConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_endpoint_connection_accepter"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_endpoint", "aws_vpc_endpoint_service"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcEndpointConnectionAccepterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcEndpointConnectionAccepterTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_endpoint_connection_accepter", e)
            })?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        let target_state = model
            .vpc_endpoint_state
            .unwrap_or_else(|| "available".to_string());

        let existing = view.vpc_endpoint_connections.iter_mut().find(|c| {
            c.service_id == model.vpc_endpoint_service_id
                && c.vpc_endpoint_id == model.vpc_endpoint_id
        });
        if let Some(conn) = existing {
            conn.vpc_endpoint_state = target_state.clone();
        } else {
            view.vpc_endpoint_connections
                .push(VpcEndpointConnectionView {
                    service_id: model.vpc_endpoint_service_id.clone(),
                    vpc_endpoint_id: model.vpc_endpoint_id.clone(),
                    vpc_endpoint_owner: ctx.default_account_id.clone(),
                    vpc_endpoint_state: target_state,
                    creation_timestamp: String::new(),
                });
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for conn in &view.vpc_endpoint_connections {
            let id = format!("{}_{}", conn.service_id, conn.vpc_endpoint_id);
            let attrs = serde_json::json!({
                "id": id,
                "vpc_endpoint_service_id": conn.service_id,
                "vpc_endpoint_id": conn.vpc_endpoint_id,
                "vpc_endpoint_state": conn.vpc_endpoint_state,
            });
            results.push(ExtractedResource {
                name: id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_endpoint_connection_notification
// ---------------------------------------------------------------------------

pub struct AwsVpcEndpointConnectionNotificationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcEndpointConnectionNotificationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcEndpointConnectionNotificationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_endpoint_connection_notification"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcEndpointConnectionNotificationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcEndpointConnectionNotificationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_endpoint_connection_notification", e)
            })?;

        let notif_id = model.id.unwrap_or_else(|| random_short_id("vpce-cn"));
        let events = parse_string_array(attrs, "connection_events");

        let notif = VpcEndpointConnectionNotificationView {
            connection_notification_id: notif_id.clone(),
            connection_notification_arn: model.connection_notification_arn,
            connection_events: events,
            connection_notification_state: model.state.unwrap_or_else(|| "Enabled".to_string()),
            connection_notification_type: model
                .notification_type
                .unwrap_or_else(|| "Topic".to_string()),
            service_id: model.vpc_endpoint_service_id,
            vpc_endpoint_id: model.vpc_endpoint_id,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view
            .vpc_endpoint_connection_notifications
            .insert(notif_id, notif);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for notif in view.vpc_endpoint_connection_notifications.values() {
            let attrs = serde_json::json!({
                "id": notif.connection_notification_id,
                "connection_notification_arn": notif.connection_notification_arn,
                "connection_events": notif.connection_events,
                "state": notif.connection_notification_state,
                "notification_type": notif.connection_notification_type,
                "vpc_endpoint_service_id": notif.service_id,
                "vpc_endpoint_id": notif.vpc_endpoint_id,
            });
            results.push(ExtractedResource {
                name: notif.connection_notification_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_endpoint_policy
// ---------------------------------------------------------------------------

pub struct AwsVpcEndpointPolicyConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcEndpointPolicyConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcEndpointPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_endpoint_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_endpoint"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Policy is emitted as part of aws_vpc_endpoint.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcEndpointPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcEndpointPolicyTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpc_endpoint_policy", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(ep) = view.vpc_endpoints.get_mut(&model.vpc_endpoint_id) {
            ep.policy_document = model.policy;
        } else {
            warnings.push(format!(
                "vpc endpoint '{}' not found in state; policy override skipped",
                model.vpc_endpoint_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_endpoint_private_dns
// ---------------------------------------------------------------------------

pub struct AwsVpcEndpointPrivateDnsConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcEndpointPrivateDnsConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcEndpointPrivateDnsConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_endpoint_private_dns"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_endpoint"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // private_dns_enabled is not modelled in the VpcEndpointView; nothing
        // to emit here.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcEndpointPrivateDnsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcEndpointPrivateDnsTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpc_endpoint_private_dns", e))?;

        let view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if !view.vpc_endpoints.contains_key(&model.vpc_endpoint_id) {
            warnings.push(format!(
                "vpc endpoint '{}' not found in state; private dns toggle ignored",
                model.vpc_endpoint_id
            ));
        } else {
            // VpcEndpointView has no private_dns_enabled field; track only via
            // a warning so the converter is still observably wired.
            let _ = model.private_dns_enabled;
            warnings.push(
                "aws_vpc_endpoint_private_dns: private_dns_enabled not modelled in \
                 VpcEndpointView state; toggle ignored"
                    .to_string(),
            );
        }

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_vpn_connection
// ---------------------------------------------------------------------------

pub struct AwsVpnConnectionConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpnConnectionConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpnConnectionConverter {
    fn resource_type(&self) -> &str {
        "aws_vpn_connection"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_customer_gateway", "aws_vpn_gateway"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpnConnectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpnConnectionTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpn_connection", e))?;

        let vpn_id = model.id.unwrap_or_else(|| random_short_id("vpn"));
        let tags = extract_tags(attrs);

        let options = VpnConnectionOptionsView {
            local_ipv4_network_cidr: model.local_ipv4_network_cidr,
            local_ipv6_network_cidr: model.local_ipv6_network_cidr,
            remote_ipv4_network_cidr: model.remote_ipv4_network_cidr,
            remote_ipv6_network_cidr: model.remote_ipv6_network_cidr,
            tunnel_inside_ip_version: model.tunnel_inside_ip_version,
            static_routes_only: Some(model.static_routes_only),
            tunnel_options: vec![],
        };

        let conn = VpnConnectionView {
            vpn_connection_id: vpn_id.clone(),
            vpn_gateway_id: model.vpn_gateway_id.unwrap_or_default(),
            customer_gateway_id: model.customer_gateway_id,
            transit_gateway_id: model.transit_gateway_id,
            connection_type: model.connection_type,
            state: "available".to_string(),
            tags,
            routes: vec![],
            options: Some(options),
            tunnel_replacement_status: None,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.vpn_connections.insert(vpn_id, conn);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for conn in view.vpn_connections.values() {
            let arn = format!(
                "arn:aws:ec2:{}:{}:vpn-connection/{}",
                ctx.default_region, ctx.default_account_id, conn.vpn_connection_id
            );
            let opts = conn.options.as_ref();
            let static_routes_only = opts.and_then(|o| o.static_routes_only).unwrap_or(false);
            let attrs = serde_json::json!({
                "id": conn.vpn_connection_id,
                "arn": arn,
                "type": conn.connection_type,
                "customer_gateway_id": conn.customer_gateway_id,
                "vpn_gateway_id": conn.vpn_gateway_id,
                "transit_gateway_id": conn.transit_gateway_id,
                "state": conn.state,
                "static_routes_only": static_routes_only,
                "local_ipv4_network_cidr": opts.and_then(|o| o.local_ipv4_network_cidr.clone()),
                "local_ipv6_network_cidr": opts.and_then(|o| o.local_ipv6_network_cidr.clone()),
                "remote_ipv4_network_cidr": opts.and_then(|o| o.remote_ipv4_network_cidr.clone()),
                "remote_ipv6_network_cidr": opts.and_then(|o| o.remote_ipv6_network_cidr.clone()),
                "tunnel_inside_ip_version": opts.and_then(|o| o.tunnel_inside_ip_version.clone()),
                "tags": conn.tags,
                "tags_all": conn.tags,
            });
            results.push(ExtractedResource {
                name: conn.vpn_connection_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpn_connection_route
// ---------------------------------------------------------------------------

pub struct AwsVpnConnectionRouteConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpnConnectionRouteConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpnConnectionRouteConverter {
    fn resource_type(&self) -> &str {
        "aws_vpn_connection_route"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpn_connection"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpnConnectionRouteConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpnConnectionRouteTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpn_connection_route", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(conn) = view.vpn_connections.get_mut(&model.vpn_connection_id) {
            if !conn
                .routes
                .iter()
                .any(|r| r.destination_cidr_block == model.destination_cidr_block)
            {
                conn.routes.push(VpnStaticRouteView {
                    destination_cidr_block: model.destination_cidr_block,
                    source: "Static".to_string(),
                    state: "available".to_string(),
                });
            }
        } else {
            warnings.push(format!(
                "vpn connection '{}' not found in state; route skipped",
                model.vpn_connection_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for conn in view.vpn_connections.values() {
            for route in &conn.routes {
                let id = format!(
                    "{}:{}",
                    conn.vpn_connection_id, route.destination_cidr_block
                );
                let attrs = serde_json::json!({
                    "id": id,
                    "vpn_connection_id": conn.vpn_connection_id,
                    "destination_cidr_block": route.destination_cidr_block,
                });
                results.push(ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpn_gateway
// ---------------------------------------------------------------------------

pub struct AwsVpnGatewayConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpnGatewayConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpnGatewayConverter {
    fn resource_type(&self) -> &str {
        "aws_vpn_gateway"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpnGatewayConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpnGatewayTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpn_gateway", e))?;

        let vgw_id = model.id.unwrap_or_else(|| random_short_id("vgw"));
        let tags = extract_tags(attrs);

        let mut vpc_attachments = vec![];
        if let Some(vpc) = model.vpc_id.as_ref().filter(|s| !s.is_empty()) {
            vpc_attachments.push(VgwVpcAttachmentView {
                vpc_id: vpc.clone(),
                state: "attached".to_string(),
            });
        }

        let amazon_side_asn = if model.amazon_side_asn != 0 {
            Some(model.amazon_side_asn)
        } else {
            None
        };

        let vgw = VpnGatewayView {
            vpn_gateway_id: vgw_id.clone(),
            gateway_type: "ipsec.1".to_string(),
            state: "available".to_string(),
            amazon_side_asn,
            vpc_attachments,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.vpn_gateways.insert(vgw_id, vgw);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for vgw in view.vpn_gateways.values() {
            let arn = format!(
                "arn:aws:ec2:{}:{}:vpn-gateway/{}",
                ctx.default_region, ctx.default_account_id, vgw.vpn_gateway_id
            );
            let vpc_id = vgw
                .vpc_attachments
                .iter()
                .find(|a| a.state == "attached")
                .map(|a| a.vpc_id.clone());
            let attrs = serde_json::json!({
                "id": vgw.vpn_gateway_id,
                "arn": arn,
                "vpc_id": vpc_id,
                "amazon_side_asn": vgw.amazon_side_asn,
                "tags": vgw.tags,
                "tags_all": vgw.tags,
            });
            results.push(ExtractedResource {
                name: vgw.vpn_gateway_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpn_gateway_route_propagation
// ---------------------------------------------------------------------------

pub struct AwsVpnGatewayRoutePropagationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpnGatewayRoutePropagationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpnGatewayRoutePropagationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpn_gateway_route_propagation"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpn_gateway", "aws_route_table"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // RouteTableView has no propagating_vgws field; nothing to emit.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpnGatewayRoutePropagationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpnGatewayRoutePropagationTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_vpn_gateway_route_propagation", e))?;

        let view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if !view.route_tables.contains_key(&model.route_table_id) {
            warnings.push(format!(
                "route table '{}' not found in state; vgw route propagation skipped",
                model.route_table_id
            ));
        } else if !view.vpn_gateways.contains_key(&model.vpn_gateway_id) {
            warnings.push(format!(
                "vpn gateway '{}' not found in state; vgw route propagation skipped",
                model.vpn_gateway_id
            ));
        } else {
            warnings.push(
                "aws_vpn_gateway_route_propagation: propagating_vgws not modelled in \
                 RouteTableView state; propagation ignored"
                    .to_string(),
            );
        }

        Ok(ConversionResult { region, warnings })
    }
}

// ===========================================================================
// Wave 4: Transit Gateway family
// ===========================================================================

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TransitGatewayConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_transit_gateway", e))?;

        let id = model.id.unwrap_or_else(|| random_short_id("tgw"));
        let description = model.description.unwrap_or_default();
        let dns_support = model.dns_support.unwrap_or_else(|| "enable".to_string());
        let vpn_ecmp_support = model
            .vpn_ecmp_support
            .unwrap_or_else(|| "enable".to_string());
        let multicast_support = model
            .multicast_support
            .unwrap_or_else(|| "disable".to_string());
        let amazon_side_asn = if model.amazon_side_asn != 0 {
            model.amazon_side_asn
        } else {
            64512
        };
        let tags = extract_tags(attrs);

        let view = TransitGatewayView {
            transit_gateway_id: id.clone(),
            state: "available".to_string(),
            amazon_side_asn,
            description,
            dns_support,
            vpn_ecmp_support,
            multicast_support,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.transit_gateways.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for t in view.transit_gateways.values() {
            let arn = format!(
                "arn:aws:ec2:{}:{}:transit-gateway/{}",
                ctx.default_region, ctx.default_account_id, t.transit_gateway_id
            );
            let attrs = serde_json::json!({
                "id": t.transit_gateway_id,
                "arn": arn,
                "description": t.description,
                "amazon_side_asn": t.amazon_side_asn,
                "dns_support": t.dns_support,
                "vpn_ecmp_support": t.vpn_ecmp_support,
                "multicast_support": t.multicast_support,
                "owner_id": ctx.default_account_id,
                "tags": t.tags,
                "tags_all": t.tags,
            });
            results.push(ExtractedResource {
                name: t.transit_gateway_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_route_table
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayRouteTableConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayRouteTableConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayRouteTableConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_route_table"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_transit_gateway"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TransitGatewayRouteTableConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayRouteTableTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_transit_gateway_route_table", e))?;

        let id = model.id.unwrap_or_else(|| random_short_id("tgw-rtb"));
        let tags = extract_tags(attrs);

        let view = TgwRouteTableView {
            route_table_id: id.clone(),
            transit_gateway_id: model.transit_gateway_id,
            state: "available".to_string(),
            default_association_route_table: model.default_association_route_table,
            default_propagation_route_table: model.default_propagation_route_table,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.tgw_route_tables.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for rt in view.tgw_route_tables.values() {
            let arn = format!(
                "arn:aws:ec2:{}:{}:transit-gateway-route-table/{}",
                ctx.default_region, ctx.default_account_id, rt.route_table_id
            );
            let attrs = serde_json::json!({
                "id": rt.route_table_id,
                "arn": arn,
                "transit_gateway_id": rt.transit_gateway_id,
                "default_association_route_table": rt.default_association_route_table,
                "default_propagation_route_table": rt.default_propagation_route_table,
                "tags": rt.tags,
                "tags_all": rt.tags,
            });
            results.push(ExtractedResource {
                name: rt.route_table_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_route
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayRouteConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayRouteConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayRouteConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_route"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_transit_gateway_route_table"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TransitGatewayRouteConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayRouteTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_transit_gateway_route", e))?;

        let mut warnings = vec![];
        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        if !view
            .tgw_route_tables
            .contains_key(&model.transit_gateway_route_table_id)
        {
            warnings.push(format!(
                "transit gateway route table '{}' not found in state; tgw route skipped",
                model.transit_gateway_route_table_id
            ));
        } else {
            let entry = view
                .tgw_routes
                .entry(model.transit_gateway_route_table_id.clone())
                .or_default();
            let state = if model.blackhole {
                "blackhole".to_string()
            } else {
                "active".to_string()
            };
            entry.push(TgwRouteView {
                destination_cidr_block: model.destination_cidr_block,
                route_type: "static".to_string(),
                state,
                attachment_id: model.transit_gateway_attachment_id,
            });
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for (rtb_id, routes) in view.tgw_routes.iter() {
            for r in routes {
                let name = format!("{}_{}", rtb_id, r.destination_cidr_block);
                let attrs = serde_json::json!({
                    "id": name,
                    "transit_gateway_route_table_id": rtb_id,
                    "destination_cidr_block": r.destination_cidr_block,
                    "transit_gateway_attachment_id": r.attachment_id,
                    "blackhole": r.state == "blackhole",
                });
                results.push(ExtractedResource {
                    name,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_route_table_association
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayRouteTableAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayRouteTableAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayRouteTableAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_route_table_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_ec2_transit_gateway_route_table",
            "aws_ec2_transit_gateway_vpc_attachment",
            "aws_ec2_transit_gateway_peering_attachment",
        ]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // TgwRouteTableView has no `associations` field; nothing to emit.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEc2TransitGatewayRouteTableAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayRouteTableAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_transit_gateway_route_table_association", e)
            })?;

        let view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if !view
            .tgw_route_tables
            .contains_key(&model.transit_gateway_route_table_id)
        {
            warnings.push(format!(
                "transit gateway route table '{}' not found in state; association skipped",
                model.transit_gateway_route_table_id
            ));
        } else {
            warnings.push(
                "aws_ec2_transit_gateway_route_table_association: associations not modelled \
                 on TgwRouteTableView; association recorded only in tf state"
                    .to_string(),
            );
        }

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_route_table_propagation
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayRouteTablePropagationConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayRouteTablePropagationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayRouteTablePropagationConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_route_table_propagation"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_ec2_transit_gateway_route_table",
            "aws_ec2_transit_gateway_vpc_attachment",
            "aws_ec2_transit_gateway_peering_attachment",
        ]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // TgwRouteTableView has no `propagations` field; nothing to emit.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEc2TransitGatewayRouteTablePropagationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayRouteTablePropagationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_transit_gateway_route_table_propagation", e)
            })?;

        let view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if !view
            .tgw_route_tables
            .contains_key(&model.transit_gateway_route_table_id)
        {
            warnings.push(format!(
                "transit gateway route table '{}' not found in state; propagation skipped",
                model.transit_gateway_route_table_id
            ));
        } else {
            warnings.push(
                "aws_ec2_transit_gateway_route_table_propagation: propagations not modelled \
                 on TgwRouteTableView; propagation recorded only in tf state"
                    .to_string(),
            );
        }

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_default_route_table_association
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayDefaultRouteTableAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayDefaultRouteTableAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayDefaultRouteTableAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_default_route_table_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_ec2_transit_gateway",
            "aws_ec2_transit_gateway_route_table",
        ]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Modifier resource; emitted via aws_ec2_transit_gateway.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEc2TransitGatewayDefaultRouteTableAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayDefaultRouteTableAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error(
                    "aws_ec2_transit_gateway_default_route_table_association",
                    e,
                )
            })?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if !view
            .transit_gateways
            .contains_key(&model.transit_gateway_id)
        {
            warnings.push(format!(
                "transit gateway '{}' not found in state; default association table skipped",
                model.transit_gateway_id
            ));
        } else if !view
            .tgw_route_tables
            .contains_key(&model.transit_gateway_route_table_id)
        {
            warnings.push(format!(
                "transit gateway route table '{}' not found in state; default association skipped",
                model.transit_gateway_route_table_id
            ));
        } else {
            // Flip the boolean flag on each TGW route table for this TGW.
            for rt in view.tgw_route_tables.values_mut() {
                if rt.transit_gateway_id == model.transit_gateway_id {
                    rt.default_association_route_table =
                        rt.route_table_id == model.transit_gateway_route_table_id;
                }
            }
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_default_route_table_propagation
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayDefaultRouteTablePropagationConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayDefaultRouteTablePropagationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayDefaultRouteTablePropagationConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_default_route_table_propagation"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_ec2_transit_gateway",
            "aws_ec2_transit_gateway_route_table",
        ]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Modifier resource; emitted via aws_ec2_transit_gateway.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEc2TransitGatewayDefaultRouteTablePropagationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayDefaultRouteTablePropagationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error(
                    "aws_ec2_transit_gateway_default_route_table_propagation",
                    e,
                )
            })?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if !view
            .transit_gateways
            .contains_key(&model.transit_gateway_id)
        {
            warnings.push(format!(
                "transit gateway '{}' not found in state; default propagation table skipped",
                model.transit_gateway_id
            ));
        } else if !view
            .tgw_route_tables
            .contains_key(&model.transit_gateway_route_table_id)
        {
            warnings.push(format!(
                "transit gateway route table '{}' not found in state; default propagation skipped",
                model.transit_gateway_route_table_id
            ));
        } else {
            for rt in view.tgw_route_tables.values_mut() {
                if rt.transit_gateway_id == model.transit_gateway_id {
                    rt.default_propagation_route_table =
                        rt.route_table_id == model.transit_gateway_route_table_id;
                }
            }
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_vpc_attachment
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayVpcAttachmentConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayVpcAttachmentConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayVpcAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_vpc_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_transit_gateway", "aws_vpc", "aws_subnet"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TransitGatewayVpcAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayVpcAttachmentTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_transit_gateway_vpc_attachment", e)
            })?;

        let id = model.id.unwrap_or_else(|| random_short_id("tgw-attach"));
        let tags = extract_tags(attrs);

        // Pull subnet_ids list directly from attrs since spec only handles scalars.
        let subnet_ids: Vec<String> = attrs
            .get("subnet_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let view = TgwVpcAttachmentView {
            attachment_id: id.clone(),
            transit_gateway_id: model.transit_gateway_id,
            vpc_id: model.vpc_id,
            subnet_ids,
            state: "available".to_string(),
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.tgw_vpc_attachments.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for a in view.tgw_vpc_attachments.values() {
            let attrs = serde_json::json!({
                "id": a.attachment_id,
                "transit_gateway_id": a.transit_gateway_id,
                "vpc_id": a.vpc_id,
                "subnet_ids": a.subnet_ids,
                "vpc_owner_id": ctx.default_account_id,
                "tags": a.tags,
                "tags_all": a.tags,
            });
            results.push(ExtractedResource {
                name: a.attachment_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_vpc_attachment_accepter
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayVpcAttachmentAccepterConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayVpcAttachmentAccepterConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayVpcAttachmentAccepterConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_vpc_attachment_accepter"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_transit_gateway_vpc_attachment"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Modifier resource; the underlying attachment is emitted by aws_ec2_transit_gateway_vpc_attachment.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEc2TransitGatewayVpcAttachmentAccepterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayVpcAttachmentAccepterTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_transit_gateway_vpc_attachment_accepter", e)
            })?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(att) = view
            .tgw_vpc_attachments
            .get_mut(&model.transit_gateway_attachment_id)
        {
            att.state = "available".to_string();
        } else {
            warnings.push(format!(
                "tgw vpc attachment '{}' not found in state; accepter skipped",
                model.transit_gateway_attachment_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_peering_attachment
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayPeeringAttachmentConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayPeeringAttachmentConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayPeeringAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_peering_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_transit_gateway"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TransitGatewayPeeringAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayPeeringAttachmentTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_transit_gateway_peering_attachment", e)
            })?;

        let id = model.id.unwrap_or_else(|| random_short_id("tgw-attach"));
        let peer_account_id = model
            .peer_account_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let peer_region = model.peer_region.unwrap_or_else(|| region.clone());
        let tags = extract_tags(attrs);

        let view = TgwPeeringAttachmentView {
            attachment_id: id.clone(),
            transit_gateway_id: model.transit_gateway_id,
            peer_transit_gateway_id: model.peer_transit_gateway_id,
            peer_account_id,
            peer_region,
            state: "pendingAcceptance".to_string(),
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.tgw_peering_attachments.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for a in view.tgw_peering_attachments.values() {
            let attrs = serde_json::json!({
                "id": a.attachment_id,
                "transit_gateway_id": a.transit_gateway_id,
                "peer_transit_gateway_id": a.peer_transit_gateway_id,
                "peer_account_id": a.peer_account_id,
                "peer_region": a.peer_region,
                "state": a.state,
                "tags": a.tags,
                "tags_all": a.tags,
            });
            results.push(ExtractedResource {
                name: a.attachment_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_peering_attachment_accepter
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayPeeringAttachmentAccepterConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayPeeringAttachmentAccepterConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayPeeringAttachmentAccepterConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_peering_attachment_accepter"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_transit_gateway_peering_attachment"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEc2TransitGatewayPeeringAttachmentAccepterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayPeeringAttachmentAccepterTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_transit_gateway_peering_attachment_accepter", e)
            })?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(att) = view
            .tgw_peering_attachments
            .get_mut(&model.transit_gateway_attachment_id)
        {
            att.state = "available".to_string();
        } else {
            warnings.push(format!(
                "tgw peering attachment '{}' not found in state; accepter skipped",
                model.transit_gateway_attachment_id
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_connect_peer
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayConnectPeerConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayConnectPeerConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayConnectPeerConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_connect_peer"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_transit_gateway_connect"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TransitGatewayConnectPeerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayConnectPeerTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_transit_gateway_connect_peer", e)
            })?;

        let id = model
            .id
            .unwrap_or_else(|| random_short_id("tgw-connect-peer"));
        let transit_gateway_address = model.transit_gateway_address.unwrap_or_default();
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());
        let tags = extract_tags(attrs);

        // Parse inside_cidr_blocks list directly from attrs.
        let inside_cidr_blocks: Vec<String> = attrs
            .get("inside_cidr_blocks")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let view = TransitGatewayConnectPeerView {
            transit_gateway_attachment_id: model.transit_gateway_attachment_id,
            transit_gateway_connect_peer_id: id.clone(),
            state: "available".to_string(),
            creation_time,
            transit_gateway_address,
            peer_address: model.peer_address,
            inside_cidr_blocks,
            protocol: "gre".to_string(),
            bgp_configurations: vec![],
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.tgw_connect_peers.insert(id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for p in view.tgw_connect_peers.values() {
            let arn = format!(
                "arn:aws:ec2:{}:{}:transit-gateway-connect-peer/{}",
                ctx.default_region, ctx.default_account_id, p.transit_gateway_connect_peer_id
            );
            let attrs = serde_json::json!({
                "id": p.transit_gateway_connect_peer_id,
                "arn": arn,
                "transit_gateway_attachment_id": p.transit_gateway_attachment_id,
                "peer_address": p.peer_address,
                "transit_gateway_address": p.transit_gateway_address,
                "inside_cidr_blocks": p.inside_cidr_blocks,
                "creation_time": p.creation_time,
                "tags": p.tags,
                "tags_all": p.tags,
            });
            results.push(ExtractedResource {
                name: p.transit_gateway_connect_peer_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_policy_table_association
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayPolicyTableAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayPolicyTableAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayPolicyTableAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_policy_table_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_transit_gateway_policy_table"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TransitGatewayPolicyTableAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayPolicyTableAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_transit_gateway_policy_table_association", e)
            })?;

        let view = TransitGatewayPolicyTableAssociationView {
            transit_gateway_policy_table_id: model.transit_gateway_policy_table_id,
            transit_gateway_attachment_id: model.transit_gateway_attachment_id,
            resource_id: model.resource_id.unwrap_or_default(),
            resource_type: model.resource_type.unwrap_or_else(|| "vpc".to_string()),
            state: "associated".to_string(),
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.tgw_policy_table_associations.push(view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for a in view.tgw_policy_table_associations.iter() {
            let id = format!(
                "{}_{}",
                a.transit_gateway_policy_table_id, a.transit_gateway_attachment_id
            );
            let attrs = serde_json::json!({
                "id": id,
                "transit_gateway_policy_table_id": a.transit_gateway_policy_table_id,
                "transit_gateway_attachment_id": a.transit_gateway_attachment_id,
                "resource_id": a.resource_id,
                "resource_type": a.resource_type,
            });
            results.push(ExtractedResource {
                name: id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_prefix_list_reference
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayPrefixListReferenceConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayPrefixListReferenceConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayPrefixListReferenceConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_prefix_list_reference"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_transit_gateway_route_table"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TransitGatewayPrefixListReferenceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayPrefixListReferenceTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_transit_gateway_prefix_list_reference", e)
            })?;

        let view = TransitGatewayPrefixListReferenceView {
            transit_gateway_route_table_id: model.transit_gateway_route_table_id,
            prefix_list_id: model.prefix_list_id,
            prefix_list_owner_id: model
                .prefix_list_owner_id
                .unwrap_or_else(|| ctx.default_account_id.clone()),
            state: "available".to_string(),
            blackhole: model.blackhole,
            transit_gateway_attachment_id: model.transit_gateway_attachment_id,
            resource_id: None,
            resource_type: None,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.tgw_prefix_list_references.push(view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for r in view.tgw_prefix_list_references.iter() {
            let id = format!("{}_{}", r.transit_gateway_route_table_id, r.prefix_list_id);
            let attrs = serde_json::json!({
                "id": id,
                "transit_gateway_route_table_id": r.transit_gateway_route_table_id,
                "prefix_list_id": r.prefix_list_id,
                "prefix_list_owner_id": r.prefix_list_owner_id,
                "blackhole": r.blackhole,
                "transit_gateway_attachment_id": r.transit_gateway_attachment_id,
            });
            results.push(ExtractedResource {
                name: id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_multicast_domain_association
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayMulticastDomainAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayMulticastDomainAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayMulticastDomainAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_multicast_domain_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_transit_gateway_multicast_domain"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TransitGatewayMulticastDomainAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayMulticastDomainAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error(
                    "aws_ec2_transit_gateway_multicast_domain_association",
                    e,
                )
            })?;

        // Snapshot+mutate: find an existing association with the same domain+attachment, or push fresh.
        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let subnet_assoc = MulticastSubnetAssociationView {
            subnet_id: model.subnet_id.clone(),
            state: "associated".to_string(),
        };

        let existing = view.tgw_multicast_domain_associations.iter_mut().find(|a| {
            a.transit_gateway_multicast_domain_id == model.transit_gateway_multicast_domain_id
                && a.transit_gateway_attachment_id == model.transit_gateway_attachment_id
        });
        if let Some(a) = existing {
            if !a.subnets.iter().any(|s| s.subnet_id == model.subnet_id) {
                a.subnets.push(subnet_assoc);
            }
        } else {
            view.tgw_multicast_domain_associations.push(
                TransitGatewayMulticastDomainAssociationView {
                    transit_gateway_multicast_domain_id: model.transit_gateway_multicast_domain_id,
                    transit_gateway_attachment_id: model.transit_gateway_attachment_id,
                    resource_id: String::new(),
                    resource_type: "vpc".to_string(),
                    subnets: vec![subnet_assoc],
                },
            );
        }

        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for a in view.tgw_multicast_domain_associations.iter() {
            for s in &a.subnets {
                let id = format!(
                    "{}_{}_{}",
                    a.transit_gateway_multicast_domain_id,
                    a.transit_gateway_attachment_id,
                    s.subnet_id
                );
                let attrs = serde_json::json!({
                    "id": id,
                    "transit_gateway_multicast_domain_id": a.transit_gateway_multicast_domain_id,
                    "transit_gateway_attachment_id": a.transit_gateway_attachment_id,
                    "subnet_id": s.subnet_id,
                });
                results.push(ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_multicast_group_member
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayMulticastGroupMemberConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayMulticastGroupMemberConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayMulticastGroupMemberConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_multicast_group_member"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_transit_gateway_multicast_domain"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TransitGatewayMulticastGroupMemberConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayMulticastGroupMemberTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_transit_gateway_multicast_group_member", e)
            })?;

        let view = TransitGatewayMulticastGroupMemberView {
            transit_gateway_multicast_domain_id: model.transit_gateway_multicast_domain_id,
            group_ip_address: model.group_ip_address,
            transit_gateway_attachment_id: None,
            subnet_id: None,
            resource_id: None,
            resource_type: "vpc".to_string(),
            network_interface_id: model.network_interface_id,
            member_type: "static".to_string(),
            source_type: "static".to_string(),
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.tgw_multicast_group_members.push(view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for m in view.tgw_multicast_group_members.iter() {
            let id = format!(
                "{}_{}_{}",
                m.transit_gateway_multicast_domain_id, m.group_ip_address, m.network_interface_id
            );
            let attrs = serde_json::json!({
                "id": id,
                "transit_gateway_multicast_domain_id": m.transit_gateway_multicast_domain_id,
                "group_ip_address": m.group_ip_address,
                "network_interface_id": m.network_interface_id,
            });
            results.push(ExtractedResource {
                name: id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_transit_gateway_multicast_group_source
// ---------------------------------------------------------------------------

pub struct AwsEc2TransitGatewayMulticastGroupSourceConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TransitGatewayMulticastGroupSourceConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TransitGatewayMulticastGroupSourceConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_transit_gateway_multicast_group_source"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_transit_gateway_multicast_domain"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2TransitGatewayMulticastGroupSourceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::TransitGatewayMulticastGroupSourceTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_transit_gateway_multicast_group_source", e)
            })?;

        let view = TransitGatewayMulticastGroupSourceView {
            transit_gateway_multicast_domain_id: model.transit_gateway_multicast_domain_id,
            group_ip_address: model.group_ip_address,
            transit_gateway_attachment_id: None,
            subnet_id: None,
            resource_id: None,
            resource_type: "vpc".to_string(),
            network_interface_id: model.network_interface_id,
            member_type: "static".to_string(),
            source_type: "static".to_string(),
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.tgw_multicast_group_sources.push(view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for m in view.tgw_multicast_group_sources.iter() {
            let id = format!(
                "{}_{}_{}",
                m.transit_gateway_multicast_domain_id, m.group_ip_address, m.network_interface_id
            );
            let attrs = serde_json::json!({
                "id": id,
                "transit_gateway_multicast_domain_id": m.transit_gateway_multicast_domain_id,
                "group_ip_address": m.group_ip_address,
                "network_interface_id": m.network_interface_id,
            });
            results.push(ExtractedResource {
                name: id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Wave 5 — Client VPN family
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// aws_ec2_client_vpn_endpoint
// ---------------------------------------------------------------------------

pub struct AwsEc2ClientVpnEndpointConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2ClientVpnEndpointConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2ClientVpnEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_client_vpn_endpoint"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2ClientVpnEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::ClientVpnEndpointTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_client_vpn_endpoint", e))?;

        let endpoint_id = model
            .id
            .clone()
            .unwrap_or_else(|| random_short_id("cvpn-endpoint"));
        let tags = extract_tags(attrs);

        let view = ClientVpnEndpointView {
            client_vpn_endpoint_id: endpoint_id.clone(),
            description: model.description,
            status: ClientVpnEndpointStatusView {
                code: "available".to_string(),
                message: None,
            },
            creation_time: String::new(),
            deletion_time: None,
            dns_name: model.dns_name.unwrap_or_default(),
            client_cidr_block: model.client_cidr_block,
            dns_servers: parse_string_array(attrs, "dns_servers"),
            split_tunnel: model.split_tunnel,
            vpn_protocol: "openvpn".to_string(),
            transport_protocol: model
                .transport_protocol
                .unwrap_or_else(|| "udp".to_string()),
            vpn_port: model.vpn_port as i32,
            server_certificate_arn: model.server_certificate_arn,
            authentication_options: Vec::new(),
            connection_log_options_enabled: false,
            connection_log_options_cloudwatch_log_group: None,
            connection_log_options_cloudwatch_log_stream: None,
            tags,
            security_group_ids: parse_string_array(attrs, "security_group_ids"),
            vpc_id: model.vpc_id,
            self_service_portal_url: None,
            self_service_portal: model
                .self_service_portal
                .unwrap_or_else(|| "disabled".to_string()),
            session_timeout_hours: if model.session_timeout_hours == 0 {
                24
            } else {
                model.session_timeout_hours as i32
            },
            client_login_banner_enabled: false,
            client_login_banner_text: None,
            disconnect_on_session_timeout: false,
            client_route_enforcement_enforced: false,
            client_certificate_revocation_list: None,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.client_vpn_endpoints.insert(endpoint_id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for ep in view.client_vpn_endpoints.values() {
            let attrs = serde_json::json!({
                "id": ep.client_vpn_endpoint_id,
                "client_cidr_block": ep.client_cidr_block,
                "server_certificate_arn": ep.server_certificate_arn,
                "description": ep.description,
                "dns_name": ep.dns_name,
                "split_tunnel": ep.split_tunnel,
                "transport_protocol": ep.transport_protocol,
                "vpn_port": ep.vpn_port,
                "self_service_portal": ep.self_service_portal,
                "session_timeout_hours": ep.session_timeout_hours,
                "vpc_id": ep.vpc_id,
                "tags": ep.tags,
            });
            results.push(ExtractedResource {
                name: ep.client_vpn_endpoint_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_client_vpn_authorization_rule
// ---------------------------------------------------------------------------

pub struct AwsEc2ClientVpnAuthorizationRuleConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2ClientVpnAuthorizationRuleConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2ClientVpnAuthorizationRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_client_vpn_authorization_rule"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_client_vpn_endpoint"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2ClientVpnAuthorizationRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::ClientVpnAuthorizationRuleTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_client_vpn_authorization_rule", e)
            })?;

        let rule = ClientVpnAuthorizationRuleView {
            client_vpn_endpoint_id: model.client_vpn_endpoint_id.clone(),
            group_id: model.access_group_id,
            access_all: model.authorize_all_groups,
            destination_cidr: model.target_network_cidr,
            description: model.description,
            status: ClientVpnAuthorizationRuleStatusView {
                code: "active".to_string(),
                message: None,
            },
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.client_vpn_authorization_rules.push(rule);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Authorization rules are modifiers on a client VPN endpoint; round-trip
        // via the endpoint resource.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_client_vpn_network_association
// ---------------------------------------------------------------------------

pub struct AwsEc2ClientVpnNetworkAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2ClientVpnNetworkAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2ClientVpnNetworkAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_client_vpn_network_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_client_vpn_endpoint", "aws_subnet"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2ClientVpnNetworkAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::ClientVpnNetworkAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_client_vpn_network_association", e)
            })?;

        let association_id = model
            .association_id
            .clone()
            .or_else(|| model.id.clone())
            .unwrap_or_else(|| random_short_id("cvpn-assoc"));
        let vpc_id = model.vpc_id.unwrap_or_default();
        let assoc = ClientVpnTargetNetworkAssociationView {
            association_id: association_id.clone(),
            vpc_id,
            target_network_id: model.subnet_id,
            client_vpn_endpoint_id: model.client_vpn_endpoint_id,
            security_groups: parse_string_array(attrs, "security_groups"),
            status: ClientVpnAssociationStatusView {
                code: "associated".to_string(),
                message: None,
            },
        };

        let mut state_view = minimal_ec2_state_view();
        state_view
            .client_vpn_target_network_associations
            .insert(association_id, assoc);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Modifier on the client VPN endpoint; round-trip via the endpoint.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_client_vpn_route
// ---------------------------------------------------------------------------

pub struct AwsEc2ClientVpnRouteConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2ClientVpnRouteConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2ClientVpnRouteConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_client_vpn_route"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_ec2_client_vpn_endpoint",
            "aws_ec2_client_vpn_network_association",
        ]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2ClientVpnRouteConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::ClientVpnRouteTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_client_vpn_route", e))?;

        let route = ClientVpnRouteView {
            client_vpn_endpoint_id: model.client_vpn_endpoint_id,
            destination_cidr: model.destination_cidr_block,
            target_subnet: model.target_vpc_subnet_id,
            r#type: model.r#type.unwrap_or_else(|| "Nat".to_string()),
            origin: model.origin.unwrap_or_else(|| "add-route".to_string()),
            status: ClientVpnRouteStatusView {
                code: "active".to_string(),
                message: None,
            },
            description: model.description,
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.client_vpn_routes.push(route);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Routes are modifiers on the client VPN endpoint.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// Wave 5 — Host / Fleet / Spot
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// aws_ec2_host
// ---------------------------------------------------------------------------

pub struct AwsEc2HostConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2HostConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2HostConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_host"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2HostConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::Ec2HostTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_host", e))?;

        let host_id = model
            .host_id
            .clone()
            .or_else(|| model.id.clone())
            .unwrap_or_else(|| random_short_id("h"));
        let tags = extract_tags(attrs);

        let view = DedicatedHostView {
            host_id: host_id.clone(),
            availability_zone: model.availability_zone,
            instance_type: model.instance_type,
            auto_placement: model.auto_placement.unwrap_or_else(|| "on".to_string()),
            host_recovery: model.host_recovery.unwrap_or_else(|| "off".to_string()),
            state: "available".to_string(),
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.dedicated_hosts.insert(host_id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for h in view.dedicated_hosts.values() {
            let attrs = serde_json::json!({
                "id": h.host_id,
                "host_id": h.host_id,
                "availability_zone": h.availability_zone,
                "instance_type": h.instance_type,
                "auto_placement": h.auto_placement,
                "host_recovery": h.host_recovery,
                "tags": h.tags,
            });
            results.push(ExtractedResource {
                name: h.host_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_fleet
// ---------------------------------------------------------------------------

pub struct AwsEc2FleetConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2FleetConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2FleetConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_fleet"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2FleetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::Ec2FleetTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_fleet", e))?;

        let fleet_id = model.id.clone().unwrap_or_else(|| random_short_id("fleet"));
        let tags = extract_tags(attrs);

        let view = Ec2FleetView {
            fleet_id: fleet_id.clone(),
            state: "active".to_string(),
            fleet_type: model.r#type.unwrap_or_else(|| "maintain".to_string()),
            create_time: String::new(),
            tags,
            total_target_capacity: None,
            on_demand_target_capacity: None,
            spot_target_capacity: None,
            context: model.context,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.ec2_fleets.insert(fleet_id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for f in view.ec2_fleets.values() {
            let attrs = serde_json::json!({
                "id": f.fleet_id,
                "type": f.fleet_type,
                "context": f.context,
                "tags": f.tags,
            });
            results.push(ExtractedResource {
                name: f.fleet_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_spot_fleet_request
// ---------------------------------------------------------------------------

pub struct AwsSpotFleetRequestConverter {
    service: Arc<Ec2Service>,
}

impl AwsSpotFleetRequestConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSpotFleetRequestConverter {
    fn resource_type(&self) -> &str {
        "aws_spot_fleet_request"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsSpotFleetRequestConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::SpotFleetRequestTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_spot_fleet_request", e))?;

        let request_id = model.id.clone().unwrap_or_else(|| random_short_id("sfr"));
        let tags = extract_tags(attrs);

        let view = SpotFleetRequestView {
            spot_fleet_request_id: request_id.clone(),
            spot_fleet_request_state: "active".to_string(),
            target_capacity: model.target_capacity as i32,
            iam_fleet_role: model.iam_fleet_role,
            create_time: String::new(),
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.spot_fleet_requests.insert(request_id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for r in view.spot_fleet_requests.values() {
            let attrs = serde_json::json!({
                "id": r.spot_fleet_request_id,
                "iam_fleet_role": r.iam_fleet_role,
                "target_capacity": r.target_capacity,
                "tags": r.tags,
            });
            results.push(ExtractedResource {
                name: r.spot_fleet_request_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_spot_instance_request
// ---------------------------------------------------------------------------

pub struct AwsSpotInstanceRequestConverter {
    service: Arc<Ec2Service>,
}

impl AwsSpotInstanceRequestConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSpotInstanceRequestConverter {
    fn resource_type(&self) -> &str {
        "aws_spot_instance_request"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsSpotInstanceRequestConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::SpotInstanceRequestTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_spot_instance_request", e))?;

        let request_id = model.id.clone().unwrap_or_else(|| random_short_id("sir"));
        let tags = extract_tags(attrs);

        let view = SpotInstanceRequestView {
            spot_instance_request_id: request_id.clone(),
            spot_price: model.spot_price.unwrap_or_default(),
            instance_type: model.instance_type.unwrap_or_default(),
            image_id: model.ami.unwrap_or_default(),
            state: "active".to_string(),
            status_code: "fulfilled".to_string(),
            instance_id: model.spot_instance_id,
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.spot_requests.insert(request_id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for r in view.spot_requests.values() {
            let attrs = serde_json::json!({
                "id": r.spot_instance_request_id,
                "spot_price": r.spot_price,
                "instance_type": r.instance_type,
                "ami": r.image_id,
                "spot_instance_id": r.instance_id,
                "tags": r.tags,
            });
            results.push(ExtractedResource {
                name: r.spot_instance_request_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_spot_datafeed_subscription
// ---------------------------------------------------------------------------

pub struct AwsSpotDatafeedSubscriptionConverter {
    service: Arc<Ec2Service>,
}

impl AwsSpotDatafeedSubscriptionConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSpotDatafeedSubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_spot_datafeed_subscription"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // No backing state view: singleton subscription is not modelled in
        // Ec2StateView.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsSpotDatafeedSubscriptionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let _model: ec2_gen::SpotDatafeedSubscriptionTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_spot_datafeed_subscription", e))?;

        // The Ec2StateView does not currently model the singleton spot datafeed
        // subscription. The TF state still round-trips through this converter so
        // the resource can be parsed; we just do not project anything.
        eprintln!(
            "warning: aws_spot_datafeed_subscription: Ec2StateView has no slot for the singleton subscription; inject is a no-op"
        );

        // Touch the service so the merge log records visiting the scope.
        self.service
            .merge(&ctx.default_account_id, &region, minimal_ec2_state_view())
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec!["spot datafeed subscription is not modelled in state".to_string()],
        })
    }
}

// ---------------------------------------------------------------------------
// Wave 5 — Carrier Gateway + Managed Prefix List + Tag
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// aws_ec2_carrier_gateway
// ---------------------------------------------------------------------------

pub struct AwsEc2CarrierGatewayConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2CarrierGatewayConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2CarrierGatewayConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_carrier_gateway"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2CarrierGatewayConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::CarrierGatewayTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_carrier_gateway", e))?;

        let carrier_gateway_id = model.id.clone().unwrap_or_else(|| random_short_id("cagw"));
        let tags = extract_tags(attrs);

        let view = CarrierGatewayView {
            carrier_gateway_id: carrier_gateway_id.clone(),
            vpc_id: model.vpc_id,
            state: "available".to_string(),
            tags,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.carrier_gateways.insert(carrier_gateway_id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for cg in view.carrier_gateways.values() {
            let attrs = serde_json::json!({
                "id": cg.carrier_gateway_id,
                "vpc_id": cg.vpc_id,
                "tags": cg.tags,
            });
            results.push(ExtractedResource {
                name: cg.carrier_gateway_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_managed_prefix_list
// ---------------------------------------------------------------------------

pub struct AwsEc2ManagedPrefixListConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2ManagedPrefixListConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2ManagedPrefixListConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_managed_prefix_list"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2ManagedPrefixListConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::ManagedPrefixListTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_managed_prefix_list", e))?;

        let prefix_list_id = model.id.clone().unwrap_or_else(|| random_short_id("pl"));
        let tags = extract_tags(attrs);

        // Parse inline entry blocks if present.
        let entries: Vec<PrefixListEntryView> = attrs
            .get("entry")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|e| {
                        let cidr = e.get("cidr").and_then(|v| v.as_str())?.to_string();
                        let description = e
                            .get("description")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        Some(PrefixListEntryView { cidr, description })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let view = ManagedPrefixListView {
            prefix_list_id: prefix_list_id.clone(),
            prefix_list_name: model.name,
            state: "create-complete".to_string(),
            address_family: model.address_family,
            max_entries: model.max_entries as i32,
            entries,
            tags,
            version: 1,
            version_history: Vec::new(),
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.managed_prefix_lists.insert(prefix_list_id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for pl in view.managed_prefix_lists.values() {
            let entry_json: Vec<serde_json::Value> = pl
                .entries
                .iter()
                .map(|e| {
                    serde_json::json!({
                        "cidr": e.cidr,
                        "description": e.description,
                    })
                })
                .collect();
            let attrs = serde_json::json!({
                "id": pl.prefix_list_id,
                "name": pl.prefix_list_name,
                "address_family": pl.address_family,
                "max_entries": pl.max_entries,
                "entry": entry_json,
                "tags": pl.tags,
            });
            results.push(ExtractedResource {
                name: pl.prefix_list_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_managed_prefix_list_entry
// ---------------------------------------------------------------------------

pub struct AwsEc2ManagedPrefixListEntryConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2ManagedPrefixListEntryConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2ManagedPrefixListEntryConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_managed_prefix_list_entry"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ec2_managed_prefix_list"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEc2ManagedPrefixListEntryConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::ManagedPrefixListEntryTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_ec2_managed_prefix_list_entry", e))?;

        let entry = PrefixListEntryView {
            cidr: model.cidr.clone(),
            description: model.description,
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if let Some(pl) = snapshot.managed_prefix_lists.get_mut(&model.prefix_list_id) {
            pl.entries.retain(|e| e.cidr != model.cidr);
            pl.entries.push(entry);
        } else {
            eprintln!(
                "warning: aws_ec2_managed_prefix_list_entry: prefix list '{}' not found in state",
                model.prefix_list_id
            );
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Entries round-trip via the parent aws_ec2_managed_prefix_list resource.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_tag
// ---------------------------------------------------------------------------

pub struct AwsEc2TagConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2TagConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2TagConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_tag"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Tags already round-trip through their parent resource's `tags` map.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEc2TagConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::Ec2TagTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_tag", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        let prefix = model
            .resource_id
            .split('-')
            .next()
            .unwrap_or("")
            .to_string();
        let mut applied = false;
        let mut warnings = vec![];

        match prefix.as_str() {
            "vpc" => {
                if let Some(v) = snapshot.vpcs.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "subnet" => {
                if let Some(v) = snapshot.subnets.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "igw" => {
                if let Some(v) = snapshot.igws.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "sg" => {
                if let Some(v) = snapshot.security_groups.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "rtb" => {
                if let Some(v) = snapshot.route_tables.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "key" => {
                if let Some(v) = snapshot.key_pairs.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "acl" => {
                if let Some(v) = snapshot.network_acls.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "eipalloc" | "eipassoc" => {
                if let Some(v) = snapshot.elastic_ips.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "nat" => {
                if let Some(v) = snapshot.nat_gateways.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "dopt" => {
                if let Some(v) = snapshot.dhcp_options.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "eigw" => {
                if let Some(v) = snapshot.egress_only_igws.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "fl" => {
                if let Some(v) = snapshot.flow_logs.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "pcx" => {
                if let Some(v) = snapshot.vpc_peering_connections.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "vpce" => {
                if let Some(v) = snapshot.vpc_endpoints.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "pl" => {
                if let Some(v) = snapshot.managed_prefix_lists.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "cgw" => {
                if let Some(v) = snapshot.customer_gateways.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "vgw" => {
                if let Some(v) = snapshot.vpn_gateways.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "vpn" => {
                if let Some(v) = snapshot.vpn_connections.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "cagw" => {
                if let Some(v) = snapshot.carrier_gateways.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "eni" => {
                if let Some(v) = snapshot.network_interfaces.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "tgw" => {
                if let Some(v) = snapshot.transit_gateways.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "i" => {
                if let Some(v) = snapshot.instances.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "vol" => {
                if let Some(v) = snapshot.volumes.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "snap" => {
                if let Some(v) = snapshot.snapshots.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "ami" => {
                if let Some(v) = snapshot.images.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "lt" => {
                if let Some(v) = snapshot.launch_templates.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "h" => {
                if let Some(v) = snapshot.dedicated_hosts.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "fleet" => {
                if let Some(v) = snapshot.ec2_fleets.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "sfr" => {
                if let Some(v) = snapshot.spot_fleet_requests.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            "sir" => {
                if let Some(v) = snapshot.spot_requests.get_mut(&model.resource_id) {
                    v.tags.insert(model.key.clone(), model.value.clone());
                    applied = true;
                }
            }
            _ => {}
        }

        if !applied {
            warnings.push(format!(
                "aws_ec2_tag: resource '{}' (prefix '{}') not found or unsupported; tag not applied",
                model.resource_id, prefix
            ));
        } else {
            self.service
                .restore(&ctx.default_account_id, &region, snapshot)
                .await?;
        }

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// Wave 5 — VPC Peering family
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// aws_vpc_peering_connection
// ---------------------------------------------------------------------------

pub struct AwsVpcPeeringConnectionConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcPeeringConnectionConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcPeeringConnectionConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_peering_connection"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsVpcPeeringConnectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcPeeringConnectionTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpc_peering_connection", e))?;

        let peering_id = model.id.clone().unwrap_or_else(|| random_short_id("pcx"));
        let tags = extract_tags(attrs);

        let status = if model.auto_accept {
            "active".to_string()
        } else {
            "pending-acceptance".to_string()
        };

        let view = VpcPeeringConnectionView {
            peering_id: peering_id.clone(),
            requester_vpc_id: model.vpc_id,
            accepter_vpc_id: Some(model.peer_vpc_id),
            status,
            tags,
            requester_peering_options: None,
            accepter_peering_options: None,
        };

        let mut state_view = minimal_ec2_state_view();
        state_view.vpc_peering_connections.insert(peering_id, view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for p in view.vpc_peering_connections.values() {
            let attrs = serde_json::json!({
                "id": p.peering_id,
                "vpc_id": p.requester_vpc_id,
                "peer_vpc_id": p.accepter_vpc_id,
                "auto_accept": p.status == "active",
                "tags": p.tags,
            });
            results.push(ExtractedResource {
                name: p.peering_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_peering_connection_accepter
// ---------------------------------------------------------------------------

pub struct AwsVpcPeeringConnectionAccepterConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcPeeringConnectionAccepterConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcPeeringConnectionAccepterConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_peering_connection_accepter"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_peering_connection"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Accepter is a status modifier; round-trip via aws_vpc_peering_connection.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcPeeringConnectionAccepterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcPeeringConnectionAccepterTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_peering_connection_accepter", e)
            })?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(conn) = snapshot
            .vpc_peering_connections
            .get_mut(&model.vpc_peering_connection_id)
        {
            conn.status = "active".to_string();
        } else {
            warnings.push(format!(
                "vpc peering connection '{}' not found; accepter ignored",
                model.vpc_peering_connection_id
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_vpc_peering_connection_options
// ---------------------------------------------------------------------------

pub struct AwsVpcPeeringConnectionOptionsConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcPeeringConnectionOptionsConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcPeeringConnectionOptionsConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_peering_connection_options"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_peering_connection"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Options round-trip via the parent aws_vpc_peering_connection resource.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcPeeringConnectionOptionsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcPeeringConnectionOptionsTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_vpc_peering_connection_options", e))?;

        let requester_opts = parse_peering_options_block(attrs, "requester");
        let accepter_opts = parse_peering_options_block(attrs, "accepter");

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(conn) = snapshot
            .vpc_peering_connections
            .get_mut(&model.vpc_peering_connection_id)
        {
            if requester_opts.is_some() {
                conn.requester_peering_options = requester_opts;
            }
            if accepter_opts.is_some() {
                conn.accepter_peering_options = accepter_opts;
            }
        } else {
            warnings.push(format!(
                "vpc peering connection '{}' not found; options ignored",
                model.vpc_peering_connection_id
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

fn parse_peering_options_block(
    attrs: &serde_json::Value,
    key: &str,
) -> Option<VpcPeeringConnectionOptionsView> {
    let block = attrs.get(key)?;
    let first = match block {
        serde_json::Value::Array(arr) => arr.first()?,
        serde_json::Value::Object(_) => block,
        _ => return None,
    };
    Some(VpcPeeringConnectionOptionsView {
        allow_dns_resolution_from_remote_vpc: first
            .get("allow_remote_vpc_dns_resolution")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        allow_egress_from_local_classic_link_to_remote_vpc: first
            .get("allow_classic_link_to_remote_vpc")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        allow_egress_from_local_vpc_to_remote_classic_link: first
            .get("allow_vpc_to_remote_classic_link")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
    })
}

// ===========================================================================
// Wave 6 — Default-* family, account singletons, IPAM extras, VPC misc,
// route server family, Verified Access extras (~28 resources).
//
// For resources without a matching state view, the converter is a
// warning-only inject — emit an `eprintln!` warning and return Ok(vec![])
// from extract. Resources that do have state are full modifiers.
// ===========================================================================

// Helper: emit a warning-only converter for a resource whose state slot is
// not modelled in winterbaume_ec2. Centralised to keep wave 6 boilerplate
// compact.
macro_rules! ec2_warning_only_converter {
    (
        struct_name = $struct_name:ident,
        resource_type = $resource_type:expr,
        model_type = $model_type:ident,
        warn_msg = $warn_msg:expr $(,)?
    ) => {
        pub struct $struct_name {
            #[allow(dead_code)]
            service: Arc<Ec2Service>,
        }

        impl $struct_name {
            pub fn new(service: Arc<Ec2Service>) -> Self {
                Self { service }
            }
        }

        impl TerraformResourceConverter for $struct_name {
            fn resource_type(&self) -> &str {
                $resource_type
            }

            fn inject<'a>(
                &'a self,
                instance: &'a ResourceInstance,
                ctx: &'a ConversionContext,
            ) -> Pin<
                Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>,
            > {
                Box::pin(async move { self.do_inject(instance, ctx).await })
            }

            fn extract<'a>(
                &'a self,
                _ctx: &'a ConversionContext,
            ) -> Pin<
                Box<
                    dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>>
                        + Send
                        + 'a,
                >,
            > {
                Box::pin(async move { Ok(vec![]) })
            }
        }

        impl $struct_name {
            async fn do_inject(
                &self,
                instance: &ResourceInstance,
                ctx: &ConversionContext,
            ) -> Result<ConversionResult, ConversionError> {
                let attrs = &instance.attributes;
                let region = extract_region(attrs, &ctx.default_region);
                let _model: ec2_gen::$model_type = serde_json::from_value(attrs.clone())
                    .map_err(|e| classify_deserialize_error($resource_type, e))?;
                eprintln!("warning: {}: {}", $resource_type, $warn_msg);
                Ok(ConversionResult {
                    region,
                    warnings: vec![format!("{}: {}", $resource_type, $warn_msg)],
                })
            }
        }
    };
}

// ---------------------------------------------------------------------------
// aws_default_network_acl — snapshot+adopt an existing default NACL.
// ---------------------------------------------------------------------------

pub struct AwsDefaultNetworkAclConverter {
    service: Arc<Ec2Service>,
}

impl AwsDefaultNetworkAclConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDefaultNetworkAclConverter {
    fn resource_type(&self) -> &str {
        "aws_default_network_acl"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDefaultNetworkAclConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::DefaultNetworkAclTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_default_network_acl", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(nacl) = snapshot.network_acls.get_mut(&model.default_network_acl_id) {
            nacl.is_default = true;
            let tags = extract_tags(attrs);
            for (k, v) in tags {
                nacl.tags.insert(k, v);
            }
        } else {
            warnings.push(format!(
                "aws_default_network_acl: NACL '{}' not found in state; default flag not applied",
                model.default_network_acl_id
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_default_route_table — adopt an existing default route table.
// ---------------------------------------------------------------------------

pub struct AwsDefaultRouteTableConverter {
    service: Arc<Ec2Service>,
}

impl AwsDefaultRouteTableConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDefaultRouteTableConverter {
    fn resource_type(&self) -> &str {
        "aws_default_route_table"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDefaultRouteTableConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::DefaultRouteTableTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_default_route_table", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(rt) = snapshot.route_tables.get_mut(&model.default_route_table_id) {
            let tags = extract_tags(attrs);
            for (k, v) in tags {
                rt.tags.insert(k, v);
            }
        } else {
            warnings.push(format!(
                "aws_default_route_table: route table '{}' not found in state",
                model.default_route_table_id
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_default_security_group — adopt an existing default security group.
// ---------------------------------------------------------------------------

pub struct AwsDefaultSecurityGroupConverter {
    service: Arc<Ec2Service>,
}

impl AwsDefaultSecurityGroupConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDefaultSecurityGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_default_security_group"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDefaultSecurityGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::DefaultSecurityGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_default_security_group", e))?;

        let sg_id = match model.id.as_deref() {
            Some(id) => id.to_string(),
            None => {
                eprintln!(
                    "warning: aws_default_security_group: no `id` attribute; cannot adopt default SG"
                );
                return Ok(ConversionResult {
                    region,
                    warnings: vec!["aws_default_security_group: missing id".into()],
                });
            }
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(sg) = snapshot.security_groups.get_mut(&sg_id) {
            let tags = extract_tags(attrs);
            for (k, v) in tags {
                sg.tags.insert(k, v);
            }
        } else {
            warnings.push(format!(
                "aws_default_security_group: SG '{}' not found in state",
                sg_id
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_default_subnet — adopt an existing default subnet for an AZ.
// ---------------------------------------------------------------------------

pub struct AwsDefaultSubnetConverter {
    service: Arc<Ec2Service>,
}

impl AwsDefaultSubnetConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDefaultSubnetConverter {
    fn resource_type(&self) -> &str {
        "aws_default_subnet"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDefaultSubnetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::DefaultSubnetTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_default_subnet", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        let target_id = model.id.clone();
        let mut adopted = false;
        if let Some(id) = target_id.as_deref() {
            if let Some(s) = snapshot.subnets.get_mut(id) {
                let tags = extract_tags(attrs);
                for (k, v) in tags {
                    s.tags.insert(k, v);
                }
                adopted = true;
            }
        }
        if !adopted {
            // Fall back to looking up by AZ.
            if let Some(s) = snapshot
                .subnets
                .values_mut()
                .find(|s| s.availability_zone == model.availability_zone)
            {
                let tags = extract_tags(attrs);
                for (k, v) in tags {
                    s.tags.insert(k, v);
                }
                adopted = true;
            }
        }
        if !adopted {
            warnings.push(format!(
                "aws_default_subnet: no default subnet found for AZ '{}'",
                model.availability_zone
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_default_vpc — adopt the region's default VPC (or create if missing).
// ---------------------------------------------------------------------------

pub struct AwsDefaultVpcConverter {
    service: Arc<Ec2Service>,
}

impl AwsDefaultVpcConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDefaultVpcConverter {
    fn resource_type(&self) -> &str {
        "aws_default_vpc"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDefaultVpcConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::DefaultVpcTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_default_vpc", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        let mut adopted = false;
        if let Some(id) = model.id.as_deref() {
            if let Some(v) = snapshot.vpcs.get_mut(id) {
                v.is_default = true;
                v.enable_dns_hostnames = model.enable_dns_hostnames;
                v.enable_dns_support = model.enable_dns_support;
                let tags = extract_tags(attrs);
                for (k, v_) in tags {
                    v.tags.insert(k, v_);
                }
                adopted = true;
            }
        }
        if !adopted {
            if let Some(v) = snapshot.vpcs.values_mut().find(|v| v.is_default) {
                v.enable_dns_hostnames = model.enable_dns_hostnames;
                v.enable_dns_support = model.enable_dns_support;
                let tags = extract_tags(attrs);
                for (k, v_) in tags {
                    v.tags.insert(k, v_);
                }
                adopted = true;
            }
        }
        if !adopted {
            warnings.push("aws_default_vpc: no default VPC found in state; nothing adopted".into());
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_default_vpc_dhcp_options — adopt the AWS-default DHCP option set.
// ---------------------------------------------------------------------------

pub struct AwsDefaultVpcDhcpOptionsConverter {
    service: Arc<Ec2Service>,
}

impl AwsDefaultVpcDhcpOptionsConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDefaultVpcDhcpOptionsConverter {
    fn resource_type(&self) -> &str {
        "aws_default_vpc_dhcp_options"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDefaultVpcDhcpOptionsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::DefaultVpcDhcpOptionsTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_default_vpc_dhcp_options", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        let mut adopted = false;
        if let Some(id) = model.id.as_deref() {
            if let Some(dopt) = snapshot.dhcp_options.get_mut(id) {
                let tags = extract_tags(attrs);
                for (k, v) in tags {
                    dopt.tags.insert(k, v);
                }
                adopted = true;
            }
        }
        if !adopted {
            warnings.push("aws_default_vpc_dhcp_options: no matching DHCP option set found".into());
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_availability_zone_group — opt the AZ group into the account.
// ---------------------------------------------------------------------------

pub struct AwsEc2AvailabilityZoneGroupConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2AvailabilityZoneGroupConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2AvailabilityZoneGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_availability_zone_group"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEc2AvailabilityZoneGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::Ec2AvailabilityZoneGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_availability_zone_group", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot
            .az_group_opt_in
            .insert(model.group_name.clone(), model.opt_in_status.clone());
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_default_credit_specification — set the per-family default credit
// ---------------------------------------------------------------------------

pub struct AwsEc2DefaultCreditSpecificationConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2DefaultCreditSpecificationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2DefaultCreditSpecificationConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_default_credit_specification"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEc2DefaultCreditSpecificationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::Ec2DefaultCreditSpecificationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_ec2_default_credit_specification", e)
            })?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot
            .default_credit_specifications
            .insert(model.instance_family.clone(), model.cpu_credits.clone());
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_serial_console_access — account-wide toggle.
// ---------------------------------------------------------------------------

pub struct AwsEc2SerialConsoleAccessConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2SerialConsoleAccessConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2SerialConsoleAccessConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_serial_console_access"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEc2SerialConsoleAccessConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::Ec2SerialConsoleAccessTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_ec2_serial_console_access", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.serial_console_access_enabled = model.enabled;
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_image_block_public_access — account-wide toggle.
// ---------------------------------------------------------------------------

pub struct AwsEc2ImageBlockPublicAccessConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2ImageBlockPublicAccessConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2ImageBlockPublicAccessConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_image_block_public_access"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEc2ImageBlockPublicAccessConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::Ec2ImageBlockPublicAccessTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_ec2_image_block_public_access", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.image_block_public_access_state = Some(model.state.clone());
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_instance_metadata_defaults — write the account-wide IMDS defaults.
// ---------------------------------------------------------------------------

pub struct AwsEc2InstanceMetadataDefaultsConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2InstanceMetadataDefaultsConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2InstanceMetadataDefaultsConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_instance_metadata_defaults"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEc2InstanceMetadataDefaultsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::Ec2InstanceMetadataDefaultsTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_ec2_instance_metadata_defaults", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.instance_metadata_defaults =
            Some(winterbaume_ec2::views::InstanceMetadataDefaultsView {
                http_tokens: model.http_tokens.clone(),
                http_put_response_hop_limit: if model.http_put_response_hop_limit > 0 {
                    Some(model.http_put_response_hop_limit as i32)
                } else {
                    None
                },
                http_endpoint: model.http_endpoint.clone(),
                instance_metadata_tags: model.instance_metadata_tags.clone(),
            });
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_ec2_instance_state — flip a single instance running/stopped.
// ---------------------------------------------------------------------------

pub struct AwsEc2InstanceStateConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2InstanceStateConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2InstanceStateConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_instance_state"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEc2InstanceStateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::Ec2InstanceStateTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_instance_state", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(inst) = snapshot.instances.get_mut(&model.instance_id) {
            let (code, name) = match model.state.as_str() {
                "running" => (16, "running"),
                "stopped" => (80, "stopped"),
                "pending" => (0, "pending"),
                "stopping" => (64, "stopping"),
                "shutting-down" => (32, "shutting-down"),
                "terminated" => (48, "terminated"),
                other => (16, other),
            };
            inst.state = InstanceStateView {
                code,
                name: name.to_string(),
            };
        } else {
            warnings.push(format!(
                "aws_ec2_instance_state: instance '{}' not found in state",
                model.instance_id
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// IPAM extras
// ---------------------------------------------------------------------------

ec2_warning_only_converter! {
    struct_name = AwsVpcIpamOrganizationAdminAccountConverter,
    resource_type = "aws_vpc_ipam_organization_admin_account",
    model_type = VpcIpamOrganizationAdminAccountTfModel,
    warn_msg = "Organizations IPAM-admin slot not modelled in winterbaume_ec2; inject is a no-op",
}

ec2_warning_only_converter! {
    struct_name = AwsVpcIpamPreviewNextCidrConverter,
    resource_type = "aws_vpc_ipam_preview_next_cidr",
    model_type = VpcIpamPreviewNextCidrTfModel,
    warn_msg = "preview-only resource; no state writes",
}

// aws_vpc_ipam_resource_discovery_association — modifier on IPAM resource
// discovery association map.
pub struct AwsVpcIpamResourceDiscoveryAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcIpamResourceDiscoveryAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcIpamResourceDiscoveryAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_ipam_resource_discovery_association"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcIpamResourceDiscoveryAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcIpamResourceDiscoveryAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_ipam_resource_discovery_association", e)
            })?;

        let association_id = model
            .id
            .clone()
            .unwrap_or_else(|| random_short_id("ipam-res-disco-assoc"));
        let assoc = IpamResourceDiscoveryAssociationView {
            ipam_resource_discovery_association_id: association_id.clone(),
            ipam_resource_discovery_association_arn: format!(
                "arn:aws:ec2::{}:ipam-resource-discovery-association/{}",
                ctx.default_account_id, association_id
            ),
            ipam_arn: format!(
                "arn:aws:ec2::{}:ipam/{}",
                ctx.default_account_id, model.ipam_id
            ),
            ipam_id: model.ipam_id.clone(),
            ipam_region: region.clone(),
            ipam_resource_discovery_id: model.ipam_resource_discovery_id.clone(),
            owner_id: ctx.default_account_id.clone(),
            is_default: false,
            resource_discovery_status: "active".to_string(),
            state: "associate-complete".to_string(),
            tags: extract_tags(attrs),
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot
            .ipam_resource_discovery_associations
            .insert(association_id, assoc);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// aws_ec2_subnet_cidr_reservation — modifier on subnet_cidr_reservations.
pub struct AwsEc2SubnetCidrReservationConverter {
    service: Arc<Ec2Service>,
}

impl AwsEc2SubnetCidrReservationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEc2SubnetCidrReservationConverter {
    fn resource_type(&self) -> &str {
        "aws_ec2_subnet_cidr_reservation"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEc2SubnetCidrReservationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::Ec2SubnetCidrReservationTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ec2_subnet_cidr_reservation", e))?;

        let reservation_id = model.id.clone().unwrap_or_else(|| random_short_id("scr"));
        let view = SubnetCidrReservationView {
            reservation_id: reservation_id.clone(),
            subnet_id: model.subnet_id.clone(),
            cidr: model.cidr_block.clone(),
            reservation_type: model.reservation_type.clone(),
            description: model.description.unwrap_or_default(),
            owner_id: ctx.default_account_id.clone(),
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot
            .subnet_cidr_reservations
            .insert(reservation_id, view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// VPC misc
// ---------------------------------------------------------------------------

// aws_vpc_block_public_access_exclusion
pub struct AwsVpcBlockPublicAccessExclusionConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcBlockPublicAccessExclusionConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcBlockPublicAccessExclusionConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_block_public_access_exclusion"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcBlockPublicAccessExclusionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcBlockPublicAccessExclusionTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_block_public_access_exclusion", e)
            })?;

        let exclusion_id = model
            .id
            .clone()
            .unwrap_or_else(|| random_short_id("vbpa-excl"));
        let resource_arn = match (&model.vpc_id, &model.subnet_id) {
            (Some(vid), _) => format!(
                "arn:aws:ec2:{}:{}:vpc/{}",
                region, ctx.default_account_id, vid
            ),
            (_, Some(sid)) => format!(
                "arn:aws:ec2:{}:{}:subnet/{}",
                region, ctx.default_account_id, sid
            ),
            _ => String::new(),
        };
        let view = VpcBlockPublicAccessExclusionView {
            exclusion_id: exclusion_id.clone(),
            internet_gateway_exclusion_mode: model.internet_gateway_exclusion_mode.clone(),
            resource_arn,
            state: "create-complete".to_string(),
            creation_timestamp: String::new(),
            last_update_timestamp: String::new(),
            tags: extract_tags(attrs),
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot
            .vpc_block_public_access_exclusions
            .insert(exclusion_id, view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// aws_vpc_block_public_access_options
pub struct AwsVpcBlockPublicAccessOptionsConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcBlockPublicAccessOptionsConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcBlockPublicAccessOptionsConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_block_public_access_options"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcBlockPublicAccessOptionsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcBlockPublicAccessOptionsTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_block_public_access_options", e)
            })?;

        let view = VpcBlockPublicAccessOptionsView {
            aws_account_id: ctx.default_account_id.clone(),
            aws_region: region.clone(),
            internet_gateway_block_mode: model.internet_gateway_block_mode.clone(),
            state: "update-complete".to_string(),
            last_update_timestamp: String::new(),
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.vpc_block_public_access_options = Some(view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// aws_vpc_ipv4_cidr_block_association — add a secondary CIDR to a VPC.
pub struct AwsVpcIpv4CidrBlockAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcIpv4CidrBlockAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcIpv4CidrBlockAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_ipv4_cidr_block_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcIpv4CidrBlockAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcIpv4CidrBlockAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_ipv4_cidr_block_association", e)
            })?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(vpc) = snapshot.vpcs.get_mut(&model.vpc_id) {
            if let Some(cidr) = model.cidr_block.as_deref() {
                let assoc_id = model
                    .id
                    .clone()
                    .unwrap_or_else(|| random_short_id("vpc-cidr-assoc"));
                vpc.secondary_cidr_blocks.push((assoc_id, cidr.to_string()));
            } else {
                warnings.push(
                    "aws_vpc_ipv4_cidr_block_association: cidr_block not present; nothing applied"
                        .into(),
                );
            }
        } else {
            warnings.push(format!(
                "aws_vpc_ipv4_cidr_block_association: VPC '{}' not found in state",
                model.vpc_id
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// aws_vpc_ipv6_cidr_block_association — add an IPv6 block to a VPC.
pub struct AwsVpcIpv6CidrBlockAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcIpv6CidrBlockAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcIpv6CidrBlockAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_ipv6_cidr_block_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcIpv6CidrBlockAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcIpv6CidrBlockAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_ipv6_cidr_block_association", e)
            })?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(vpc) = snapshot.vpcs.get_mut(&model.vpc_id) {
            if let Some(cidr) = model.ipv6_cidr_block.as_deref() {
                let assoc_id = model
                    .id
                    .clone()
                    .unwrap_or_else(|| random_short_id("vpc-ipv6-cidr-assoc"));
                vpc.secondary_cidr_blocks.push((assoc_id, cidr.to_string()));
            } else {
                warnings.push(
                    "aws_vpc_ipv6_cidr_block_association: ipv6_cidr_block not present".into(),
                );
            }
        } else {
            warnings.push(format!(
                "aws_vpc_ipv6_cidr_block_association: VPC '{}' not found in state",
                model.vpc_id
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// aws_vpc_network_performance_metric_subscription
pub struct AwsVpcNetworkPerformanceMetricSubscriptionConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcNetworkPerformanceMetricSubscriptionConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcNetworkPerformanceMetricSubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_network_performance_metric_subscription"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcNetworkPerformanceMetricSubscriptionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcNetworkPerformanceMetricSubscriptionTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_network_performance_metric_subscription", e)
            })?;

        let view = AwsNetworkPerformanceSubscriptionView {
            source: model.source.clone(),
            destination: model.destination.clone(),
            metric: model.metric.clone(),
            statistic: model.statistic.clone(),
            period: model.period.unwrap_or_default(),
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        // Idempotent insert keyed by (source, destination, metric, statistic).
        snapshot.aws_network_performance_subscriptions.retain(|s| {
            !(s.source == view.source
                && s.destination == view.destination
                && s.metric == view.metric
                && s.statistic == view.statistic)
        });
        snapshot.aws_network_performance_subscriptions.push(view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// Route server family — full state writes via route_servers/.../associations.
// ---------------------------------------------------------------------------

// aws_vpc_route_server
pub struct AwsVpcRouteServerConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcRouteServerConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcRouteServerConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_route_server"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcRouteServerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcRouteServerTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpc_route_server", e))?;

        let rs_id = model.id.clone().unwrap_or_else(|| random_short_id("rs"));
        let view = winterbaume_ec2::views::RouteServerView {
            route_server_id: rs_id.clone(),
            route_server_arn: format!(
                "arn:aws:ec2:{}:{}:route-server/{}",
                region, ctx.default_account_id, rs_id
            ),
            amazon_side_asn: model.amazon_side_asn,
            state: "available".to_string(),
            persist_routes: model
                .persist_routes
                .unwrap_or_else(|| "disable".to_string()),
            persist_routes_duration: if model.persist_routes_duration > 0 {
                Some(model.persist_routes_duration)
            } else {
                None
            },
            sns_notifications_enabled: model.sns_notifications_enabled,
            sns_topic_arn: model.sns_topic_arn.clone(),
            tags: extract_tags(attrs),
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.route_servers.insert(rs_id, view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// aws_vpc_route_server_endpoint
pub struct AwsVpcRouteServerEndpointConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcRouteServerEndpointConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcRouteServerEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_route_server_endpoint"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_route_server"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcRouteServerEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcRouteServerEndpointTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_vpc_route_server_endpoint", e))?;

        let ep_id = model.id.clone().unwrap_or_else(|| random_short_id("rse"));
        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let vpc_id = snapshot
            .subnets
            .get(&model.subnet_id)
            .map(|s| s.vpc_id.clone())
            .unwrap_or_default();
        let view = winterbaume_ec2::views::RouteServerEndpointView {
            route_server_endpoint_id: ep_id.clone(),
            route_server_id: model.route_server_id.clone(),
            vpc_id,
            subnet_id: model.subnet_id.clone(),
            eni_id: random_short_id("eni"),
            eni_address: None,
            state: "available".to_string(),
            failure_reason: None,
            tags: extract_tags(attrs),
        };
        snapshot.route_server_endpoints.insert(ep_id, view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// aws_vpc_route_server_peer
pub struct AwsVpcRouteServerPeerConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcRouteServerPeerConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcRouteServerPeerConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_route_server_peer"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_route_server_endpoint"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcRouteServerPeerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcRouteServerPeerTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_vpc_route_server_peer", e))?;

        let peer_id = model.id.clone().unwrap_or_else(|| random_short_id("rsp"));
        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let (rs_id, vpc_id, subnet_id) = match snapshot
            .route_server_endpoints
            .get(&model.route_server_endpoint_id)
        {
            Some(ep) => (
                ep.route_server_id.clone(),
                ep.vpc_id.clone(),
                ep.subnet_id.clone(),
            ),
            None => (String::new(), String::new(), String::new()),
        };
        let view = winterbaume_ec2::views::RouteServerPeerView {
            route_server_peer_id: peer_id.clone(),
            route_server_endpoint_id: model.route_server_endpoint_id.clone(),
            route_server_id: rs_id,
            vpc_id,
            subnet_id,
            peer_address: model.peer_address.clone(),
            state: "available".to_string(),
            failure_reason: None,
            options: winterbaume_ec2::views::RouteServerPeerOptionsView::default(),
            endpoint_eni_id: None,
            endpoint_eni_address: None,
            tags: extract_tags(attrs),
        };
        snapshot.route_server_peers.insert(peer_id, view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// aws_vpc_route_server_propagation — modifier appending propagation onto the
// route-server association for the route table's VPC.
pub struct AwsVpcRouteServerPropagationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcRouteServerPropagationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcRouteServerPropagationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_route_server_propagation"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_route_server"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcRouteServerPropagationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcRouteServerPropagationTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_vpc_route_server_propagation", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let vpc_id = snapshot
            .route_tables
            .get(&model.route_table_id)
            .map(|rt| rt.vpc_id.clone())
            .unwrap_or_default();
        let mut warnings = vec![];
        let assoc = snapshot.route_server_associations.iter_mut().find(|a| {
            a.route_server_id == model.route_server_id && a.vpc_id == vpc_id && !vpc_id.is_empty()
        });
        match assoc {
            Some(a) => {
                if !a.propagations.contains(&model.route_table_id) {
                    a.propagations.push(model.route_table_id.clone());
                }
            }
            None => {
                warnings.push(format!(
                    "aws_vpc_route_server_propagation: no route-server '{}' association for VPC '{}'",
                    model.route_server_id, vpc_id
                ));
            }
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// aws_vpc_route_server_vpc_association — append a new association.
pub struct AwsVpcRouteServerVpcAssociationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVpcRouteServerVpcAssociationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVpcRouteServerVpcAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_vpc_route_server_vpc_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_vpc_route_server"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVpcRouteServerVpcAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VpcRouteServerVpcAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_vpc_route_server_vpc_association", e)
            })?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let exists = snapshot
            .route_server_associations
            .iter()
            .any(|a| a.route_server_id == model.route_server_id && a.vpc_id == model.vpc_id);
        if !exists {
            snapshot
                .route_server_associations
                .push(RouteServerAssociationView {
                    route_server_id: model.route_server_id.clone(),
                    vpc_id: model.vpc_id.clone(),
                    state: "available".to_string(),
                    propagations: vec![],
                });
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// Verified Access extras
// ---------------------------------------------------------------------------

// aws_verifiedaccess_instance_logging_configuration — modifier on
// VerifiedAccessInstanceView's logging configuration map.
pub struct AwsVerifiedaccessInstanceLoggingConfigurationConverter {
    service: Arc<Ec2Service>,
}

impl AwsVerifiedaccessInstanceLoggingConfigurationConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVerifiedaccessInstanceLoggingConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_verifiedaccess_instance_logging_configuration"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_verifiedaccess_instance"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVerifiedaccessInstanceLoggingConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VerifiedAccessInstanceLoggingConfigurationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_verifiedaccess_instance_logging_configuration", e)
            })?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if snapshot
            .verified_access_instances
            .contains_key(&model.verifiedaccess_instance_id)
        {
            snapshot
                .verified_access_instance_logging_configurations
                .insert(
                    model.verifiedaccess_instance_id.clone(),
                    winterbaume_ec2::views::VerifiedAccessLogsView::default(),
                );
        } else {
            warnings.push(format!(
                "aws_verifiedaccess_instance_logging_configuration: instance '{}' not found",
                model.verifiedaccess_instance_id
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// aws_verifiedaccess_instance_trust_provider_attachment
pub struct AwsVerifiedaccessInstanceTrustProviderAttachmentConverter {
    service: Arc<Ec2Service>,
}

impl AwsVerifiedaccessInstanceTrustProviderAttachmentConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsVerifiedaccessInstanceTrustProviderAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_verifiedaccess_instance_trust_provider_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_verifiedaccess_instance",
            "aws_verifiedaccess_trust_provider",
        ]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsVerifiedaccessInstanceTrustProviderAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: ec2_gen::VerifiedAccessInstanceTrustProviderAttachmentTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error(
                    "aws_verifiedaccess_instance_trust_provider_attachment",
                    e,
                )
            })?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(va) = snapshot
            .verified_access_instances
            .get_mut(&model.verifiedaccess_instance_id)
        {
            if !va
                .trust_provider_ids
                .contains(&model.verifiedaccess_trust_provider_id)
            {
                va.trust_provider_ids
                    .push(model.verifiedaccess_trust_provider_id.clone());
            }
        } else {
            warnings.push(format!(
                "aws_verifiedaccess_instance_trust_provider_attachment: instance '{}' not found",
                model.verifiedaccess_instance_id
            ));
        }
        let exists = snapshot
            .verified_access_trust_provider_attachments
            .iter()
            .any(|a| {
                a.instance_id == model.verifiedaccess_instance_id
                    && a.trust_provider_id == model.verifiedaccess_trust_provider_id
            });
        if !exists {
            snapshot.verified_access_trust_provider_attachments.push(
                VerifiedAccessTrustProviderAttachmentView {
                    instance_id: model.verifiedaccess_instance_id.clone(),
                    trust_provider_id: model.verifiedaccess_trust_provider_id.clone(),
                },
            );
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// Wave 7 — EBS account-level singletons and snapshot lifecycle resources.
// All six `aws_ebs_*` resource types below are served by the EC2 service
// crate: account-level toggles live as scalar fields on `Ec2State`, and
// `aws_ebs_snapshot_copy` / `aws_ebs_snapshot_import` materialise into the
// existing `Ec2State::snapshots` and `Ec2State::snapshot_import_tasks`
// collections shared with `aws_ebs_snapshot`.
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// aws_ebs_default_kms_key — account-wide default KMS key ARN.
// ---------------------------------------------------------------------------

pub struct AwsEbsDefaultKmsKeyConverter {
    service: Arc<Ec2Service>,
}

impl AwsEbsDefaultKmsKeyConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEbsDefaultKmsKeyConverter {
    fn resource_type(&self) -> &str {
        "aws_ebs_default_kms_key"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEbsDefaultKmsKeyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model = serde_json::from_value::<ec2_gen::EbsDefaultKmsKeyTfModel>(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ebs_default_kms_key", e))?;

        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let snapshot = Ec2StateView {
            ebs_default_kms_key_id: Some(model.key_arn),
            ..snapshot
        };
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_ebs_encryption_by_default — account-wide encryption toggle.
// ---------------------------------------------------------------------------

pub struct AwsEbsEncryptionByDefaultConverter {
    service: Arc<Ec2Service>,
}

impl AwsEbsEncryptionByDefaultConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEbsEncryptionByDefaultConverter {
    fn resource_type(&self) -> &str {
        "aws_ebs_encryption_by_default"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEbsEncryptionByDefaultConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model = serde_json::from_value::<ec2_gen::EbsEncryptionByDefaultTfModel>(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ebs_encryption_by_default", e))?;

        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let snapshot = Ec2StateView {
            ebs_encryption_by_default: model.enabled,
            ..snapshot
        };
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_ebs_fast_snapshot_restore — append an AZ entry to the per-snapshot
// `fast_snapshot_restore_states` list.
// ---------------------------------------------------------------------------

pub struct AwsEbsFastSnapshotRestoreConverter {
    service: Arc<Ec2Service>,
}

impl AwsEbsFastSnapshotRestoreConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEbsFastSnapshotRestoreConverter {
    fn resource_type(&self) -> &str {
        "aws_ebs_fast_snapshot_restore"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ebs_snapshot", "aws_ebs_snapshot_copy"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEbsFastSnapshotRestoreConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model = serde_json::from_value::<ec2_gen::EbsFastSnapshotRestoreTfModel>(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ebs_fast_snapshot_restore", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(snap) = snapshot.snapshots.get_mut(&model.snapshot_id) {
            let already = snap
                .fast_snapshot_restore_states
                .iter()
                .any(|s| s.availability_zone == model.availability_zone);
            if !already {
                snap.fast_snapshot_restore_states.push(
                    winterbaume_ec2::views::FastSnapshotRestoreStateView {
                        availability_zone: model.availability_zone.clone(),
                        state: "enabled".to_string(),
                    },
                );
            }
        } else {
            warnings.push(format!(
                "aws_ebs_fast_snapshot_restore: snapshot '{}' not found",
                model.snapshot_id
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for snap in view.snapshots.values() {
            for state in &snap.fast_snapshot_restore_states {
                let id = format!("{}:{}", state.availability_zone, snap.snapshot_id);
                let attrs = serde_json::json!({
                    "id": id,
                    "availability_zone": state.availability_zone,
                    "snapshot_id": snap.snapshot_id,
                    "state": state.state,
                });
                results.push(ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ebs_snapshot_block_public_access — account-wide toggle. The EC2 mock
// has a stub handler but no state field today, so the converter records a
// warning rather than dropping state silently.
// ---------------------------------------------------------------------------

pub struct AwsEbsSnapshotBlockPublicAccessConverter {
    #[allow(dead_code)]
    service: Arc<Ec2Service>,
}

impl AwsEbsSnapshotBlockPublicAccessConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEbsSnapshotBlockPublicAccessConverter {
    fn resource_type(&self) -> &str {
        "aws_ebs_snapshot_block_public_access"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsEbsSnapshotBlockPublicAccessConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model = serde_json::from_value::<ec2_gen::EbsSnapshotBlockPublicAccessTfModel>(
            attrs.clone(),
        )
        .map_err(|e| classify_deserialize_error("aws_ebs_snapshot_block_public_access", e))?;

        Ok(ConversionResult {
            region,
            warnings: vec![format!(
                "aws_ebs_snapshot_block_public_access: requested state '{}' is not persisted; EC2State has no `snapshot_block_public_access_state` field",
                model.state
            )],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_ebs_snapshot_copy — clone an existing snapshot. The new snapshot is
// inserted into `Ec2State::snapshots` so subsequent EBS / EC2 operations and
// the `aws_ebs_snapshot` extractor can see it.
// ---------------------------------------------------------------------------

pub struct AwsEbsSnapshotCopyConverter {
    service: Arc<Ec2Service>,
}

impl AwsEbsSnapshotCopyConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEbsSnapshotCopyConverter {
    fn resource_type(&self) -> &str {
        "aws_ebs_snapshot_copy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ebs_snapshot"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEbsSnapshotCopyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model = serde_json::from_value::<ec2_gen::EbsSnapshotCopyTfModel>(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ebs_snapshot_copy", e))?;

        let mut snapshot_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];

        let new_id = optional_str(attrs, "id").unwrap_or_else(|| {
            format!("snap-{}", &uuid::Uuid::new_v4().simple().to_string()[..17])
        });

        // Start from the source snapshot's fields if available, otherwise
        // synthesise a fresh entry. Either way, layer the TF-supplied
        // description / encryption / storage tier / tags over the top.
        let source = snapshot_view
            .snapshots
            .get(&model.source_snapshot_id)
            .cloned();
        if source.is_none() {
            warnings.push(format!(
                "aws_ebs_snapshot_copy: source snapshot '{}' not found; synthesising a stub",
                model.source_snapshot_id
            ));
        }

        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let base = source.unwrap_or_else(|| winterbaume_ec2::views::SnapshotView {
            snapshot_id: new_id.clone(),
            volume_id: String::new(),
            volume_size: 8,
            state: "completed".to_string(),
            description: String::new(),
            start_time: now.clone(),
            progress: "100%".to_string(),
            owner_id: ctx.default_account_id.clone(),
            encrypted: false,
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
        });
        let new_snap = winterbaume_ec2::views::SnapshotView {
            snapshot_id: new_id.clone(),
            owner_id: ctx.default_account_id.clone(),
            start_time: now,
            description: model.description.unwrap_or(base.description.clone()),
            encrypted: model.encrypted || base.encrypted,
            storage_tier: model.storage_tier.unwrap_or(base.storage_tier.clone()),
            tags: if model.tags.is_empty() {
                base.tags.clone()
            } else {
                model.tags
            },
            ..base
        };
        snapshot_view.snapshots.insert(new_id, new_snap);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot_view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // `aws_ebs_snapshot_copy` and `aws_ebs_snapshot` share
        // `Ec2State::snapshots`, so we cannot tell which terraform
        // resource a given snapshot originally came from. Leave the
        // extractor empty to avoid double-counting.
        let _ = ctx;
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_ebs_snapshot_import — import a snapshot from an S3 disk image. The
// converter materialises both a `SnapshotImportTaskView` and the
// derived `SnapshotView` so describe / list operations see the result.
// `disk_container` is a nested block; format / url / s3 bucket / s3 key are
// read raw because the spec vocabulary is scalar-only.
// ---------------------------------------------------------------------------

pub struct AwsEbsSnapshotImportConverter {
    service: Arc<Ec2Service>,
}

impl AwsEbsSnapshotImportConverter {
    pub fn new(service: Arc<Ec2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEbsSnapshotImportConverter {
    fn resource_type(&self) -> &str {
        "aws_ebs_snapshot_import"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsEbsSnapshotImportConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model = serde_json::from_value::<ec2_gen::EbsSnapshotImportTfModel>(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ebs_snapshot_import", e))?;

        // `disk_container` is a nested block (TF state may emit it as either
        // a list-of-one or a single object).
        let disk_container = attrs.get("disk_container").and_then(|v| match v {
            serde_json::Value::Array(arr) => arr.first().cloned(),
            serde_json::Value::Object(_) => Some(v.clone()),
            _ => None,
        });
        let (format, url, s3_bucket, s3_key, dc_description) = match disk_container.as_ref() {
            Some(dc) => {
                let format = dc.get("format").and_then(|v| v.as_str()).map(String::from);
                let url = dc.get("url").and_then(|v| v.as_str()).map(String::from);
                let description = dc
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                let user_bucket = dc.get("user_bucket").and_then(|v| match v {
                    serde_json::Value::Array(arr) => arr.first().cloned(),
                    serde_json::Value::Object(_) => Some(v.clone()),
                    _ => None,
                });
                let (s3_bucket, s3_key) = match user_bucket {
                    Some(b) => (
                        b.get("s3_bucket")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        b.get("s3_key").and_then(|v| v.as_str()).map(String::from),
                    ),
                    None => (None, None),
                };
                (format, url, s3_bucket, s3_key, description)
            }
            None => (None, None, None, None, None),
        };

        let mut snapshot_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        let task_id = optional_str(attrs, "import_task_id").unwrap_or_else(|| {
            format!(
                "import-snap-{}",
                &uuid::Uuid::new_v4().simple().to_string()[..17]
            )
        });
        let new_snap_id = optional_str(attrs, "id").unwrap_or_else(|| {
            format!("snap-{}", &uuid::Uuid::new_v4().simple().to_string()[..17])
        });

        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S.000Z")
            .to_string();
        let description = model.description.clone().or(dc_description);
        let new_snap = winterbaume_ec2::views::SnapshotView {
            snapshot_id: new_snap_id.clone(),
            volume_id: String::new(),
            volume_size: 8,
            state: "completed".to_string(),
            description: description.clone().unwrap_or_default(),
            start_time: now.clone(),
            progress: "100%".to_string(),
            owner_id: ctx.default_account_id.clone(),
            encrypted: model.encrypted,
            tags: model.tags.clone(),
            lock_state: "none".to_string(),
            lock_duration: None,
            lock_created_on: None,
            lock_expires_on: None,
            lock_duration_start_time: None,
            cool_off_period: None,
            cool_off_period_expires_on: None,
            storage_tier: model.storage_tier.unwrap_or_else(|| "standard".to_string()),
            last_tiering_operation_status: None,
            fast_snapshot_restore_states: Vec::new(),
        };
        snapshot_view
            .snapshots
            .insert(new_snap_id.clone(), new_snap);

        let task = winterbaume_ec2::views::SnapshotImportTaskView {
            import_task_id: task_id.clone(),
            status: "completed".to_string(),
            description,
            disk_image_size: Some(8.0 * 1024.0 * 1024.0 * 1024.0),
            format,
            url,
            user_bucket_s3_bucket: s3_bucket,
            user_bucket_s3_key: s3_key,
            owner_id: ctx.default_account_id.clone(),
            encrypted: model.encrypted,
            kms_key_id: model.kms_key_id,
            snapshot_id: Some(new_snap_id),
            tags: model.tags,
        };
        snapshot_view.snapshot_import_tasks.insert(task_id, task);

        self.service
            .restore(&ctx.default_account_id, &region, snapshot_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for task in view.snapshot_import_tasks.values() {
            let snap_id = task.snapshot_id.clone().unwrap_or_default();
            let attrs = serde_json::json!({
                "id": snap_id.clone(),
                "import_task_id": task.import_task_id,
                "description": task.description,
                "encrypted": task.encrypted,
                "kms_key_id": task.kms_key_id,
                "tags": task.tags,
                "tags_all": task.tags,
            });
            results.push(ExtractedResource {
                name: snap_id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn minimal_ec2_state_view() -> Ec2StateView {
    Ec2StateView::default()
}

/// Parse inline `route` blocks from a route table resource.
fn parse_route_blocks(attrs: &serde_json::Value) -> Vec<RouteView> {
    attrs
        .get("route")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|r| RouteView {
                    destination_cidr_block: r
                        .get("cidr_block")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    destination_ipv6_cidr_block: r
                        .get("ipv6_cidr_block")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    gateway_id: r
                        .get("gateway_id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .or_else(|| {
                            r.get("nat_gateway_id")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string())
                        }),
                    state: "active".to_string(),
                    origin: "CreateRoute".to_string(),
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Parse route table association blocks from TF state.
fn parse_route_table_associations(attrs: &serde_json::Value) -> Vec<RouteTableAssociationView> {
    attrs
        .get("associations")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|a| RouteTableAssociationView {
                    association_id: a
                        .get("route_table_association_id")
                        .or_else(|| a.get("association_id"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    subnet_id: a
                        .get("subnet_id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    main: a.get("main").and_then(|v| v.as_bool()).unwrap_or(false),
                    state: "associated".to_string(),
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Parse security group rule blocks (ingress/egress).
fn parse_sg_rules(attrs: &serde_json::Value, key: &str) -> Vec<IpPermissionView> {
    attrs
        .get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|rule| {
                    let cidr_blocks: Vec<IpRangeView> = rule
                        .get("cidr_blocks")
                        .and_then(|v| v.as_array())
                        .map(|cidrs| {
                            cidrs
                                .iter()
                                .filter_map(|c| {
                                    c.as_str().map(|s| IpRangeView {
                                        cidr_ip: s.to_string(),
                                        description: None,
                                    })
                                })
                                .collect()
                        })
                        .unwrap_or_default();

                    let ipv6_blocks: Vec<Ipv6RangeView> = rule
                        .get("ipv6_cidr_blocks")
                        .and_then(|v| v.as_array())
                        .map(|cidrs| {
                            cidrs
                                .iter()
                                .filter_map(|c| {
                                    c.as_str().map(|s| Ipv6RangeView {
                                        cidr_ipv6: s.to_string(),
                                        description: None,
                                    })
                                })
                                .collect()
                        })
                        .unwrap_or_default();

                    let sg_refs: Vec<UserIdGroupPairView> = rule
                        .get("security_groups")
                        .and_then(|v| v.as_array())
                        .map(|sgs| {
                            sgs.iter()
                                .filter_map(|s| {
                                    s.as_str().map(|sg_id| UserIdGroupPairView {
                                        group_id: sg_id.to_string(),
                                        user_id: None,
                                    })
                                })
                                .collect()
                        })
                        .unwrap_or_default();

                    IpPermissionView {
                        from_port: rule.get("from_port").and_then(|v| v.as_i64()),
                        to_port: rule.get("to_port").and_then(|v| v.as_i64()),
                        ip_protocol: rule
                            .get("protocol")
                            .and_then(|v| v.as_str())
                            .unwrap_or("-1")
                            .to_string(),
                        ip_ranges: cidr_blocks,
                        ipv6_ranges: ipv6_blocks,
                        user_id_group_pairs: sg_refs,
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Convert an IP permission view to a JSON value for extract.
fn sg_rule_to_json(rule: &IpPermissionView) -> serde_json::Value {
    let cidr_blocks: Vec<&str> = rule.ip_ranges.iter().map(|r| r.cidr_ip.as_str()).collect();
    let ipv6_cidr_blocks: Vec<&str> = rule
        .ipv6_ranges
        .iter()
        .map(|r| r.cidr_ipv6.as_str())
        .collect();
    let security_groups: Vec<&str> = rule
        .user_id_group_pairs
        .iter()
        .map(|p| p.group_id.as_str())
        .collect();
    serde_json::json!({
        "from_port": rule.from_port.unwrap_or(0),
        "to_port": rule.to_port.unwrap_or(0),
        "protocol": rule.ip_protocol,
        "cidr_blocks": cidr_blocks,
        "ipv6_cidr_blocks": ipv6_cidr_blocks,
        "security_groups": security_groups,
    })
}

/// Parse inline `ingress`/`egress` blocks from an `aws_network_acl` resource.
fn parse_nacl_entries(
    attrs: &serde_json::Value,
    key: &str,
    egress: bool,
) -> Vec<NetworkAclEntryView> {
    attrs
        .get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|e| NetworkAclEntryView {
                    rule_number: e.get("rule_no").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                    protocol: e
                        .get("protocol")
                        .and_then(|v| v.as_str())
                        .unwrap_or("-1")
                        .to_string(),
                    rule_action: e
                        .get("action")
                        .and_then(|v| v.as_str())
                        .unwrap_or("allow")
                        .to_string(),
                    egress,
                    cidr_block: e
                        .get("cidr_block")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    ipv6_cidr_block: e
                        .get("ipv6_cidr_block")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    port_range: match (
                        e.get("from_port").and_then(|v| v.as_i64()),
                        e.get("to_port").and_then(|v| v.as_i64()),
                    ) {
                        (Some(f), Some(t)) => Some(PortRangeView {
                            from: f as i32,
                            to: t as i32,
                        }),
                        _ => None,
                    },
                    icmp_type_code: match (
                        e.get("icmp_type").and_then(|v| v.as_i64()),
                        e.get("icmp_code").and_then(|v| v.as_i64()),
                    ) {
                        (Some(t), Some(c)) => Some(IcmpTypeCodeView {
                            type_num: t as i32,
                            code: c as i32,
                        }),
                        _ => None,
                    },
                })
                .collect()
        })
        .unwrap_or_default()
}

fn nacl_entry_to_json(e: &NetworkAclEntryView) -> serde_json::Value {
    let (from_port, to_port) = match &e.port_range {
        Some(p) => (p.from, p.to),
        None => (0, 0),
    };
    let (icmp_type, icmp_code) = match &e.icmp_type_code {
        Some(ic) => (ic.type_num, ic.code),
        None => (0, 0),
    };
    serde_json::json!({
        "rule_no": e.rule_number,
        "protocol": e.protocol,
        "action": e.rule_action,
        "cidr_block": e.cidr_block.clone().unwrap_or_default(),
        "ipv6_cidr_block": e.ipv6_cidr_block.clone().unwrap_or_default(),
        "from_port": from_port,
        "to_port": to_port,
        "icmp_type": icmp_type,
        "icmp_code": icmp_code,
    })
}

fn random_short_id(prefix: &str) -> String {
    format!(
        "{}-{}",
        prefix,
        &uuid::Uuid::new_v4().to_string().replace('-', "")[..17]
    )
}

fn parse_string_array(attrs: &serde_json::Value, key: &str) -> Vec<String> {
    attrs
        .get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}
