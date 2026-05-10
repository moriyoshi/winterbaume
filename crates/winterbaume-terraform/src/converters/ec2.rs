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
    CapacityBlockView, CapacityReservationView, CustomerGatewayView, Ec2StateView,
    EgressOnlyIgwView, ElasticIpView, EoigwAttachmentView, IcmpTypeCodeView, IgwAttachmentView,
    InstanceConnectEndpointView, InternetGatewayView, IpPermissionView, IpRangeView,
    IpamOperatingRegionView, IpamPoolAllocationView, IpamPoolCidrView, IpamPoolView,
    IpamResourceDiscoveryView, IpamScopeView, IpamView, Ipv6RangeView, KeyPairView,
    LocalGatewayRouteTableVpcAssociationView, LocalGatewayRouteView, NatGatewayView,
    NetworkAclAssociationView, NetworkAclEntryView, NetworkAclView, NetworkInsightsAnalysisView,
    NetworkInsightsPathView, NetworkInterfacePermissionView, PlacementGroupView, PortRangeView,
    RouteTableAssociationView, RouteTableView, RouteView, SecurityGroupView,
    SubnetIpv6CidrAssocView, SubnetView, TrafficMirrorFilterRuleView, TrafficMirrorFilterView,
    TrafficMirrorPortRangeView, TrafficMirrorSessionView, TrafficMirrorTargetView,
    TransitGatewayConnectView, TransitGatewayMulticastDomainView, TransitGatewayPolicyTableView,
    UserIdGroupPairView, VerifiedAccessEndpointView, VerifiedAccessGroupView,
    VerifiedAccessInstanceView, VerifiedAccessSseSpecificationView,
    VerifiedAccessTrustProviderView, VpcView,
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
