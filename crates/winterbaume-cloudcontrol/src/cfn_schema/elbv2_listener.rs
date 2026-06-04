//! `AWS::ElasticLoadBalancingV2::Listener` shaper.
//!
//! Reference: <https://schema.cloudformation.us-east-1.amazonaws.com/aws-elasticloadbalancingv2-listener.json>
//! - `primaryIdentifier`: `/properties/ListenerArn`
//! - `readOnlyProperties`: `ListenerArn`
//! - `writeOnlyProperties`: `DefaultActions/*/AuthenticateOidcConfig/ClientSecret`
//! - The schema declares no top-level `default`s, but the real AWS read
//!   handler fills an 11-entry `ListenerAttributes` block and expands every
//!   `{Type: forward, TargetGroupArn}` default action with a full
//!   `ForwardConfig` (`TargetGroupStickinessConfig` + `TargetGroups`) (see
//!   issue #11 for the captured live-account snapshot). Mirror that handler
//!   behaviour here so the stored model matches `GetResource` on real AWS.
//!
//! Closely related to [[elbv2_target_group]] and [[elbv2_load_balancer]].

use serde_json::{Map, Value, json};
use uuid::Uuid;

use super::{CfnResourceShaper, ShapeContext, ShapedResource};

pub(super) struct ElbV2ListenerShaper;

fn random_hex16() -> String {
    Uuid::new_v4().simple().to_string()[..16].to_string()
}

/// Strip the nested writeOnly `ClientSecret` from every OIDC default action.
fn strip_write_only(obj: &mut Map<String, Value>) {
    let Some(actions) = obj.get_mut("DefaultActions").and_then(|v| v.as_array_mut()) else {
        return;
    };
    for action in actions {
        if let Some(oidc) = action
            .as_object_mut()
            .and_then(|o| o.get_mut("AuthenticateOidcConfig"))
            .and_then(|v| v.as_object_mut())
        {
            oidc.remove("ClientSecret");
        }
    }
}

/// Real AWS expands a bare `{Type: forward, TargetGroupArn}` default action
/// into a full `ForwardConfig` so the read model is self-describing.
fn enrich_default_actions(obj: &mut Map<String, Value>) {
    let Some(actions) = obj.get_mut("DefaultActions").and_then(|v| v.as_array_mut()) else {
        return;
    };
    for action in actions {
        let Some(map) = action.as_object_mut() else {
            continue;
        };
        let is_forward = map.get("Type").and_then(|v| v.as_str()) == Some("forward");
        if !is_forward || map.contains_key("ForwardConfig") {
            continue;
        }
        let tg_arn = map.get("TargetGroupArn").cloned().unwrap_or(Value::Null);
        map.insert(
            "ForwardConfig".to_string(),
            json!({
                "TargetGroupStickinessConfig": {"Enabled": false},
                "TargetGroups": [
                    {"TargetGroupArn": tg_arn, "Weight": 1}
                ]
            }),
        );
    }
}

fn fill_defaults(obj: &mut Map<String, Value>) {
    if obj.contains_key("ListenerAttributes") {
        return;
    }
    obj.insert(
        "ListenerAttributes".to_string(),
        json!([
            {"Key": "routing.http.response.server.enabled", "Value": "true"},
            {"Key": "routing.http.response.access_control_allow_headers.header_value", "Value": ""},
            {"Key": "routing.http.response.x_frame_options.header_value", "Value": ""},
            {"Key": "routing.http.response.access_control_allow_methods.header_value", "Value": ""},
            {"Key": "routing.http.response.access_control_allow_origin.header_value", "Value": ""},
            {"Key": "routing.http.response.access_control_allow_credentials.header_value", "Value": ""},
            {"Key": "routing.http.response.x_content_type_options.header_value", "Value": ""},
            {"Key": "routing.http.response.content_security_policy.header_value", "Value": ""},
            {"Key": "routing.http.response.access_control_expose_headers.header_value", "Value": ""},
            {"Key": "routing.http.response.strict_transport_security.header_value", "Value": ""},
            {"Key": "routing.http.response.access_control_max_age.header_value", "Value": ""}
        ]),
    );
}

/// Derive the listener ARN from the load balancer ARN. AWS layouts:
///
/// LB:       `arn:...:loadbalancer/app/<name>/<lb-id>`
/// Listener: `arn:...:listener/app/<name>/<lb-id>/<listener-id>`
fn listener_arn(lb_arn: &str, listener_id: &str) -> String {
    if let Some((prefix, lb_path)) = lb_arn.split_once(":loadbalancer/") {
        format!("{prefix}:listener/{lb_path}/{listener_id}")
    } else {
        format!("{lb_arn}/listener/{listener_id}")
    }
}

impl CfnResourceShaper for ElbV2ListenerShaper {
    fn shape_create(
        &self,
        desired_state: &Value,
        _ctx: &ShapeContext<'_>,
    ) -> Result<ShapedResource, String> {
        let mut obj = desired_state.as_object().cloned().ok_or_else(|| {
            "AWS::ElasticLoadBalancingV2::Listener DesiredState must be a JSON object".to_string()
        })?;

        strip_write_only(&mut obj);

        let lb_arn = obj
            .get("LoadBalancerArn")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                "AWS::ElasticLoadBalancingV2::Listener requires LoadBalancerArn in DesiredState"
                    .to_string()
            })?
            .to_string();
        let arn = listener_arn(&lb_arn, &random_hex16());
        obj.insert("ListenerArn".to_string(), json!(arn.clone()));

        enrich_default_actions(&mut obj);
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
            strip_write_only(obj);
            enrich_default_actions(obj);
            fill_defaults(obj);
        }
        Ok(patched)
    }
}
