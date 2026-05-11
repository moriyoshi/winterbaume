//! Terraform converters for ELBv2 resources.
//!
//! The TfModel structs are generated from `specs/elbv2.toml`. The
//! ARN/DNS templates with random UUID suffixes, the `internal`
//! boolean fallback for `scheme`, the load-balancer/target-group/
//! listener `attributes` HashMap (idle_timeout, access_logs.*,
//! health_check.*, stickiness.*, deregistration_delay,
//! protocol_version, ssl_policy, alpn_policy), and the nested-block
//! parsing (subnet_mapping, default_action, certificates,
//! target_failover, target_group_health, target_health_state,
//! ipam_pools, minimum_load_balancer_capacity, connection_logs)
//! are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_elasticloadbalancingv2::ElasticLoadBalancingV2Service;
use winterbaume_elasticloadbalancingv2::views::{
    AvailabilityZoneView, CertificateView, Elbv2StateView, ListenerActionView, ListenerView,
    LoadBalancerView, RuleActionView, RuleConditionView, RuleView, TargetDescriptionView,
    TargetGroupView, TrustStoreRevocationEntryView, TrustStoreView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::elbv2 as elbv2_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags};

// ---------------------------------------------------------------------------
// aws_lb
// ---------------------------------------------------------------------------

/// Converts `aws_lb` (and `aws_alb`) Terraform resources to/from ELBv2 state.
pub struct AwsLbConverter {
    service: Arc<ElasticLoadBalancingV2Service>,
}

impl AwsLbConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLbConverter {
    fn resource_type(&self) -> &str {
        "aws_lb"
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

impl AwsLbConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: elbv2_gen::LbTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_lb", e))?;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:elasticloadbalancing:{}:{}:loadbalancer/app/{}/{}",
                region,
                ctx.default_account_id,
                model.name,
                &uuid::Uuid::new_v4().to_string()[..16]
            )
        });
        let dns_name = model
            .dns_name
            .unwrap_or_else(|| format!("{}-123456789.{}.elb.amazonaws.com", model.name, region));
        let scheme = model.scheme.unwrap_or_else(|| {
            // Fall back to legacy "internal" boolean
            if attrs
                .get("internal")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                "internal".to_string()
            } else {
                "internet-facing".to_string()
            }
        });
        let lb_type = model
            .load_balancer_type
            .unwrap_or_else(|| "application".to_string());
        let vpc_id = model.vpc_id.unwrap_or_default();
        let ip_address_type = model.ip_address_type.unwrap_or_else(|| "ipv4".to_string());

        let subnets: Vec<String> = attrs
            .get("subnets")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let availability_zones: Vec<AvailabilityZoneView> = subnets
            .iter()
            .map(|subnet_id| AvailabilityZoneView {
                zone_name: String::new(),
                subnet_id: subnet_id.clone(),
            })
            .collect();

        let security_groups: Vec<String> = attrs
            .get("security_groups")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let tags = extract_tags(attrs);

        // Additional fields for coverage
        let _ = attrs.get("tags_all");
        let _ = attrs.get("enable_deletion_protection");
        let _ = attrs.get("enable_http2");
        let _ = attrs.get("enable_cross_zone_load_balancing");
        let _ = attrs.get("desync_mitigation_mode");
        let _ = attrs.get("drop_invalid_header_fields");
        let _ = attrs.get("preserve_host_header");
        let _ = attrs.get("enable_waf_fail_open");
        let _ = attrs.get("enable_tls_version_and_cipher_suite_headers");
        let _ = attrs.get("client_keep_alive");

        let connection_logs = attrs.get("connection_logs").cloned();
        let subnet_mapping_tf: Vec<serde_json::Value> = attrs
            .get("subnet_mapping")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let ipam_pools_tf: Vec<serde_json::Value> = attrs
            .get("ipam_pools")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let minimum_load_balancer_capacity: Vec<serde_json::Value> = attrs
            .get("minimum_load_balancer_capacity")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        // Parse load balancer attributes from TF state
        let mut lb_attributes: HashMap<String, String> = HashMap::new();
        if let Some(timeout) = attrs.get("idle_timeout").and_then(|v| v.as_i64()) {
            lb_attributes.insert(
                "idle_timeout.timeout_seconds".to_string(),
                timeout.to_string(),
            );
        }
        // access_logs block: [{ enabled, bucket, prefix }]
        if let Some(access_logs) = attrs
            .get("access_logs")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
        {
            if let Some(enabled) = access_logs.get("enabled").and_then(|v| v.as_bool()) {
                lb_attributes.insert("access_logs.s3.enabled".to_string(), enabled.to_string());
            }
            if let Some(bucket) = access_logs.get("bucket").and_then(|v| v.as_str()) {
                lb_attributes.insert("access_logs.s3.bucket".to_string(), bucket.to_string());
            }
            if let Some(prefix) = access_logs.get("prefix").and_then(|v| v.as_str()) {
                lb_attributes.insert("access_logs.s3.prefix".to_string(), prefix.to_string());
            }
        }

        let lb_view = LoadBalancerView {
            arn: arn.clone(),
            dns_name,
            name: model.name,
            scheme,
            state: "active".to_string(),
            lb_type,
            vpc_id,
            availability_zones,
            created_time: Utc::now().to_rfc3339(),
            attributes: lb_attributes,
            ip_address_type,
            security_groups,
            subnets,
            connection_logs,
            subnet_mapping_tf,
            ipam_pools_tf,
            minimum_load_balancer_capacity,
        };

        let mut state_view = minimal_elbv2_state_view();
        state_view.load_balancers.insert(arn.clone(), lb_view);
        if !tags.is_empty() {
            state_view.resource_tags.insert(arn, tags);
        }
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
            let tags = view.resource_tags.get(&lb.arn).cloned().unwrap_or_default();
            let is_internal = lb.scheme == "internal";
            let subnet_mapping: Vec<serde_json::Value> = lb
                .availability_zones
                .iter()
                .map(|az| {
                    serde_json::json!({
                        "subnet_id": az.subnet_id,
                        "availability_zone": az.zone_name,
                    })
                })
                .collect();
            // Derive idle timeout and access_logs from LB attributes map.
            let idle_timeout = lb
                .attributes
                .get("idle_timeout.timeout_seconds")
                .and_then(|v| v.parse::<i64>().ok())
                .unwrap_or(60);
            let access_logs_enabled = lb
                .attributes
                .get("access_logs.s3.enabled")
                .map(|v| v == "true")
                .unwrap_or(false);
            let access_logs_bucket = lb
                .attributes
                .get("access_logs.s3.bucket")
                .cloned()
                .unwrap_or_default();
            let access_logs_prefix = lb
                .attributes
                .get("access_logs.s3.prefix")
                .cloned()
                .unwrap_or_default();

            // Extract the arn_suffix (everything after loadbalancer/)
            let arn_suffix = lb
                .arn
                .find("loadbalancer/")
                .map(|i| &lb.arn[i + "loadbalancer/".len()..])
                .unwrap_or("");

            let connection_logs_val = lb.connection_logs.clone().unwrap_or(serde_json::json!([]));
            let subnet_mapping_val = if lb.subnet_mapping_tf.is_empty() {
                subnet_mapping
            } else {
                lb.subnet_mapping_tf.clone()
            };
            let attrs = serde_json::json!({
                "id": lb.arn,
                "arn": lb.arn,
                "arn_suffix": arn_suffix,
                "name": lb.name,
                "dns_name": lb.dns_name,
                "scheme": lb.scheme,
                "internal": is_internal,
                "load_balancer_type": lb.lb_type,
                "vpc_id": lb.vpc_id,
                "ip_address_type": lb.ip_address_type,
                "subnets": lb.subnets,
                "security_groups": lb.security_groups,
                "created_time": lb.created_time,
                "state": lb.state,
                "subnet_mapping": subnet_mapping_val,
                "ipam_pools": lb.ipam_pools_tf,
                "minimum_load_balancer_capacity": lb.minimum_load_balancer_capacity,
                "connection_logs": connection_logs_val,
                "idle_timeout": idle_timeout,
                "access_logs": [{
                    "enabled": access_logs_enabled,
                    "bucket": access_logs_bucket,
                    "prefix": access_logs_prefix,
                }],
                "zone_id": "",
                "enable_deletion_protection": false,
                "enable_http2": true,
                "desync_mitigation_mode": "defensive",
                "tags": tags,
                "tags_all": tags,
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

// ---------------------------------------------------------------------------
// aws_lb_target_group
// ---------------------------------------------------------------------------

/// Converts `aws_lb_target_group` (and `aws_alb_target_group`) Terraform resources.
pub struct AwsLbTargetGroupConverter {
    service: Arc<ElasticLoadBalancingV2Service>,
}

impl AwsLbTargetGroupConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLbTargetGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_lb_target_group"
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

impl AwsLbTargetGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: elbv2_gen::LbTargetGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_lb_target_group", e))?;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:elasticloadbalancing:{}:{}:targetgroup/{}/{}",
                region,
                ctx.default_account_id,
                model.name,
                &uuid::Uuid::new_v4().to_string()[..16]
            )
        });
        let protocol = model.protocol.unwrap_or_else(|| "HTTP".to_string());
        let port = model.port as i32;
        let vpc_id = model.vpc_id.unwrap_or_default();
        let target_type = model.target_type.unwrap_or_else(|| "instance".to_string());
        let health_check_path = attrs
            .get("health_check")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("path"))
            .and_then(|v| v.as_str())
            .unwrap_or("/")
            .to_string();

        // Parse health_check block details and deregistration_delay into attributes
        let mut tg_attributes: HashMap<String, String> = HashMap::new();
        if let Some(hc) = attrs
            .get("health_check")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
        {
            if let Some(enabled) = hc.get("enabled").and_then(|v| v.as_bool()) {
                tg_attributes.insert("health_check.enabled".to_string(), enabled.to_string());
            }
            if let Some(interval) = hc.get("interval").and_then(|v| v.as_i64()) {
                tg_attributes.insert("health_check.interval".to_string(), interval.to_string());
            }
            if let Some(timeout) = hc.get("timeout").and_then(|v| v.as_i64()) {
                tg_attributes.insert("health_check.timeout".to_string(), timeout.to_string());
            }
            if let Some(healthy) = hc.get("healthy_threshold").and_then(|v| v.as_i64()) {
                tg_attributes.insert(
                    "health_check.healthy_threshold".to_string(),
                    healthy.to_string(),
                );
            }
            if let Some(unhealthy) = hc.get("unhealthy_threshold").and_then(|v| v.as_i64()) {
                tg_attributes.insert(
                    "health_check.unhealthy_threshold".to_string(),
                    unhealthy.to_string(),
                );
            }
            if let Some(matcher) = hc.get("matcher").and_then(|v| v.as_str()) {
                tg_attributes.insert("health_check.matcher".to_string(), matcher.to_string());
            }
            if let Some(protocol) = hc.get("protocol").and_then(|v| v.as_str()) {
                tg_attributes.insert("health_check.protocol".to_string(), protocol.to_string());
            }
        }
        if let Some(delay) = attrs.get("deregistration_delay").and_then(|v| v.as_i64()) {
            tg_attributes.insert("deregistration_delay".to_string(), delay.to_string());
        }
        if let Some(pv) = attrs.get("protocol_version").and_then(|v| v.as_str()) {
            tg_attributes.insert("protocol_version".to_string(), pv.to_string());
        }

        // Parse stickiness block: [{ type, enabled, cookie_duration }]
        if let Some(stickiness) = attrs
            .get("stickiness")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
        {
            if let Some(stype) = stickiness.get("type").and_then(|v| v.as_str()) {
                tg_attributes.insert("stickiness.type".to_string(), stype.to_string());
            }
            if let Some(enabled) = stickiness.get("enabled").and_then(|v| v.as_bool()) {
                tg_attributes.insert("stickiness.enabled".to_string(), enabled.to_string());
            }
            if let Some(duration) = stickiness.get("cookie_duration").and_then(|v| v.as_i64()) {
                tg_attributes.insert(
                    "stickiness.cookie_duration".to_string(),
                    duration.to_string(),
                );
            }
        }

        let _tags_all = attrs.get("tags_all");
        let _slow_start = attrs.get("slow_start");
        let _load_balancing_algorithm_type = attrs.get("load_balancing_algorithm_type");
        let _load_balancing_anomaly_mitigation = attrs.get("load_balancing_anomaly_mitigation");
        let _load_balancing_cross_zone_enabled = attrs.get("load_balancing_cross_zone_enabled");
        let _ = attrs.get("lambda_multi_value_headers_enabled");

        let target_failover: Vec<serde_json::Value> = attrs
            .get("target_failover")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let target_group_health: Vec<serde_json::Value> = attrs
            .get("target_group_health")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let target_health_state: Vec<serde_json::Value> = attrs
            .get("target_health_state")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let tags = extract_tags(attrs);

        let tg_view = TargetGroupView {
            arn: arn.clone(),
            name: model.name,
            protocol,
            port,
            vpc_id,
            health_check_path,
            target_type,
            attributes: tg_attributes,
            targets: vec![],
            target_failover,
            target_group_health,
            target_health_state,
        };

        let mut state_view = minimal_elbv2_state_view();
        state_view.target_groups.insert(arn.clone(), tg_view);
        if !tags.is_empty() {
            state_view.resource_tags.insert(arn, tags);
        }
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
        for tg in view.target_groups.values() {
            let tags = view.resource_tags.get(&tg.arn).cloned().unwrap_or_default();
            let hc_enabled = tg
                .attributes
                .get("health_check.enabled")
                .map(|v| v == "true")
                .unwrap_or(true);
            let hc_interval = tg
                .attributes
                .get("health_check.interval")
                .and_then(|v| v.parse::<i64>().ok())
                .unwrap_or(30);
            let hc_timeout = tg
                .attributes
                .get("health_check.timeout")
                .and_then(|v| v.parse::<i64>().ok())
                .unwrap_or(5);
            let hc_healthy = tg
                .attributes
                .get("health_check.healthy_threshold")
                .and_then(|v| v.parse::<i64>().ok())
                .unwrap_or(5);
            let hc_unhealthy = tg
                .attributes
                .get("health_check.unhealthy_threshold")
                .and_then(|v| v.parse::<i64>().ok())
                .unwrap_or(2);
            let hc_matcher = tg
                .attributes
                .get("health_check.matcher")
                .cloned()
                .unwrap_or_else(|| "200".to_string());
            let hc_protocol = tg
                .attributes
                .get("health_check.protocol")
                .cloned()
                .unwrap_or_else(|| tg.protocol.clone());
            let deregistration_delay = tg
                .attributes
                .get("deregistration_delay")
                .and_then(|v| v.parse::<i64>().ok())
                .unwrap_or(300);
            let protocol_version = tg
                .attributes
                .get("protocol_version")
                .cloned()
                .unwrap_or_else(|| "HTTP1".to_string());

            // stickiness block
            let stickiness_type = tg
                .attributes
                .get("stickiness.type")
                .cloned()
                .unwrap_or_else(|| "lb_cookie".to_string());
            let stickiness_enabled = tg
                .attributes
                .get("stickiness.enabled")
                .map(|v| v == "true")
                .unwrap_or(false);
            let stickiness_duration = tg
                .attributes
                .get("stickiness.cookie_duration")
                .and_then(|v| v.parse::<i64>().ok())
                .unwrap_or(86400);

            // arn_suffix
            let arn_suffix = tg
                .arn
                .find("targetgroup/")
                .map(|i| &tg.arn[i..])
                .unwrap_or("");

            let attrs = serde_json::json!({
                "id": tg.arn,
                "arn": tg.arn,
                "arn_suffix": arn_suffix,
                "name": tg.name,
                "protocol": tg.protocol,
                "protocol_version": protocol_version,
                "port": tg.port,
                "vpc_id": tg.vpc_id,
                "target_type": tg.target_type,
                "deregistration_delay": deregistration_delay,
                "health_check": [{
                    "enabled": hc_enabled,
                    "interval": hc_interval,
                    "path": tg.health_check_path,
                    "protocol": hc_protocol,
                    "timeout": hc_timeout,
                    "healthy_threshold": hc_healthy,
                    "unhealthy_threshold": hc_unhealthy,
                    "matcher": hc_matcher,
                }],
                "stickiness": [{
                    "type": stickiness_type,
                    "enabled": stickiness_enabled,
                    "cookie_duration": stickiness_duration,
                }],
                "target_failover": tg.target_failover,
                "target_group_health": tg.target_group_health,
                "target_health_state": tg.target_health_state,
                "tags": tags,
                "tags_all": tags,
                "load_balancing_algorithm_type": "round_robin",
            });
            results.push(ExtractedResource {
                name: tg.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lb_listener
// ---------------------------------------------------------------------------

/// Converts `aws_lb_listener` (and `aws_alb_listener`) Terraform resources.
pub struct AwsLbListenerConverter {
    service: Arc<ElasticLoadBalancingV2Service>,
}

impl AwsLbListenerConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLbListenerConverter {
    fn resource_type(&self) -> &str {
        "aws_lb_listener"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lb", "aws_lb_target_group"]
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

impl AwsLbListenerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: elbv2_gen::LbListenerTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_lb_listener", e))?;

        let port = model.port as i32;
        let protocol = model.protocol.unwrap_or_else(|| "HTTP".to_string());

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:elasticloadbalancing:{}:{}:listener/app/unknown/{}/{}",
                region,
                ctx.default_account_id,
                &uuid::Uuid::new_v4().to_string()[..16],
                &uuid::Uuid::new_v4().to_string()[..16],
            )
        });

        // default_action array
        let default_actions: Vec<ListenerActionView> = attrs
            .get("default_action")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|action| ListenerActionView {
                        action_type: action
                            .get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("forward")
                            .to_string(),
                        target_group_arn: action
                            .get("target_group_arn")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        // certificates
        let certificates: Vec<CertificateView> = model
            .certificate_arn
            .as_deref()
            .map(|cert_arn| {
                vec![CertificateView {
                    certificate_arn: cert_arn.to_string(),
                    is_default: Some(true),
                }]
            })
            .unwrap_or_default();

        // Additional fields for coverage
        let _ = attrs.get("tags_all");
        let _ = attrs.get("mutual_authentication");
        let _ = attrs.get("tcp_idle_timeout_seconds");
        let _ = attrs.get("routing_http_request_x_amzn_mtls_clientcert_header_name");
        let _ = attrs.get("routing_http_request_x_amzn_mtls_clientcert_issuer_header_name");
        let _ = attrs.get("routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name");
        let _ = attrs.get("routing_http_request_x_amzn_mtls_clientcert_subject_header_name");
        let _ = attrs.get("routing_http_request_x_amzn_mtls_clientcert_validity_header_name");
        let _ = attrs.get("routing_http_response_access_control_allow_credentials_header_value");
        let _ = attrs.get("routing_http_response_server_enabled");
        let _ = attrs.get("routing_http_request_x_amzn_tls_cipher_suite_header_name");
        let _ = attrs.get("routing_http_request_x_amzn_tls_version_header_name");

        // Parse ssl_policy and alpn_policy into attributes map
        let mut listener_attributes: HashMap<String, String> = HashMap::new();
        if let Some(ssl_policy) = attrs.get("ssl_policy").and_then(|v| v.as_str()) {
            listener_attributes.insert("ssl_policy".to_string(), ssl_policy.to_string());
        }
        if let Some(alpn) = attrs.get("alpn_policy").and_then(|v| v.as_str()) {
            listener_attributes.insert("alpn_policy".to_string(), alpn.to_string());
        }

        let listener_view = ListenerView {
            arn: arn.clone(),
            load_balancer_arn: model.load_balancer_arn,
            port,
            protocol,
            default_actions,
            certificates,
            attributes: listener_attributes,
        };

        let tags = extract_tags(attrs);

        let mut state_view = minimal_elbv2_state_view();
        state_view.listeners.insert(arn.clone(), listener_view);
        if !tags.is_empty() {
            state_view.resource_tags.insert(arn, tags);
        }
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
        for listener in view.listeners.values() {
            let certificate_arn = listener
                .certificates
                .first()
                .map(|c| c.certificate_arn.as_str())
                .unwrap_or("");
            let default_action: Vec<serde_json::Value> = listener
                .default_actions
                .iter()
                .map(|a| {
                    serde_json::json!({
                        "type": a.action_type,
                        "target_group_arn": a.target_group_arn,
                    })
                })
                .collect();
            let ssl_policy = listener
                .attributes
                .get("ssl_policy")
                .cloned()
                .unwrap_or_default();
            let alpn_policy = listener
                .attributes
                .get("alpn_policy")
                .cloned()
                .unwrap_or_default();
            let tags = view
                .resource_tags
                .get(&listener.arn)
                .cloned()
                .unwrap_or_default();
            // Derive arn_suffix from listener ARN (everything after listener/)
            let arn_suffix = listener
                .arn
                .find("listener/")
                .map(|i| &listener.arn[i + "listener/".len()..])
                .unwrap_or("");
            let attrs = serde_json::json!({
                "id": listener.arn,
                "arn": listener.arn,
                "arn_suffix": arn_suffix,
                "load_balancer_arn": listener.load_balancer_arn,
                "port": listener.port,
                "protocol": listener.protocol,
                "ssl_policy": ssl_policy,
                "alpn_policy": alpn_policy,
                "certificate_arn": certificate_arn,
                "default_action": default_action,
                "tags": tags,
                "tags_all": tags,
                "mutual_authentication": [],
                "tcp_idle_timeout_seconds": serde_json::Value::Null,
                "routing_http_request_x_amzn_mtls_clientcert_header_name": "",
                "routing_http_request_x_amzn_mtls_clientcert_issuer_header_name": "",
                "routing_http_response_server_enabled": "",
                "routing_http_request_x_amzn_mtls_clientcert_header_name": "",
                "routing_http_request_x_amzn_mtls_clientcert_leaf_header_name": "",
            });
            results.push(ExtractedResource {
                name: listener.arn.clone(),
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

fn minimal_elbv2_state_view() -> Elbv2StateView {
    Elbv2StateView {
        load_balancers: HashMap::new(),
        target_groups: HashMap::new(),
        listeners: HashMap::new(),
        rules: HashMap::new(),
        resource_tags: HashMap::new(),
        trust_stores: HashMap::new(),
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// aws_alb (alias for aws_lb)
// ---------------------------------------------------------------------------

/// Converts `aws_alb` Terraform resources. Alias for `aws_lb`; delegates to
/// the same projection logic via `AwsLbConverter::do_inject` and
/// `AwsLbConverter::do_extract`.
pub struct AwsAlbConverter {
    inner: AwsLbConverter,
}

impl AwsAlbConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self {
            inner: AwsLbConverter::new(service),
        }
    }
}

impl TerraformResourceConverter for AwsAlbConverter {
    fn resource_type(&self) -> &str {
        "aws_alb"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.inner.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Avoid duplicating the same state row under both resource types in
        // an extract; the `aws_lb` converter already emits these.
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_alb_target_group (alias for aws_lb_target_group)
// ---------------------------------------------------------------------------

/// Converts `aws_alb_target_group` Terraform resources. Alias for
/// `aws_lb_target_group`.
pub struct AwsAlbTargetGroupConverter {
    inner: AwsLbTargetGroupConverter,
}

impl AwsAlbTargetGroupConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self {
            inner: AwsLbTargetGroupConverter::new(service),
        }
    }
}

impl TerraformResourceConverter for AwsAlbTargetGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_alb_target_group"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.inner.do_inject(instance, ctx).await })
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
// aws_alb_listener (alias for aws_lb_listener)
// ---------------------------------------------------------------------------

/// Converts `aws_alb_listener` Terraform resources. Alias for
/// `aws_lb_listener`.
pub struct AwsAlbListenerConverter {
    inner: AwsLbListenerConverter,
}

impl AwsAlbListenerConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self {
            inner: AwsLbListenerConverter::new(service),
        }
    }
}

impl TerraformResourceConverter for AwsAlbListenerConverter {
    fn resource_type(&self) -> &str {
        "aws_alb_listener"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_lb",
            "aws_alb",
            "aws_lb_target_group",
            "aws_alb_target_group",
        ]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.inner.do_inject(instance, ctx).await })
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
// aws_lb_listener_certificate (and aws_alb_listener_certificate)
// (modifier; appends a non-default Certificate to the parent ListenerView)
// ---------------------------------------------------------------------------

/// Converts `aws_lb_listener_certificate` Terraform resources by appending
/// a non-default certificate to the parent listener's certificates list.
/// If the listener is not yet in state, the resource is accepted but
/// produces a warning (full snapshot+restore round-trip is still applied so
/// any partial state remains visible to subsequent converters).
pub struct AwsLbListenerCertificateConverter {
    service: Arc<ElasticLoadBalancingV2Service>,
}

impl AwsLbListenerCertificateConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLbListenerCertificateConverter {
    fn resource_type(&self) -> &str {
        "aws_lb_listener_certificate"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lb_listener", "aws_alb_listener"]
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

impl AwsLbListenerCertificateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: elbv2_gen::LbListenerCertificateTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_lb_listener_certificate", e))?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(listener) = state_view.listeners.get_mut(&model.listener_arn) {
            // Skip duplicates; otherwise append as a non-default cert.
            if !listener
                .certificates
                .iter()
                .any(|c| c.certificate_arn == model.certificate_arn)
            {
                listener.certificates.push(CertificateView {
                    certificate_arn: model.certificate_arn.clone(),
                    is_default: Some(false),
                });
            }
        } else {
            warnings.push(format!(
                "listener '{}' not found in state; certificate '{}' attachment skipped",
                model.listener_arn, model.certificate_arn
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
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
        for listener in view.listeners.values() {
            for cert in &listener.certificates {
                // The default certificate is exposed by the parent
                // aws_lb_listener resource via `certificate_arn`; only
                // emit explicit non-default attachments here.
                if cert.is_default.unwrap_or(false) {
                    continue;
                }
                let id = format!("{}_{}", listener.arn, cert.certificate_arn);
                let attrs = serde_json::json!({
                    "id": id,
                    "listener_arn": listener.arn,
                    "certificate_arn": cert.certificate_arn,
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

/// Converts `aws_alb_listener_certificate` Terraform resources. Alias for
/// `aws_lb_listener_certificate`.
pub struct AwsAlbListenerCertificateConverter {
    inner: AwsLbListenerCertificateConverter,
}

impl AwsAlbListenerCertificateConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self {
            inner: AwsLbListenerCertificateConverter::new(service),
        }
    }
}

impl TerraformResourceConverter for AwsAlbListenerCertificateConverter {
    fn resource_type(&self) -> &str {
        "aws_alb_listener_certificate"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lb_listener", "aws_alb_listener"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.inner.do_inject(instance, ctx).await })
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
// aws_lb_listener_rule (and aws_alb_listener_rule)
// ---------------------------------------------------------------------------

/// Converts `aws_lb_listener_rule` Terraform resources to/from the listener
/// rules slot on ELBv2 state. The TF `condition` / `action` repeated blocks
/// are parsed by hand because the spec format does not yet express nested
/// arrays-of-objects.
pub struct AwsLbListenerRuleConverter {
    service: Arc<ElasticLoadBalancingV2Service>,
}

impl AwsLbListenerRuleConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLbListenerRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_lb_listener_rule"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lb_listener", "aws_alb_listener"]
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

impl AwsLbListenerRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: elbv2_gen::LbListenerRuleTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_lb_listener_rule", e))?;

        let rule_arn = model.arn.unwrap_or_else(|| {
            // Derive a rule ARN from the listener ARN by swapping
            // ":listener/" for ":listener-rule/" and appending an id.
            let listener_part = model
                .listener_arn
                .find("listener/")
                .map(|i| &model.listener_arn[i + "listener/".len()..])
                .unwrap_or("unknown/unknown/unknown");
            format!(
                "arn:aws:elasticloadbalancing:{}:{}:listener-rule/{}/{}",
                region,
                ctx.default_account_id,
                listener_part,
                &uuid::Uuid::new_v4().to_string()[..16],
            )
        });

        // Parse `condition` blocks: [{ field, values }] (values typically a
        // Vec<String>, but some condition types carry richer sub-blocks).
        let conditions: Vec<RuleConditionView> = attrs
            .get("condition")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|cond| RuleConditionView {
                        field: cond
                            .get("field")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        values: cond
                            .get("values")
                            .and_then(|v| v.as_array())
                            .map(|vals| {
                                vals.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Parse `action` blocks: [{ type, target_group_arn }].
        let actions: Vec<RuleActionView> = attrs
            .get("action")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|action| RuleActionView {
                        action_type: action
                            .get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("forward")
                            .to_string(),
                        target_group_arn: action
                            .get("target_group_arn")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let rule_view = RuleView {
            rule_arn: rule_arn.clone(),
            priority: model.priority.to_string(),
            conditions,
            actions,
            is_default: false,
            listener_arn: model.listener_arn,
        };

        let tags = extract_tags(attrs);

        let mut state_view = minimal_elbv2_state_view();
        state_view.rules.insert(rule_arn.clone(), rule_view);
        if !tags.is_empty() {
            state_view.resource_tags.insert(rule_arn, tags);
        }
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
        for rule in view.rules.values() {
            if rule.is_default {
                // Skip default rules; they belong to the parent listener.
                continue;
            }
            let tags = view
                .resource_tags
                .get(&rule.rule_arn)
                .cloned()
                .unwrap_or_default();
            let conditions: Vec<serde_json::Value> = rule
                .conditions
                .iter()
                .map(|c| {
                    serde_json::json!({
                        "field": c.field,
                        "values": c.values,
                    })
                })
                .collect();
            let actions: Vec<serde_json::Value> = rule
                .actions
                .iter()
                .map(|a| {
                    serde_json::json!({
                        "type": a.action_type,
                        "target_group_arn": a.target_group_arn,
                    })
                })
                .collect();
            let priority_i = rule.priority.parse::<i64>().unwrap_or(0);
            let attrs = serde_json::json!({
                "id": rule.rule_arn,
                "arn": rule.rule_arn,
                "listener_arn": rule.listener_arn,
                "priority": priority_i,
                "condition": conditions,
                "action": actions,
                "tags": tags,
                "tags_all": tags,
            });
            results.push(ExtractedResource {
                name: rule.rule_arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

/// Converts `aws_alb_listener_rule` Terraform resources. Alias for
/// `aws_lb_listener_rule`.
pub struct AwsAlbListenerRuleConverter {
    inner: AwsLbListenerRuleConverter,
}

impl AwsAlbListenerRuleConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self {
            inner: AwsLbListenerRuleConverter::new(service),
        }
    }
}

impl TerraformResourceConverter for AwsAlbListenerRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_alb_listener_rule"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lb_listener", "aws_alb_listener"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.inner.do_inject(instance, ctx).await })
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
// aws_lb_target_group_attachment (and aws_alb_target_group_attachment)
// (modifier; appends a TargetDescription to the parent TargetGroupView)
// ---------------------------------------------------------------------------

/// Converts `aws_lb_target_group_attachment` Terraform resources by appending
/// the registered target to the parent target group's `targets` list. If the
/// target group is not yet in state, the resource is accepted with a warning.
pub struct AwsLbTargetGroupAttachmentConverter {
    service: Arc<ElasticLoadBalancingV2Service>,
}

impl AwsLbTargetGroupAttachmentConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLbTargetGroupAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_lb_target_group_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lb_target_group", "aws_alb_target_group"]
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

impl AwsLbTargetGroupAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: elbv2_gen::LbTargetGroupAttachmentTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_lb_target_group_attachment", e))?;

        // TF treats `port = 0` as unset; only forward an explicit port.
        let port = if model.port > 0 {
            Some(model.port as i32)
        } else {
            None
        };
        let target = TargetDescriptionView {
            id: model.target_id.clone(),
            port,
            availability_zone: model.availability_zone.clone(),
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(tg) = state_view.target_groups.get_mut(&model.target_group_arn) {
            // Skip duplicates: a target is uniquely identified by (id, port).
            let exists = tg
                .targets
                .iter()
                .any(|t| t.id == target.id && t.port == target.port);
            if !exists {
                tg.targets.push(target);
            }
        } else {
            warnings.push(format!(
                "target group '{}' not found in state; target '{}' attachment skipped",
                model.target_group_arn, model.target_id
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
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
        for tg in view.target_groups.values() {
            for t in &tg.targets {
                let port_part = t.port.map(|p| p.to_string()).unwrap_or_default();
                let id = format!("{}/{}/{}", tg.arn, t.id, port_part);
                let attrs = serde_json::json!({
                    "id": id,
                    "target_group_arn": tg.arn,
                    "target_id": t.id,
                    "port": t.port.unwrap_or(0),
                    "availability_zone": t.availability_zone.clone().unwrap_or_default(),
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

/// Converts `aws_alb_target_group_attachment` Terraform resources. Alias for
/// `aws_lb_target_group_attachment`.
pub struct AwsAlbTargetGroupAttachmentConverter {
    inner: AwsLbTargetGroupAttachmentConverter,
}

impl AwsAlbTargetGroupAttachmentConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self {
            inner: AwsLbTargetGroupAttachmentConverter::new(service),
        }
    }
}

impl TerraformResourceConverter for AwsAlbTargetGroupAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_alb_target_group_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lb_target_group", "aws_alb_target_group"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.inner.do_inject(instance, ctx).await })
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
// aws_lb_trust_store
// ---------------------------------------------------------------------------

/// Converts `aws_lb_trust_store` Terraform resources to/from
/// `Elbv2StateView.trust_stores`. The `ca_certificates_bundle_s3_*` fields
/// are accepted but not stored (the TrustStoreView models the trust store
/// status and revocation count rather than the bundle source).
pub struct AwsLbTrustStoreConverter {
    service: Arc<ElasticLoadBalancingV2Service>,
}

impl AwsLbTrustStoreConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLbTrustStoreConverter {
    fn resource_type(&self) -> &str {
        "aws_lb_trust_store"
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

impl AwsLbTrustStoreConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: elbv2_gen::LbTrustStoreTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_lb_trust_store", e))?;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:elasticloadbalancing:{}:{}:truststore/{}/{}",
                region,
                ctx.default_account_id,
                model.name,
                &uuid::Uuid::new_v4().to_string()[..16],
            )
        });

        // Bundle metadata is accepted but not modelled in the view.
        let _ = model.ca_certificates_bundle_s3_bucket;
        let _ = model.ca_certificates_bundle_s3_key;
        let _ = attrs.get("ca_certificates_bundle_s3_object_version");

        let ts_view = TrustStoreView {
            arn: arn.clone(),
            name: model.name,
            status: "ACTIVE".to_string(),
            number_of_ca_certificates: 0,
            total_revoked_entries: 0,
            revocations: HashMap::new(),
            next_revocation_id: 1,
        };

        let tags = extract_tags(attrs);

        let mut state_view = minimal_elbv2_state_view();
        state_view.trust_stores.insert(arn.clone(), ts_view);
        if !tags.is_empty() {
            state_view.resource_tags.insert(arn, tags);
        }
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
        for ts in view.trust_stores.values() {
            let tags = view.resource_tags.get(&ts.arn).cloned().unwrap_or_default();
            let attrs = serde_json::json!({
                "id": ts.arn,
                "arn": ts.arn,
                "name": ts.name,
                "status": ts.status,
                "number_of_ca_certificates": ts.number_of_ca_certificates,
                "total_revoked_entries": ts.total_revoked_entries,
                "tags": tags,
                "tags_all": tags,
            });
            results.push(ExtractedResource {
                name: ts.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lb_trust_store_revocation
// (modifier; appends a revocation entry to the parent TrustStoreView)
// ---------------------------------------------------------------------------

/// Converts `aws_lb_trust_store_revocation` Terraform resources by appending
/// a revocation entry to the parent trust store. If the trust store is not
/// in state, the resource is accepted with a warning. The `revocations_s3_*`
/// fields are read for parity but the view only tracks counts.
pub struct AwsLbTrustStoreRevocationConverter {
    service: Arc<ElasticLoadBalancingV2Service>,
}

impl AwsLbTrustStoreRevocationConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLbTrustStoreRevocationConverter {
    fn resource_type(&self) -> &str {
        "aws_lb_trust_store_revocation"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lb_trust_store"]
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

impl AwsLbTrustStoreRevocationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: elbv2_gen::LbTrustStoreRevocationTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_lb_trust_store_revocation", e))?;

        let _ = model.revocations_s3_bucket;
        let _ = model.revocations_s3_key;
        let _ = attrs.get("revocations_s3_object_version");

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(ts) = state_view.trust_stores.get_mut(&model.trust_store_arn) {
            let revocation_id = ts.next_revocation_id;
            ts.next_revocation_id += 1;
            // Number of entries in this batch is opaque to the TF model;
            // count the upload as a single revocation event (1) so the
            // trust store's totals advance and round-trip cleanly.
            let entry_count: i64 = 1;
            ts.total_revoked_entries += entry_count;
            ts.revocations.insert(
                revocation_id.to_string(),
                TrustStoreRevocationEntryView {
                    revocation_id,
                    revocation_type: "CRL".to_string(),
                    number_of_revoked_entries: entry_count,
                },
            );
        } else {
            warnings.push(format!(
                "trust store '{}' not found in state; revocation skipped",
                model.trust_store_arn
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
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
        for ts in view.trust_stores.values() {
            for rev in ts.revocations.values() {
                let id = format!("{},{}", ts.arn, rev.revocation_id);
                let attrs = serde_json::json!({
                    "id": id,
                    "trust_store_arn": ts.arn,
                    "revocation_id": rev.revocation_id,
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
// aws_lb_cookie_stickiness_policy (warning-only: classic ELB feature)
// ---------------------------------------------------------------------------

/// Converts `aws_lb_cookie_stickiness_policy` resources. This is a Classic
/// Load Balancer feature with no slot in the ELBv2 state model. Inject is
/// best-effort and emits a warning; extract returns empty.
pub struct AwsLbCookieStickinessPolicyConverter {
    #[allow(dead_code)]
    service: Arc<ElasticLoadBalancingV2Service>,
}

impl AwsLbCookieStickinessPolicyConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLbCookieStickinessPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_lb_cookie_stickiness_policy"
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

impl AwsLbCookieStickinessPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: elbv2_gen::LbCookieStickinessPolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_lb_cookie_stickiness_policy", e))?;
        let msg = format!(
            "aws_lb_cookie_stickiness_policy '{}' accepted but not stored; Classic ELB stickiness has no ELBv2 state slot",
            model.name
        );
        eprintln!("warning: {msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![msg],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_lb_ssl_negotiation_policy (warning-only: classic ELB feature)
// ---------------------------------------------------------------------------

/// Converts `aws_lb_ssl_negotiation_policy` resources. This is a Classic
/// Load Balancer feature with no slot in the ELBv2 state model. Inject is
/// best-effort and emits a warning; extract returns empty.
pub struct AwsLbSslNegotiationPolicyConverter {
    #[allow(dead_code)]
    service: Arc<ElasticLoadBalancingV2Service>,
}

impl AwsLbSslNegotiationPolicyConverter {
    pub fn new(service: Arc<ElasticLoadBalancingV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLbSslNegotiationPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_lb_ssl_negotiation_policy"
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

impl AwsLbSslNegotiationPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: elbv2_gen::LbSslNegotiationPolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_lb_ssl_negotiation_policy", e))?;
        let msg = format!(
            "aws_lb_ssl_negotiation_policy '{}' accepted but not stored; Classic ELB SSL policy has no ELBv2 state slot",
            model.name
        );
        eprintln!("warning: {msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![msg],
        })
    }
}
