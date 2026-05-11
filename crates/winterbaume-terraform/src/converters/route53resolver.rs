//! Terraform converter for Route53 Resolver resources.
//!
//! Full state-backed converters: endpoint, rule, rule_association,
//! query_log_config, query_log_config_association, dnssec_config.
//!
//! Warning-only converters (validate-and-no-op; `winterbaume_route53resolver`
//! does not yet model these resource families): config, firewall_config,
//! firewall_domain_list, firewall_rule, firewall_rule_group,
//! firewall_rule_group_association.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_route53resolver::Route53ResolverService;
use winterbaume_route53resolver::views::{
    IpAddressEntryView, ResolverDnssecConfigView, ResolverEndpointView,
    ResolverQueryLogConfigAssociationView, ResolverQueryLogConfigView, ResolverRuleAssociationView,
    ResolverRuleView, Route53ResolverStateView, TargetAddressView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::route53resolver as route53resolver_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags};

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
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53resolver_gen::ResolverEndpointTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_resolver_endpoint", e))?;

        let name = model.name.unwrap_or_default();
        let direction = model.direction.clone();

        let id = model
            .id
            .unwrap_or_else(|| format!("rslvr-{}", uuid::Uuid::new_v4().simple()));
        let arn = model.arn.unwrap_or_else(|| {
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

        let host_vpc_id = model.host_vpc_id.unwrap_or_default();
        let resolver_endpoint_type = model
            .resolver_endpoint_type
            .unwrap_or_else(|| "IPV4".to_string());
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
            direction,
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

        let state_view = Route53ResolverStateView {
            endpoints: std::iter::once((id, ep_view)).collect(),
            ..Default::default()
        };
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
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53resolver_gen::ResolverRuleTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_resolver_rule", e))?;

        let domain_name = model.domain_name.clone();
        let rule_type = model.rule_type.clone();

        let id = model
            .id
            .unwrap_or_else(|| format!("rslvr-rr-{}", uuid::Uuid::new_v4().simple()));
        let name = model.name.unwrap_or_default();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:route53resolver:{}:{}:resolver-rule/{}",
                region, ctx.default_account_id, id
            )
        });
        let resolver_endpoint_id = model.resolver_endpoint_id;
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
            domain_name,
            rule_type,
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

        let state_view = Route53ResolverStateView {
            resolver_rules: std::iter::once((id, rule_view)).collect(),
            ..Default::default()
        };
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

// ---------------------------------------------------------------------------
// aws_route53_resolver_rule_association
// ---------------------------------------------------------------------------

/// Converts `aws_route53_resolver_rule_association` Terraform resources.
pub struct AwsRoute53ResolverRuleAssociationConverter {
    service: Arc<Route53ResolverService>,
}

impl AwsRoute53ResolverRuleAssociationConverter {
    pub fn new(service: Arc<Route53ResolverService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53ResolverRuleAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_resolver_rule_association"
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

impl AwsRoute53ResolverRuleAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53resolver_gen::ResolverRuleAssociationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_route53_resolver_rule_association", e)
            })?;

        let id = model
            .id
            .unwrap_or_else(|| format!("rslvr-rrassoc-{}", uuid::Uuid::new_v4().simple()));
        let name = model.name.unwrap_or_default();

        let assoc_view = ResolverRuleAssociationView {
            id: id.clone(),
            resolver_rule_id: model.resolver_rule_id,
            name,
            vpc_id: model.vpc_id,
            status: "COMPLETE".to_string(),
            status_message: "".to_string(),
        };

        let state_view = Route53ResolverStateView {
            rule_associations: std::iter::once((id, assoc_view)).collect(),
            ..Default::default()
        };
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
        for assoc in view.rule_associations.values() {
            let attrs = serde_json::json!({
                "id": assoc.id,
                "resolver_rule_id": assoc.resolver_rule_id,
                "name": assoc.name,
                "vpc_id": assoc.vpc_id,
                "status": assoc.status,
                "status_message": assoc.status_message,
            });
            results.push(ExtractedResource {
                name: assoc.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_resolver_query_log_config
// ---------------------------------------------------------------------------

/// Converts `aws_route53_resolver_query_log_config` Terraform resources.
pub struct AwsRoute53ResolverQueryLogConfigConverter {
    service: Arc<Route53ResolverService>,
}

impl AwsRoute53ResolverQueryLogConfigConverter {
    pub fn new(service: Arc<Route53ResolverService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53ResolverQueryLogConfigConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_resolver_query_log_config"
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

impl AwsRoute53ResolverQueryLogConfigConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53resolver_gen::ResolverQueryLogConfigTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_route53_resolver_query_log_config", e)
            })?;

        let id = model
            .id
            .unwrap_or_else(|| format!("rqlc-{}", uuid::Uuid::new_v4().simple()));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:route53resolver:{}:{}:resolver-query-log-config/{}",
                region, ctx.default_account_id, id
            )
        });
        let now = chrono::Utc::now().to_rfc3339();

        let config_view = ResolverQueryLogConfigView {
            id: id.clone(),
            arn,
            name: model.name,
            destination_arn: model.destination_arn,
            owner_id: ctx.default_account_id.clone(),
            status: "CREATED".to_string(),
            share_status: "NOT_SHARED".to_string(),
            association_count: 0,
            creator_request_id: "".to_string(),
            creation_time: now,
            tags: extract_tags(attrs),
        };

        let state_view = Route53ResolverStateView {
            query_log_configs: std::iter::once((id, config_view)).collect(),
            ..Default::default()
        };
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
        for c in view.query_log_configs.values() {
            let attrs = serde_json::json!({
                "id": c.id,
                "arn": c.arn,
                "name": c.name,
                "destination_arn": c.destination_arn,
                "owner_id": c.owner_id,
                "status": c.status,
                "share_status": c.share_status,
                "association_count": c.association_count,
                "tags": c.tags,
            });
            results.push(ExtractedResource {
                name: c.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_resolver_query_log_config_association
// ---------------------------------------------------------------------------

/// Converts `aws_route53_resolver_query_log_config_association` Terraform resources.
pub struct AwsRoute53ResolverQueryLogConfigAssociationConverter {
    service: Arc<Route53ResolverService>,
}

impl AwsRoute53ResolverQueryLogConfigAssociationConverter {
    pub fn new(service: Arc<Route53ResolverService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53ResolverQueryLogConfigAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_resolver_query_log_config_association"
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

impl AwsRoute53ResolverQueryLogConfigAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53resolver_gen::ResolverQueryLogConfigAssociationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_route53_resolver_query_log_config_association", e)
            })?;

        let id = model
            .id
            .unwrap_or_else(|| format!("rqlca-{}", uuid::Uuid::new_v4().simple()));
        let now = chrono::Utc::now().to_rfc3339();

        let assoc_view = ResolverQueryLogConfigAssociationView {
            id: id.clone(),
            resolver_query_log_config_id: model.resolver_query_log_config_id,
            resource_id: model.resource_id,
            status: "ACTIVE".to_string(),
            error: "NONE".to_string(),
            error_message: "".to_string(),
            creation_time: now,
        };

        let state_view = Route53ResolverStateView {
            query_log_config_associations: std::iter::once((id, assoc_view)).collect(),
            ..Default::default()
        };
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
        for a in view.query_log_config_associations.values() {
            let attrs = serde_json::json!({
                "id": a.id,
                "resolver_query_log_config_id": a.resolver_query_log_config_id,
                "resource_id": a.resource_id,
                "status": a.status,
                "error": a.error,
                "error_message": a.error_message,
            });
            results.push(ExtractedResource {
                name: a.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_resolver_dnssec_config
// ---------------------------------------------------------------------------

/// Converts `aws_route53_resolver_dnssec_config` Terraform resources.
pub struct AwsRoute53ResolverDnssecConfigConverter {
    service: Arc<Route53ResolverService>,
}

impl AwsRoute53ResolverDnssecConfigConverter {
    pub fn new(service: Arc<Route53ResolverService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53ResolverDnssecConfigConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_resolver_dnssec_config"
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

impl AwsRoute53ResolverDnssecConfigConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: route53resolver_gen::ResolverDnssecConfigTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_resolver_dnssec_config", e))?;

        let id = model
            .id
            .unwrap_or_else(|| format!("rdsc-{}", uuid::Uuid::new_v4().simple()));
        let owner_id = model
            .owner_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let validation_status = model
            .validation_status
            .unwrap_or_else(|| "ENABLED".to_string());

        let cfg_view = ResolverDnssecConfigView {
            id: id.clone(),
            owner_id,
            resource_id: model.resource_id,
            validation_status,
        };

        let state_view = Route53ResolverStateView {
            dnssec_configs: std::iter::once((id, cfg_view)).collect(),
            ..Default::default()
        };
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
        for c in view.dnssec_configs.values() {
            let attrs = serde_json::json!({
                "id": c.id,
                "owner_id": c.owner_id,
                "resource_id": c.resource_id,
                "validation_status": c.validation_status,
            });
            results.push(ExtractedResource {
                name: c.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_route53_resolver_config (warning-only)
//
// `winterbaume_route53resolver` does not yet model per-VPC resolver
// configuration (autodefined-reverse flag). Inject validates the TF
// attributes via the generated TfModel and emits a warning. Extract
// returns an empty list.
// ---------------------------------------------------------------------------

/// Converts `aws_route53_resolver_config` Terraform resources (validation-only; no state slot).
pub struct AwsRoute53ResolverConfigConverter {
    #[allow(dead_code)]
    service: Arc<Route53ResolverService>,
}

impl AwsRoute53ResolverConfigConverter {
    pub fn new(service: Arc<Route53ResolverService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53ResolverConfigConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_resolver_config"
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

impl AwsRoute53ResolverConfigConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: route53resolver_gen::ResolverConfigTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_resolver_config", e))?;
        let warn_msg =
            "no state slot in winterbaume_route53resolver for VPC resolver configs; inject is a no-op"
                .to_string();
        eprintln!("warning: aws_route53_resolver_config: {}", warn_msg);
        Ok(ConversionResult {
            region,
            warnings: vec![format!("aws_route53_resolver_config: {}", warn_msg)],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_route53_resolver_firewall_config (warning-only)
// ---------------------------------------------------------------------------

/// Converts `aws_route53_resolver_firewall_config` Terraform resources (validation-only; no state slot).
pub struct AwsRoute53ResolverFirewallConfigConverter {
    #[allow(dead_code)]
    service: Arc<Route53ResolverService>,
}

impl AwsRoute53ResolverFirewallConfigConverter {
    pub fn new(service: Arc<Route53ResolverService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53ResolverFirewallConfigConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_resolver_firewall_config"
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

impl AwsRoute53ResolverFirewallConfigConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: route53resolver_gen::ResolverFirewallConfigTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_route53_resolver_firewall_config", e)
            })?;
        let warn_msg =
            "no state slot in winterbaume_route53resolver for DNS firewall configs; inject is a no-op"
                .to_string();
        eprintln!(
            "warning: aws_route53_resolver_firewall_config: {}",
            warn_msg
        );
        Ok(ConversionResult {
            region,
            warnings: vec![format!(
                "aws_route53_resolver_firewall_config: {}",
                warn_msg
            )],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_route53_resolver_firewall_domain_list (warning-only)
// ---------------------------------------------------------------------------

/// Converts `aws_route53_resolver_firewall_domain_list` Terraform resources (validation-only; no state slot).
pub struct AwsRoute53ResolverFirewallDomainListConverter {
    #[allow(dead_code)]
    service: Arc<Route53ResolverService>,
}

impl AwsRoute53ResolverFirewallDomainListConverter {
    pub fn new(service: Arc<Route53ResolverService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53ResolverFirewallDomainListConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_resolver_firewall_domain_list"
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

impl AwsRoute53ResolverFirewallDomainListConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: route53resolver_gen::ResolverFirewallDomainListTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_route53_resolver_firewall_domain_list", e)
            })?;
        let warn_msg =
            "no state slot in winterbaume_route53resolver for DNS firewall domain lists; inject is a no-op"
                .to_string();
        eprintln!(
            "warning: aws_route53_resolver_firewall_domain_list: {}",
            warn_msg
        );
        Ok(ConversionResult {
            region,
            warnings: vec![format!(
                "aws_route53_resolver_firewall_domain_list: {}",
                warn_msg
            )],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_route53_resolver_firewall_rule (warning-only)
// ---------------------------------------------------------------------------

/// Converts `aws_route53_resolver_firewall_rule` Terraform resources (validation-only; no state slot).
pub struct AwsRoute53ResolverFirewallRuleConverter {
    #[allow(dead_code)]
    service: Arc<Route53ResolverService>,
}

impl AwsRoute53ResolverFirewallRuleConverter {
    pub fn new(service: Arc<Route53ResolverService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53ResolverFirewallRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_resolver_firewall_rule"
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

impl AwsRoute53ResolverFirewallRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: route53resolver_gen::ResolverFirewallRuleTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_route53_resolver_firewall_rule", e))?;
        let warn_msg =
            "no state slot in winterbaume_route53resolver for DNS firewall rules; inject is a no-op"
                .to_string();
        eprintln!("warning: aws_route53_resolver_firewall_rule: {}", warn_msg);
        Ok(ConversionResult {
            region,
            warnings: vec![format!("aws_route53_resolver_firewall_rule: {}", warn_msg)],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_route53_resolver_firewall_rule_group (warning-only)
// ---------------------------------------------------------------------------

/// Converts `aws_route53_resolver_firewall_rule_group` Terraform resources (validation-only; no state slot).
pub struct AwsRoute53ResolverFirewallRuleGroupConverter {
    #[allow(dead_code)]
    service: Arc<Route53ResolverService>,
}

impl AwsRoute53ResolverFirewallRuleGroupConverter {
    pub fn new(service: Arc<Route53ResolverService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53ResolverFirewallRuleGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_resolver_firewall_rule_group"
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

impl AwsRoute53ResolverFirewallRuleGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: route53resolver_gen::ResolverFirewallRuleGroupTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_route53_resolver_firewall_rule_group", e)
            })?;
        let warn_msg =
            "no state slot in winterbaume_route53resolver for DNS firewall rule groups; inject is a no-op"
                .to_string();
        eprintln!(
            "warning: aws_route53_resolver_firewall_rule_group: {}",
            warn_msg
        );
        Ok(ConversionResult {
            region,
            warnings: vec![format!(
                "aws_route53_resolver_firewall_rule_group: {}",
                warn_msg
            )],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_route53_resolver_firewall_rule_group_association (warning-only)
// ---------------------------------------------------------------------------

/// Converts `aws_route53_resolver_firewall_rule_group_association` Terraform resources (validation-only; no state slot).
pub struct AwsRoute53ResolverFirewallRuleGroupAssociationConverter {
    #[allow(dead_code)]
    service: Arc<Route53ResolverService>,
}

impl AwsRoute53ResolverFirewallRuleGroupAssociationConverter {
    pub fn new(service: Arc<Route53ResolverService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53ResolverFirewallRuleGroupAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_route53_resolver_firewall_rule_group_association"
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

impl AwsRoute53ResolverFirewallRuleGroupAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: route53resolver_gen::ResolverFirewallRuleGroupAssociationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error(
                    "aws_route53_resolver_firewall_rule_group_association",
                    e,
                )
            })?;
        let warn_msg =
            "no state slot in winterbaume_route53resolver for DNS firewall rule group associations; inject is a no-op"
                .to_string();
        eprintln!(
            "warning: aws_route53_resolver_firewall_rule_group_association: {}",
            warn_msg
        );
        Ok(ConversionResult {
            region,
            warnings: vec![format!(
                "aws_route53_resolver_firewall_rule_group_association: {}",
                warn_msg
            )],
        })
    }
}
