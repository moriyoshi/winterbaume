//! `AWS::ElasticLoadBalancingV2::LoadBalancer` shaper.
//!
//! Reference: <https://schema.cloudformation.us-east-1.amazonaws.com/aws-elasticloadbalancingv2-loadbalancer.json>
//! - `primaryIdentifier`: `/properties/LoadBalancerArn`
//! - `readOnlyProperties`: `LoadBalancerName`, `LoadBalancerFullName`,
//!   `CanonicalHostedZoneID`, `LoadBalancerArn`, `DNSName`
//! - `writeOnlyProperties`: `EnableCapacityReservationProvisionStabilize`
//! - The schema declares a `default` only for `EnablePrefixForIpv6SourceNat`
//!   (`"off"`). The real AWS read handler additionally fills `IpAddressType`
//!   (`"ipv4"`), a 22-entry `LoadBalancerAttributes` block, and derives
//!   `SubnetMappings` from `Subnets` (see issue #10 for the captured
//!   live-account snapshot). Mirror that handler behaviour here.
//!
//! Closely related to [[elbv2_target_group]] and [[elbv2_listener]].

use serde_json::{Map, Value, json};
use uuid::Uuid;

use super::{CfnResourceShaper, ShapeContext, ShapedResource};

pub(super) struct ElbV2LoadBalancerShaper;

const WRITE_ONLY: &[&str] = &["EnableCapacityReservationProvisionStabilize"];

fn strip_write_only(obj: &mut Map<String, Value>) {
    for key in WRITE_ONLY {
        obj.remove(*key);
    }
}

/// Map the user-facing `Type` to the LB-ARN path segment AWS uses for the
/// load balancer (and listener) ARN form.
fn type_prefix(lb_type: &str) -> &'static str {
    match lb_type {
        "network" => "net",
        "gateway" => "gwy",
        _ => "app",
    }
}

/// A documented, region-agnostic canonical hosted zone ID per LB type.
/// Real AWS returns a region-specific ID; for a fake the format and stability
/// are what matters, so we return well-formed real IDs from the us-east-1
/// row of the published zone-ID table.
fn canonical_hosted_zone_id(lb_type: &str) -> &'static str {
    match lb_type {
        "network" => "Z26RNL4JYFTOTI",
        "gateway" => "Z08475332ECTYE9MO92F",
        _ => "Z35SXDOTRQ7X7K",
    }
}

fn random_hex16() -> String {
    Uuid::new_v4().simple().to_string()[..16].to_string()
}

/// Real ALB/NLB DNS names look like `<name>-<10digits>.<region>.elb.amazonaws.com`.
/// Derive a stable-format 10-digit decimal suffix from random bytes so the
/// shape matches without taking on a `rand` dependency.
fn dns_decimal_suffix() -> String {
    let bytes = Uuid::new_v4().into_bytes();
    let raw = u64::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]);
    let n = raw % 9_000_000_000 + 1_000_000_000;
    n.to_string()
}

fn fill_defaults(obj: &mut Map<String, Value>) {
    let defaults: [(&str, Value); 4] = [
        ("IpAddressType", json!("ipv4")),
        ("EnablePrefixForIpv6SourceNat", json!("off")),
        (
            "LoadBalancerAttributes",
            json!([
                {"Key": "access_logs.s3.prefix", "Value": ""},
                {"Key": "routing.http.xff_header_processing.mode", "Value": "append"},
                {"Key": "routing.http2.enabled", "Value": "true"},
                {"Key": "waf.fail_open.enabled", "Value": "false"},
                {"Key": "connection_logs.s3.bucket", "Value": ""},
                {"Key": "access_logs.s3.enabled", "Value": "false"},
                {"Key": "zonal_shift.config.enabled", "Value": "false"},
                {"Key": "routing.http.desync_mitigation_mode", "Value": "defensive"},
                {"Key": "connection_logs.s3.prefix", "Value": ""},
                {"Key": "health_check_logs.s3.prefix", "Value": ""},
                {"Key": "routing.http.x_amzn_tls_version_and_cipher_suite.enabled", "Value": "false"},
                {"Key": "routing.http.preserve_host_header.enabled", "Value": "false"},
                {"Key": "load_balancing.cross_zone.enabled", "Value": "true"},
                {"Key": "health_check_logs.s3.enabled", "Value": "false"},
                {"Key": "health_check_logs.s3.bucket", "Value": ""},
                {"Key": "routing.http.xff_client_port.enabled", "Value": "false"},
                {"Key": "access_logs.s3.bucket", "Value": ""},
                {"Key": "deletion_protection.enabled", "Value": "false"},
                {"Key": "client_keep_alive.seconds", "Value": "3600"},
                {"Key": "routing.http.drop_invalid_header_fields.enabled", "Value": "false"},
                {"Key": "connection_logs.s3.enabled", "Value": "false"},
                {"Key": "idle_timeout.timeout_seconds", "Value": "60"}
            ]),
        ),
        ("Tags", json!([])),
    ];
    for (k, v) in defaults {
        if !obj.contains_key(k) {
            obj.insert(k.to_string(), v);
        }
    }
}

/// Real Cloud Control derives `SubnetMappings` from the bare `Subnets` list
/// when the caller does not supply explicit mappings.
fn derive_subnet_mappings(obj: &mut Map<String, Value>) {
    if obj.contains_key("SubnetMappings") {
        return;
    }
    let mappings: Vec<Value> = obj
        .get("Subnets")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|sid| sid.as_str())
                .map(|sid| json!({"SubnetId": sid}))
                .collect()
        })
        .unwrap_or_default();
    obj.insert("SubnetMappings".to_string(), json!(mappings));
}

impl CfnResourceShaper for ElbV2LoadBalancerShaper {
    fn shape_create(
        &self,
        desired_state: &Value,
        ctx: &ShapeContext<'_>,
    ) -> Result<ShapedResource, String> {
        let mut obj = desired_state.as_object().cloned().ok_or_else(|| {
            "AWS::ElasticLoadBalancingV2::LoadBalancer DesiredState must be a JSON object"
                .to_string()
        })?;

        strip_write_only(&mut obj);

        let lb_type = obj
            .get("Type")
            .and_then(|v| v.as_str())
            .unwrap_or("application")
            .to_string();
        let prefix = type_prefix(&lb_type);

        let name = obj
            .get("Name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("elb-{}", random_hex16()));
        let suffix = random_hex16();
        let full_name = format!("{prefix}/{name}/{suffix}");
        let arn = format!(
            "arn:aws:elasticloadbalancing:{region}:{account_id}:loadbalancer/{full_name}",
            region = ctx.region,
            account_id = ctx.account_id,
        );
        let dns_name = format!(
            "{name}-{rand}.{region}.elb.amazonaws.com",
            rand = dns_decimal_suffix(),
            region = ctx.region,
        );

        obj.insert("Name".to_string(), json!(name.clone()));
        obj.insert("LoadBalancerName".to_string(), json!(name));
        obj.insert("LoadBalancerFullName".to_string(), json!(full_name));
        obj.insert("LoadBalancerArn".to_string(), json!(arn.clone()));
        obj.insert("DNSName".to_string(), json!(dns_name));
        obj.insert(
            "CanonicalHostedZoneID".to_string(),
            json!(canonical_hosted_zone_id(&lb_type)),
        );

        fill_defaults(&mut obj);
        derive_subnet_mappings(&mut obj);

        Ok(ShapedResource {
            primary_identifier: arn,
            properties: Value::Object(obj),
        })
    }

    fn shape_update(
        &self,
        _previous: &Value,
        mut patched: Value,
        _ctx: &ShapeContext<'_>,
    ) -> Result<Value, String> {
        if let Some(obj) = patched.as_object_mut() {
            strip_write_only(obj);
            fill_defaults(obj);
        }
        Ok(patched)
    }
}
