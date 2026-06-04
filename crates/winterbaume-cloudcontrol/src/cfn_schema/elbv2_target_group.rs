//! `AWS::ElasticLoadBalancingV2::TargetGroup` shaper.
//!
//! Reference: <https://schema.cloudformation.us-east-1.amazonaws.com/aws-elasticloadbalancingv2-targetgroup.json>
//! - `primaryIdentifier`: `/properties/TargetGroupArn`
//! - `readOnlyProperties`: `TargetGroupArn`, `TargetGroupName`,
//!   `TargetGroupFullName`, `LoadBalancerArns`
//! - `writeOnlyProperties`: none declared
//! - The schema itself does not declare per-property `default`s, but the real
//!   AWS read handler reports an extensive set of health-check defaults plus
//!   a 14-entry `TargetGroupAttributes` block (see issue #9 for the captured
//!   live-account snapshot). Mirror that handler behaviour here so the stored
//!   model matches `GetResource` on real AWS.
//!
//! Closely related to [[kms_key]], [[dynamodb_table]], and [[ecs_cluster]].

use serde_json::{Map, Value, json};
use uuid::Uuid;

use super::{CfnResourceShaper, ShapeContext, ShapedResource};

pub(super) struct ElbV2TargetGroupShaper;

fn random_id_suffix() -> String {
    Uuid::new_v4().simple().to_string()[..16].to_string()
}

fn fill_defaults(obj: &mut Map<String, Value>) {
    let defaults: [(&str, Value); 13] = [
        ("IpAddressType", json!("ipv4")),
        ("HealthCheckEnabled", json!(true)),
        ("HealthCheckIntervalSeconds", json!(30)),
        ("HealthCheckProtocol", json!("HTTP")),
        ("HealthCheckPort", json!("traffic-port")),
        ("HealthCheckTimeoutSeconds", json!(5)),
        ("HealthyThresholdCount", json!(5)),
        ("UnhealthyThresholdCount", json!(2)),
        ("ProtocolVersion", json!("HTTP1")),
        ("Matcher", json!({"HttpCode": "200"})),
        ("Targets", json!([])),
        (
            "TargetGroupAttributes",
            json!([
                {"Key": "stickiness.type", "Value": "lb_cookie"},
                {"Key": "stickiness.app_cookie.duration_seconds", "Value": "86400"},
                {"Key": "target_group_health.dns_failover.minimum_healthy_targets.count", "Value": "1"},
                {"Key": "load_balancing.cross_zone.enabled", "Value": "use_load_balancer_configuration"},
                {"Key": "stickiness.lb_cookie.duration_seconds", "Value": "86400"},
                {"Key": "target_group_health.dns_failover.minimum_healthy_targets.percentage", "Value": "off"},
                {"Key": "stickiness.enabled", "Value": "false"},
                {"Key": "target_group_health.unhealthy_state_routing.minimum_healthy_targets.percentage", "Value": "off"},
                {"Key": "slow_start.duration_seconds", "Value": "0"},
                {"Key": "deregistration_delay.timeout_seconds", "Value": "300"},
                {"Key": "target_group_health.unhealthy_state_routing.minimum_healthy_targets.count", "Value": "1"},
                {"Key": "load_balancing.algorithm.anomaly_mitigation", "Value": "off"},
                {"Key": "stickiness.app_cookie.cookie_name", "Value": ""},
                {"Key": "load_balancing.algorithm.type", "Value": "round_robin"}
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

impl CfnResourceShaper for ElbV2TargetGroupShaper {
    fn shape_create(
        &self,
        desired_state: &Value,
        ctx: &ShapeContext<'_>,
    ) -> Result<ShapedResource, String> {
        let mut obj = desired_state.as_object().cloned().ok_or_else(|| {
            "AWS::ElasticLoadBalancingV2::TargetGroup DesiredState must be a JSON object"
                .to_string()
        })?;

        let name = obj
            .get("Name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("tg-{}", random_id_suffix()));
        let suffix = random_id_suffix();
        let full_name = format!("targetgroup/{name}/{suffix}");
        let arn = format!(
            "arn:aws:elasticloadbalancing:{region}:{account_id}:{full_name}",
            region = ctx.region,
            account_id = ctx.account_id,
        );

        obj.insert("Name".to_string(), json!(name.clone()));
        obj.insert("TargetGroupName".to_string(), json!(name));
        obj.insert("TargetGroupFullName".to_string(), json!(full_name));
        obj.insert("TargetGroupArn".to_string(), json!(arn.clone()));
        obj.insert("LoadBalancerArns".to_string(), json!([]));

        fill_defaults(&mut obj);

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
            fill_defaults(obj);
        }
        Ok(patched)
    }
}
