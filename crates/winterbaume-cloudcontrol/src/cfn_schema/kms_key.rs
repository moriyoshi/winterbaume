//! `AWS::KMS::Key` shaper.
//!
//! Reference: <https://schema.cloudformation.us-east-1.amazonaws.com/aws-kms-key.json>
//! - `primaryIdentifier`: `/properties/KeyId`
//! - `readOnlyProperties`: `KeyId`, `Arn`
//! - `writeOnlyProperties`: `PendingWindowInDays`,
//!   `BypassPolicyLockoutSafetyCheck`, `RotationPeriodInDays`
//! - Schema defaults populated when omitted from `DesiredState`:
//!   `Enabled`, `EnableKeyRotation`, `KeySpec`, `KeyUsage`, `MultiRegion`,
//!   `Origin`, `Tags`.

use serde_json::{Map, Value, json};
use uuid::Uuid;

use super::{CfnResourceShaper, ShapeContext, ShapedResource};

pub(super) struct KmsKeyShaper;

const WRITE_ONLY: &[&str] = &[
    "PendingWindowInDays",
    "BypassPolicyLockoutSafetyCheck",
    "RotationPeriodInDays",
];

fn strip_write_only(obj: &mut Map<String, Value>) {
    for key in WRITE_ONLY {
        obj.remove(*key);
    }
}

fn fill_defaults(obj: &mut Map<String, Value>) {
    let defaults: [(&str, Value); 7] = [
        ("Enabled", json!(true)),
        ("EnableKeyRotation", json!(false)),
        ("KeySpec", json!("SYMMETRIC_DEFAULT")),
        ("KeyUsage", json!("ENCRYPT_DECRYPT")),
        ("MultiRegion", json!(false)),
        ("Origin", json!("AWS_KMS")),
        ("Tags", json!([])),
    ];
    for (k, v) in defaults {
        if !obj.contains_key(k) {
            obj.insert(k.to_string(), v);
        }
    }
}

impl CfnResourceShaper for KmsKeyShaper {
    fn shape_create(
        &self,
        desired_state: &Value,
        ctx: &ShapeContext<'_>,
    ) -> Result<ShapedResource, String> {
        let mut obj = desired_state
            .as_object()
            .cloned()
            .ok_or_else(|| "AWS::KMS::Key DesiredState must be a JSON object".to_string())?;

        strip_write_only(&mut obj);

        let key_id = obj
            .get("KeyId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        let arn = format!(
            "arn:aws:kms:{region}:{account_id}:key/{key_id}",
            region = ctx.region,
            account_id = ctx.account_id,
        );
        obj.insert("KeyId".to_string(), json!(key_id));
        obj.insert("Arn".to_string(), json!(arn));

        fill_defaults(&mut obj);

        Ok(ShapedResource {
            primary_identifier: key_id,
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
