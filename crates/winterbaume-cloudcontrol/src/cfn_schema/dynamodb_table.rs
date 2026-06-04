//! `AWS::DynamoDB::Table` shaper.
//!
//! Reference: <https://schema.cloudformation.us-east-1.amazonaws.com/aws-dynamodb-table.json>
//! - `primaryIdentifier`: `/properties/TableName`
//! - `readOnlyProperties`: `Arn`, `StreamArn`
//! - `writeOnlyProperties`: `ImportSourceSpecification`
//! - The schema itself does not declare per-property `default`s, but the real
//!   AWS read handler always reports the seven specification blocks below
//!   (see issue #7 for the captured live-account snapshot). Mirror that
//!   handler behaviour here so the stored model matches `GetResource` on
//!   real AWS.

use serde_json::{Map, Value, json};
use uuid::Uuid;

use super::{CfnResourceShaper, ShapeContext, ShapedResource};

pub(super) struct DynamoDbTableShaper;

const WRITE_ONLY: &[&str] = &["ImportSourceSpecification"];

fn strip_write_only(obj: &mut Map<String, Value>) {
    for key in WRITE_ONLY {
        obj.remove(*key);
    }
}

fn fill_defaults(obj: &mut Map<String, Value>) {
    let defaults: [(&str, Value); 7] = [
        (
            "ContributorInsightsSpecification",
            json!({"Enabled": false}),
        ),
        ("DeletionProtectionEnabled", json!(false)),
        (
            "PointInTimeRecoverySpecification",
            json!({"PointInTimeRecoveryEnabled": false}),
        ),
        ("SSESpecification", json!({"SSEEnabled": false})),
        ("TimeToLiveSpecification", json!({"Enabled": false})),
        ("Tags", json!([])),
        (
            "WarmThroughput",
            json!({"ReadUnitsPerSecond": 12000, "WriteUnitsPerSecond": 4000}),
        ),
    ];
    for (k, v) in defaults {
        if !obj.contains_key(k) {
            obj.insert(k.to_string(), v);
        }
    }
}

impl CfnResourceShaper for DynamoDbTableShaper {
    fn shape_create(
        &self,
        desired_state: &Value,
        ctx: &ShapeContext<'_>,
    ) -> Result<ShapedResource, String> {
        let mut obj = desired_state
            .as_object()
            .cloned()
            .ok_or_else(|| "AWS::DynamoDB::Table DesiredState must be a JSON object".to_string())?;

        strip_write_only(&mut obj);

        let table_name = obj
            .get("TableName")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        let arn = format!(
            "arn:aws:dynamodb:{region}:{account_id}:table/{table_name}",
            region = ctx.region,
            account_id = ctx.account_id,
        );
        obj.insert("TableName".to_string(), json!(table_name));
        obj.insert("Arn".to_string(), json!(arn));

        fill_defaults(&mut obj);

        Ok(ShapedResource {
            primary_identifier: table_name,
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
