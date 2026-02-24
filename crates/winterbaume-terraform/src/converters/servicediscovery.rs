//! Terraform converter for Service Discovery resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_servicediscovery::ServiceDiscoveryService;
use winterbaume_servicediscovery::views::{
    DnsConfigEntryView, DnsRecordEntryView, HealthCheckConfigEntryView,
    HealthCheckCustomConfigEntryView, NamespaceView, ServiceDiscoveryStateView, ServiceEntryView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_i64, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_service_discovery_private_dns_namespace
// ---------------------------------------------------------------------------

/// Converts `aws_service_discovery_private_dns_namespace` Terraform resources.
pub struct AwsServiceDiscoveryPrivateDnsNamespaceConverter {
    service: Arc<ServiceDiscoveryService>,
}

impl AwsServiceDiscoveryPrivateDnsNamespaceConverter {
    pub fn new(service: Arc<ServiceDiscoveryService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsServiceDiscoveryPrivateDnsNamespaceConverter {
    fn resource_type(&self) -> &str {
        "aws_service_discovery_private_dns_namespace"
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

impl AwsServiceDiscoveryPrivateDnsNamespaceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_service_discovery_private_dns_namespace")?;
        let region = extract_region(attrs, &ctx.default_region);

        let id = optional_str(attrs, "id")
            .unwrap_or_else(|| format!("ns-{}", uuid::Uuid::new_v4().simple()));
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:servicediscovery:{}:{}:namespace/{}",
                region, ctx.default_account_id, id
            )
        });
        let description = optional_str(attrs, "description");
        let vpc = optional_str(attrs, "vpc");
        let hosted_zone_id = optional_str(attrs, "hosted_zone_id");

        let ns_view = NamespaceView {
            id: id.clone(),
            arn,
            name: name.to_string(),
            namespace_type: "DNS_PRIVATE".to_string(),
            description,
            creator_request_id: None,
            vpc,
            hosted_zone_id,
            soa_ttl: optional_i64(attrs, "soa_ttl"),
            service_count: 0,
            create_date: chrono::Utc::now().to_rfc3339(),
            tags: extract_tags(attrs),
        };

        let mut state_view = ServiceDiscoveryStateView {
            namespaces: HashMap::new(),
            services: HashMap::new(),
            instances: HashMap::new(),
            operations: HashMap::new(),
            instances_revision: 0,
        };
        state_view.namespaces.insert(id, ns_view);
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
        for ns in view.namespaces.values() {
            if ns.namespace_type != "DNS_PRIVATE" {
                continue;
            }
            let attrs = serde_json::json!({
                "id": ns.id,
                "name": ns.name,
                "arn": ns.arn,
                "description": ns.description,
                "vpc": ns.vpc,
                "hosted_zone_id": ns.hosted_zone_id,
                "soa_ttl": ns.soa_ttl,
                "tags": ns.tags,
            });
            results.push(ExtractedResource {
                name: ns.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_service_discovery_service
// ---------------------------------------------------------------------------

/// Converts `aws_service_discovery_service` Terraform resources.
pub struct AwsServiceDiscoveryServiceConverter {
    service: Arc<ServiceDiscoveryService>,
}

impl AwsServiceDiscoveryServiceConverter {
    pub fn new(service: Arc<ServiceDiscoveryService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsServiceDiscoveryServiceConverter {
    fn resource_type(&self) -> &str {
        "aws_service_discovery_service"
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

impl AwsServiceDiscoveryServiceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_service_discovery_service")?;
        let region = extract_region(attrs, &ctx.default_region);

        let id = optional_str(attrs, "id")
            .unwrap_or_else(|| format!("srv-{}", uuid::Uuid::new_v4().simple()));
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:servicediscovery:{}:{}:service/{}",
                region, ctx.default_account_id, id
            )
        });
        let namespace_id = optional_str(attrs, "namespace_id").unwrap_or_default();
        let description = optional_str(attrs, "description");

        // Parse dns_config
        let dns_config = attrs.get("dns_config").and_then(|v| {
            // Terraform represents dns_config as a list with one element
            let cfg = if v.is_array() {
                v.as_array().and_then(|a| a.first())
            } else {
                Some(v)
            };
            cfg.map(|c| {
                let dns_records = c
                    .get("dns_records")
                    .and_then(|r| r.as_array())
                    .map(|records| {
                        records
                            .iter()
                            .map(|r| DnsRecordEntryView {
                                record_type: r
                                    .get("type")
                                    .and_then(|t| t.as_str())
                                    .unwrap_or("A")
                                    .to_string(),
                                ttl: r.get("ttl").and_then(|t| t.as_i64()).unwrap_or(60),
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                DnsConfigEntryView {
                    namespace_id: c
                        .get("namespace_id")
                        .and_then(|n| n.as_str())
                        .map(|s| s.to_string()),
                    routing_policy: c
                        .get("routing_policy")
                        .and_then(|r| r.as_str())
                        .map(|s| s.to_string()),
                    dns_records,
                }
            })
        });

        // Parse health_check_config
        let health_check_config = attrs.get("health_check_config").and_then(|v| {
            let cfg = if v.is_array() {
                v.as_array().and_then(|a| a.first())
            } else {
                Some(v)
            };
            cfg.map(|c| HealthCheckConfigEntryView {
                check_type: c
                    .get("type")
                    .and_then(|t| t.as_str())
                    .unwrap_or("HTTP")
                    .to_string(),
                resource_path: c
                    .get("resource_path")
                    .and_then(|r| r.as_str())
                    .map(|s| s.to_string()),
                failure_threshold: c
                    .get("failure_threshold")
                    .and_then(|f| f.as_i64())
                    .map(|f| f as i32),
            })
        });

        // Parse health_check_custom_config
        let health_check_custom_config = attrs.get("health_check_custom_config").and_then(|v| {
            let cfg = if v.is_array() {
                v.as_array().and_then(|a| a.first())
            } else {
                Some(v)
            };
            cfg.map(|c| HealthCheckCustomConfigEntryView {
                failure_threshold: c
                    .get("failure_threshold")
                    .and_then(|f| f.as_i64())
                    .map(|f| f as i32),
            })
        });

        let svc_view = ServiceEntryView {
            id: id.clone(),
            arn,
            name: name.to_string(),
            namespace_id,
            description,
            creator_request_id: None,
            dns_config,
            health_check_config,
            health_check_custom_config,
            instance_count: 0,
            create_date: chrono::Utc::now().to_rfc3339(),
            tags: extract_tags(attrs),
            service_type: optional_str(attrs, "type"),
            include_namespace_id_in_response: false,
        };

        let mut state_view = ServiceDiscoveryStateView {
            namespaces: HashMap::new(),
            services: HashMap::new(),
            instances: HashMap::new(),
            operations: HashMap::new(),
            instances_revision: 0,
        };
        state_view.services.insert(id, svc_view);
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
        for svc in view.services.values() {
            let dns_config_json = svc.dns_config.as_ref().map(|d| {
                serde_json::json!({
                    "namespace_id": d.namespace_id,
                    "routing_policy": d.routing_policy,
                    "dns_records": d.dns_records.iter().map(|r| serde_json::json!({
                        "type": r.record_type,
                        "ttl": r.ttl,
                    })).collect::<Vec<_>>(),
                })
            });
            let health_check_config_json = svc.health_check_config.as_ref().map(|h| {
                serde_json::json!({
                    "type": h.check_type,
                    "resource_path": h.resource_path,
                    "failure_threshold": h.failure_threshold,
                })
            });
            let health_check_custom_config_json =
                svc.health_check_custom_config.as_ref().map(|h| {
                    serde_json::json!({
                        "failure_threshold": h.failure_threshold,
                    })
                });

            let attrs = serde_json::json!({
                "id": svc.id,
                "name": svc.name,
                "arn": svc.arn,
                "namespace_id": svc.namespace_id,
                "description": svc.description,
                "dns_config": dns_config_json,
                "health_check_config": health_check_config_json,
                "health_check_custom_config": health_check_custom_config_json,
                "instance_count": svc.instance_count,
                "tags": svc.tags,
                "type": svc.service_type,
            });
            results.push(ExtractedResource {
                name: svc.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
