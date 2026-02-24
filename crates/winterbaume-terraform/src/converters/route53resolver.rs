//! Terraform converter for Route53 Resolver resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_route53resolver::Route53ResolverService;
use winterbaume_route53resolver::views::{
    IpAddressEntryView, ResolverEndpointView, ResolverRuleView, Route53ResolverStateView,
    TargetAddressView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_route53_resolver_endpoint
// ---------------------------------------------------------------------------

/// Converts `aws_route53_resolver_endpoint` Terraform resources.
pub struct AwsRoute53ResolverEndpointConverter {
    service: Arc<Route53ResolverService>,
}

impl AwsRoute53ResolverEndpointConverter {
    pub fn new(service: Arc<Route53ResolverService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53ResolverEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_resolver_endpoint"
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

impl AwsRoute53ResolverEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = optional_str(attrs, "name").unwrap_or_default();
        let direction = require_str(attrs, "direction", "aws_route53_resolver_endpoint")?;
        let region = extract_region(attrs, &ctx.default_region);

        let id = optional_str(attrs, "id")
            .unwrap_or_else(|| format!("rslvr-{}", uuid::Uuid::new_v4().simple()));
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:route53resolver:{}:{}:resolver-endpoint/{}",
                region, ctx.default_account_id, id
            )
        });

        // Parse security_group_ids
        let security_group_ids: Vec<String> = attrs
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let host_vpc_id = optional_str(attrs, "host_vpc_id").unwrap_or_default();
        let resolver_endpoint_type =
            optional_str(attrs, "resolver_endpoint_type").unwrap_or_else(|| "IPV4".to_string());
        let now = chrono::Utc::now().to_rfc3339();

        // Parse protocols
        let protocols: Vec<String> = attrs
            .get("protocols")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_else(|| vec!["Do53".to_string()]);

        // Parse ip_address blocks
        let ip_addresses: Vec<IpAddressEntryView> = attrs
            .get("ip_address")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .enumerate()
                    .map(|(i, ip)| {
                        let subnet_id = ip
                            .get("subnet_id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let ip_val = ip.get("ip").and_then(|v| v.as_str()).map(|s| s.to_string());
                        IpAddressEntryView {
                            ip_id: ip
                                .get("ip_id")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| format!("rslvr-ip-{}", i)),
                            subnet_id,
                            ip: ip_val,
                            status: "ATTACHED".to_string(),
                            status_message: "".to_string(),
                            creation_time: now.clone(),
                            modification_time: now.clone(),
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        let ip_address_count = ip_addresses.len() as i32;

        let ep_view = ResolverEndpointView {
            id: id.clone(),
            arn,
            name,
            security_group_ids,
            direction: direction.to_string(),
            ip_address_count,
            host_vpc_id,
            status: "OPERATIONAL".to_string(),
            status_message: "".to_string(),
            creation_time: now.clone(),
            modification_time: now,
            creator_request_id: "".to_string(),
            protocols,
            resolver_endpoint_type,
            ip_addresses,
            tags: extract_tags(attrs),
        };

        let mut state_view = Route53ResolverStateView::default();
        state_view.endpoints.insert(id, ep_view);
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
        for ep in view.endpoints.values() {
            let ip_addrs: Vec<serde_json::Value> = ep
                .ip_addresses
                .iter()
                .map(|ip| {
                    serde_json::json!({
                        "ip_id": ip.ip_id,
                        "subnet_id": ip.subnet_id,
                        "ip": ip.ip,
                    })
                })
                .collect();
            let attrs = serde_json::json!({
                "id": ep.id,
                "arn": ep.arn,
                "name": ep.name,
                "direction": ep.direction,
                "security_group_ids": ep.security_group_ids,
                "ip_address_count": ep.ip_address_count,
                "host_vpc_id": ep.host_vpc_id,
                "status": ep.status,
                "status_message": ep.status_message,
                "protocols": ep.protocols,
                "resolver_endpoint_type": ep.resolver_endpoint_type,
                "ip_address": ip_addrs,
                "tags": ep.tags,
            });
            results.push(ExtractedResource {
                name: ep.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_resolver_rule
// ---------------------------------------------------------------------------

/// Converts `aws_route53_resolver_rule` Terraform resources.
pub struct AwsRoute53ResolverRuleConverter {
    service: Arc<Route53ResolverService>,
}

impl AwsRoute53ResolverRuleConverter {
    pub fn new(service: Arc<Route53ResolverService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53ResolverRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_resolver_rule"
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

impl AwsRoute53ResolverRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let domain_name = require_str(attrs, "domain_name", "aws_route53_resolver_rule")?;
        let rule_type = require_str(attrs, "rule_type", "aws_route53_resolver_rule")?;
        let region = extract_region(attrs, &ctx.default_region);

        let id = optional_str(attrs, "id")
            .unwrap_or_else(|| format!("rslvr-rr-{}", uuid::Uuid::new_v4().simple()));
        let name = optional_str(attrs, "name").unwrap_or_default();
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:route53resolver:{}:{}:resolver-rule/{}",
                region, ctx.default_account_id, id
            )
        });
        let resolver_endpoint_id = optional_str(attrs, "resolver_endpoint_id");
        let now = chrono::Utc::now().to_rfc3339();

        // Parse target_ip blocks
        let target_ips: Vec<TargetAddressView> = attrs
            .get("target_ip")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|t| TargetAddressView {
                        ip: t.get("ip").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        ipv6: t
                            .get("ipv6")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        port: t.get("port").and_then(|v| v.as_i64()).map(|p| p as i32),
                        protocol: t
                            .get("protocol")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let rule_view = ResolverRuleView {
            id: id.clone(),
            arn,
            name,
            domain_name: domain_name.to_string(),
            rule_type: rule_type.to_string(),
            resolver_endpoint_id,
            target_ips,
            status: "COMPLETE".to_string(),
            status_message: "".to_string(),
            owner_id: ctx.default_account_id.clone(),
            share_status: "NOT_SHARED".to_string(),
            creator_request_id: "".to_string(),
            creation_time: now.clone(),
            modification_time: now,
            tags: extract_tags(attrs),
        };

        let mut state_view = Route53ResolverStateView::default();
        state_view.resolver_rules.insert(id, rule_view);
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
        for rule in view.resolver_rules.values() {
            let target_ips: Vec<serde_json::Value> = rule
                .target_ips
                .iter()
                .map(|t| {
                    serde_json::json!({
                        "ip": t.ip,
                        "ipv6": t.ipv6,
                        "port": t.port,
                        "protocol": t.protocol,
                    })
                })
                .collect();
            let attrs = serde_json::json!({
                "id": rule.id,
                "arn": rule.arn,
                "name": rule.name,
                "domain_name": rule.domain_name,
                "rule_type": rule.rule_type,
                "resolver_endpoint_id": rule.resolver_endpoint_id,
                "target_ip": target_ips,
                "status": rule.status,
                "owner_id": rule.owner_id,
                "share_status": rule.share_status,
                "tags": rule.tags,
            });
            results.push(ExtractedResource {
                name: rule.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
