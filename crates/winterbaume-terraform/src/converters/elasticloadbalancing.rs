//! Terraform converter for Classic ELB resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_elasticloadbalancing::ElasticLoadBalancingService;
use winterbaume_elasticloadbalancing::views::{
    ElbAttributesView, ElbHealthCheckView, ElbListenerView, ElbLoadBalancerView, ElbStateView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{
    extract_region, extract_tags, optional_bool, optional_i64, optional_str, require_str,
};

// ---------------------------------------------------------------------------
// aws_elb
// ---------------------------------------------------------------------------

/// Converts `aws_elb` Terraform resources to/from Classic ELB state.
pub struct AwsElbConverter {
    service: Arc<ElasticLoadBalancingService>,
}

impl AwsElbConverter {
    pub fn new(service: Arc<ElasticLoadBalancingService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsElbConverter {
    fn resource_type(&self) -> &str {
        "aws_elb"
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

fn extract_string_array(attrs: &serde_json::Value, key: &str) -> Vec<String> {
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

impl AwsElbConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_elb")?;
        let region = extract_region(attrs, &ctx.default_region);
        let _name_prefix = optional_str(attrs, "name_prefix");

        let dns_name = optional_str(attrs, "dns_name")
            .unwrap_or_else(|| format!("{}-000000000.{}.elb.amazonaws.com", name, region));
        let internal = optional_bool(attrs, "internal").unwrap_or(false);
        let scheme = if internal {
            "internal".to_string()
        } else {
            "internet-facing".to_string()
        };
        let _source_security_group = optional_str(attrs, "source_security_group");

        let availability_zones = extract_string_array(attrs, "availability_zones");
        let subnets = extract_string_array(attrs, "subnets");
        let security_groups = extract_string_array(attrs, "security_groups");
        let instances = extract_string_array(attrs, "instances");

        // Parse listeners
        let listeners = attrs
            .get("listener")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|l| ElbListenerView {
                        load_balancer_port: l.get("lb_port").and_then(|v| v.as_i64()).unwrap_or(80)
                            as i32,
                        instance_port: l
                            .get("instance_port")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(80) as i32,
                        protocol: l
                            .get("lb_protocol")
                            .and_then(|v| v.as_str())
                            .unwrap_or("HTTP")
                            .to_string(),
                        instance_protocol: l
                            .get("instance_protocol")
                            .and_then(|v| v.as_str())
                            .unwrap_or("HTTP")
                            .to_string(),
                        ssl_certificate_id: l
                            .get("ssl_certificate_id")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        policy_names: vec![],
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Parse health check
        let hc = attrs
            .get("health_check")
            .and_then(|v| v.as_array().and_then(|a| a.first()).or(Some(v)));
        let health_check = if let Some(hc) = hc {
            ElbHealthCheckView {
                target: hc
                    .get("target")
                    .and_then(|v| v.as_str())
                    .unwrap_or("TCP:80")
                    .to_string(),
                interval: hc.get("interval").and_then(|v| v.as_i64()).unwrap_or(30) as i32,
                timeout: hc.get("timeout").and_then(|v| v.as_i64()).unwrap_or(5) as i32,
                unhealthy_threshold: hc
                    .get("unhealthy_threshold")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(2) as i32,
                healthy_threshold: hc
                    .get("healthy_threshold")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(10) as i32,
            }
        } else {
            ElbHealthCheckView {
                target: "TCP:80".to_string(),
                interval: 30,
                timeout: 5,
                unhealthy_threshold: 2,
                healthy_threshold: 10,
            }
        };

        // Parse access_logs block if present
        let access_log_block = attrs
            .get("access_logs")
            .and_then(|v| v.as_array().and_then(|a| a.first()).or(Some(v)));
        let access_log_enabled = access_log_block
            .and_then(|al| al.get("enabled"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let access_log_s3_bucket = access_log_block
            .and_then(|al| al.get("bucket"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let access_log_s3_prefix = access_log_block
            .and_then(|al| al.get("bucket_prefix"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let access_log_interval = access_log_block
            .and_then(|al| al.get("interval"))
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let cross_zone = optional_bool(attrs, "cross_zone_load_balancing").unwrap_or(true);
        let idle_timeout = optional_i64(attrs, "idle_timeout").unwrap_or(60) as i32;
        let connection_draining = optional_bool(attrs, "connection_draining").unwrap_or(false);
        let connection_draining_timeout =
            optional_i64(attrs, "connection_draining_timeout").map(|v| v as i32);

        let elb_attributes = ElbAttributesView {
            cross_zone_load_balancing_enabled: cross_zone,
            access_log_enabled,
            access_log_emit_interval: access_log_interval,
            access_log_s3_bucket_name: access_log_s3_bucket,
            access_log_s3_bucket_prefix: access_log_s3_prefix,
            connection_draining_enabled: connection_draining,
            connection_draining_timeout,
            connection_idle_timeout: idle_timeout,
        };

        let mut tags_map = extract_tags(attrs);
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags_map.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let lb_view = ElbLoadBalancerView {
            name: name.to_string(),
            dns_name,
            scheme,
            availability_zones,
            subnets,
            security_groups,
            instances,
            listeners,
            health_check,
            tags: tags_map,
            policies: vec![],
            attributes: elb_attributes,
            vpc_id: None,
        };

        let mut state_view = ElbStateView {
            load_balancers: HashMap::new(),
        };
        state_view.load_balancers.insert(name.to_string(), lb_view);
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
        for lb in view.load_balancers.values() {
            let listeners: Vec<serde_json::Value> = lb
                .listeners
                .iter()
                .map(|l| {
                    serde_json::json!({
                        "lb_port": l.load_balancer_port,
                        "instance_port": l.instance_port,
                        "lb_protocol": l.protocol,
                        "instance_protocol": l.instance_protocol,
                        "ssl_certificate_id": l.ssl_certificate_id,
                    })
                })
                .collect();
            let attrs = serde_json::json!({
                "id": lb.name,
                "name": lb.name,
                "dns_name": lb.dns_name,
                "internal": lb.scheme == "internal",
                "availability_zones": lb.availability_zones,
                "subnets": lb.subnets,
                "security_groups": lb.security_groups,
                "instances": lb.instances,
                "listener": listeners,
                "health_check": [{
                    "target": lb.health_check.target,
                    "interval": lb.health_check.interval,
                    "timeout": lb.health_check.timeout,
                    "unhealthy_threshold": lb.health_check.unhealthy_threshold,
                    "healthy_threshold": lb.health_check.healthy_threshold,
                }],
                "cross_zone_load_balancing": lb.attributes.cross_zone_load_balancing_enabled,
                "idle_timeout": lb.attributes.connection_idle_timeout,
                "connection_draining": lb.attributes.connection_draining_enabled,
                "connection_draining_timeout": lb.attributes.connection_draining_timeout,
                "tags": lb.tags,
                "tags_all": lb.tags,
                "source_security_group": lb.security_groups.first().cloned().unwrap_or_default(),
            });
            results.push(ExtractedResource {
                name: lb.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
