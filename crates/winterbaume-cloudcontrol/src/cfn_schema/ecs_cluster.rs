//! `AWS::ECS::Cluster` shaper.
//!
//! Reference: <https://schema.cloudformation.us-east-1.amazonaws.com/aws-ecs-cluster.json>
//! - `primaryIdentifier`: `/properties/ClusterName`
//! - `readOnlyProperties`: `Arn`
//! - `writeOnlyProperties`: `ServiceConnectDefaults`
//! - The schema itself does not declare per-property `default`s, but the real
//!   AWS read handler always reports `DefaultCapacityProviderStrategy: []`
//!   for omitted input (see issue #8 for the captured live-account snapshot).
//!   Mirror that handler behaviour here so the stored model matches
//!   `GetResource` on real AWS.

use serde_json::{Map, Value, json};
use uuid::Uuid;

use super::{CfnResourceShaper, ShapeContext, ShapedResource};

pub(super) struct EcsClusterShaper;

const WRITE_ONLY: &[&str] = &["ServiceConnectDefaults"];

fn strip_write_only(obj: &mut Map<String, Value>) {
    for key in WRITE_ONLY {
        obj.remove(*key);
    }
}

fn fill_defaults(obj: &mut Map<String, Value>) {
    let defaults: [(&str, Value); 1] = [("DefaultCapacityProviderStrategy", json!([]))];
    for (k, v) in defaults {
        if !obj.contains_key(k) {
            obj.insert(k.to_string(), v);
        }
    }
}

impl CfnResourceShaper for EcsClusterShaper {
    fn shape_create(
        &self,
        desired_state: &Value,
        ctx: &ShapeContext<'_>,
    ) -> Result<ShapedResource, String> {
        let mut obj = desired_state
            .as_object()
            .cloned()
            .ok_or_else(|| "AWS::ECS::Cluster DesiredState must be a JSON object".to_string())?;

        strip_write_only(&mut obj);

        let cluster_name = obj
            .get("ClusterName")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        let arn = format!(
            "arn:aws:ecs:{region}:{account_id}:cluster/{cluster_name}",
            region = ctx.region,
            account_id = ctx.account_id,
        );
        obj.insert("ClusterName".to_string(), json!(cluster_name));
        obj.insert("Arn".to_string(), json!(arn));

        fill_defaults(&mut obj);

        Ok(ShapedResource {
            primary_identifier: cluster_name,
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
