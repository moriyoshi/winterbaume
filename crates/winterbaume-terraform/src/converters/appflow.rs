//! Terraform converters for AWS AppFlow resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Map, Value};
use winterbaume_appflow::AppFlowService;
use winterbaume_appflow::views::{AppFlowStateView, FlowView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_appflow_flow
// ---------------------------------------------------------------------------

pub struct AwsAppFlowFlowConverter {
    service: Arc<AppFlowService>,
}

impl AwsAppFlowFlowConverter {
    pub fn new(service: Arc<AppFlowService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppFlowFlowConverter {
    fn resource_type(&self) -> &str {
        "aws_appflow_flow"
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

impl AwsAppFlowFlowConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_appflow_flow")?;
        let region = extract_region(attrs, &ctx.default_region);
        let description = optional_str(attrs, "description");
        let kms_arn = optional_str(attrs, "kms_arn");

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:appflow:{}:{}:flow/{}",
                region, ctx.default_account_id, name
            )
        });

        // Reshape Terraform provider-schema blocks into AWS REST JSON shape.
        // Terraform uses snake_case keys and represents singleton blocks as
        // length-1 arrays; the AppFlow handlers downstream try to deserialize
        // these via `serde_json::from_value` against the AWS shapes, so the
        // converter must do the translation here.
        let trigger_config = tf_to_aws_trigger_config(attrs.get("trigger_config"))
            .unwrap_or(Value::Object(Map::new()));
        let source_flow_config = tf_to_aws_source_flow_config(attrs.get("source_flow_config"))
            .unwrap_or(Value::Object(Map::new()));
        let destination_flow_config_list =
            tf_to_aws_destination_flow_config_list(attrs.get("destination_flow_config"));
        let tasks = tf_to_aws_tasks(attrs.get("task"));

        let tags: HashMap<String, String> = attrs
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let user = format!("arn:aws:iam::{}:user/terraform", ctx.default_account_id);
        let now = chrono::Utc::now().timestamp();

        let mut state_view = AppFlowStateView::default();
        state_view.flows.insert(
            name.to_string(),
            FlowView {
                flow_name: name.to_string(),
                flow_arn: arn,
                description,
                kms_arn,
                flow_status: "Active".to_string(),
                flow_status_message: None,
                trigger_config,
                source_flow_config,
                destination_flow_config_list,
                tasks,
                metadata_catalog_config: attrs.get("metadata_catalog_config").cloned(),
                created_at: now,
                last_updated_at: now,
                created_by: user.clone(),
                last_updated_by: user,
                schema_version: 1,
                tags,
                last_execution_id: None,
            },
        );
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

        let mut resources = Vec::new();
        for (name, flow) in &view.flows {
            let mut attrs = serde_json::Map::new();
            attrs.insert("id".to_string(), serde_json::json!(flow.flow_arn));
            attrs.insert("arn".to_string(), serde_json::json!(flow.flow_arn));
            attrs.insert("name".to_string(), serde_json::json!(flow.flow_name));
            if let Some(d) = &flow.description {
                attrs.insert("description".to_string(), serde_json::json!(d));
            }
            if let Some(k) = &flow.kms_arn {
                attrs.insert("kms_arn".to_string(), serde_json::json!(k));
            }
            attrs.insert("trigger_config".to_string(), flow.trigger_config.clone());
            attrs.insert(
                "source_flow_config".to_string(),
                flow.source_flow_config.clone(),
            );
            attrs.insert(
                "destination_flow_config".to_string(),
                flow.destination_flow_config_list.clone(),
            );
            attrs.insert("task".to_string(), flow.tasks.clone());
            if !flow.tags.is_empty() {
                attrs.insert("tags".to_string(), serde_json::json!(flow.tags));
            }
            resources.push(ExtractedResource {
                name: name.clone(),
                attributes: serde_json::Value::Object(attrs),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
            });
        }

        Ok(resources)
    }
}

// ---------------------------------------------------------------------------
// Terraform -> AWS shape reshape helpers
// ---------------------------------------------------------------------------

/// Convert a snake_case identifier to camelCase.
fn snake_to_camel(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut upper_next = false;
    for c in s.chars() {
        if c == '_' {
            upper_next = true;
        } else if upper_next {
            out.extend(c.to_uppercase());
            upper_next = false;
        } else {
            out.push(c);
        }
    }
    out
}

/// Lift a Terraform singleton-block value into a single object. Terraform
/// represents singleton nested blocks as a length-1 array of objects; if the
/// value is already an object (or null/missing) it is returned as-is.
fn lift_singleton(value: Option<&Value>) -> Option<Value> {
    match value {
        Some(Value::Array(arr)) => {
            if arr.is_empty() {
                None
            } else {
                Some(arr[0].clone())
            }
        }
        Some(v) if v.is_null() => None,
        Some(v) => Some(v.clone()),
        None => None,
    }
}

/// Generic camelCase-key transformer for an object. Recurses into nested
/// objects and arrays, leaving leaf scalars alone. Singleton-block lifting
/// must be done explicitly by callers; this helper only renames keys.
fn camelize_keys(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = Map::new();
            for (k, v) in map {
                out.insert(snake_to_camel(&k), camelize_keys(v));
            }
            Value::Object(out)
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(camelize_keys).collect()),
        other => other,
    }
}

/// camelCase-key transformer that ALSO lifts every length-1 array-of-object
/// into the object itself. Use this inside connector-properties subtrees
/// where the AWS shape never has genuine list-of-object fields, so every
/// singleton-block array in tfstate must collapse to a plain object.
fn camelize_and_lift_singletons(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = Map::new();
            for (k, v) in map {
                out.insert(snake_to_camel(&k), camelize_and_lift_singletons(v));
            }
            Value::Object(out)
        }
        Value::Array(mut arr)
            if arr.len() == 1 && matches!(arr.first(), Some(Value::Object(_))) =>
        {
            camelize_and_lift_singletons(arr.remove(0))
        }
        Value::Array(arr) => {
            Value::Array(arr.into_iter().map(camelize_and_lift_singletons).collect())
        }
        other => other,
    }
}

/// Connector-property polymorphic key map (snake_case Terraform block name ->
/// AWS shape PascalCase member name). These appear as nested keys inside
/// `source_connector_properties`, `destination_connector_properties`, and
/// `connector_operator`.
fn connector_property_key(snake: &str) -> &str {
    match snake {
        "amplitude" => "Amplitude",
        "custom_connector" => "CustomConnector",
        "customer_profiles" => "CustomerProfiles",
        "datadog" => "Datadog",
        "dynatrace" => "Dynatrace",
        "event_bridge" => "EventBridge",
        "google_analytics" => "GoogleAnalytics",
        "honeycode" => "Honeycode",
        "infor_nexus" => "InforNexus",
        "lookout_metrics" => "LookoutMetrics",
        "marketo" => "Marketo",
        "pardot" => "Pardot",
        "redshift" => "Redshift",
        "s3" => "S3",
        "sapo_data" => "SAPOData",
        "salesforce" => "Salesforce",
        "service_now" => "ServiceNow",
        "singular" => "Singular",
        "slack" => "Slack",
        "snowflake" => "Snowflake",
        "trendmicro" => "Trendmicro",
        "upsolver" => "Upsolver",
        "veeva" => "Veeva",
        "zendesk" => "Zendesk",
        // Trigger properties polymorphic key.
        "scheduled" => "Scheduled",
        other => other,
    }
}

/// Reshape `trigger_config` (Terraform singleton block) into AWS shape.
fn tf_to_aws_trigger_config(value: Option<&Value>) -> Option<Value> {
    let lifted = lift_singleton(value)?;
    let obj = lifted.as_object()?;

    let mut out = Map::new();
    if let Some(t) = obj.get("trigger_type").and_then(|v| v.as_str()) {
        out.insert("triggerType".to_string(), Value::String(t.to_string()));
    }
    if let Some(props) = lift_singleton(obj.get("trigger_properties")) {
        if let Some(props_obj) = props.as_object() {
            let mut props_out = Map::new();
            for (k, v) in props_obj {
                let aws_key = connector_property_key(k);
                let lifted_inner = lift_singleton(Some(v)).unwrap_or(Value::Null);
                if !lifted_inner.is_null() {
                    props_out.insert(
                        aws_key.to_string(),
                        camelize_and_lift_singletons(lifted_inner),
                    );
                }
            }
            if !props_out.is_empty() {
                out.insert("triggerProperties".to_string(), Value::Object(props_out));
            }
        }
    }
    Some(Value::Object(out))
}

/// Reshape `source_flow_config` (Terraform singleton block) into AWS shape.
fn tf_to_aws_source_flow_config(value: Option<&Value>) -> Option<Value> {
    let lifted = lift_singleton(value)?;
    let obj = lifted.as_object()?;

    let mut out = Map::new();
    copy_str(obj, "api_version", "apiVersion", &mut out);
    copy_str(
        obj,
        "connector_profile_name",
        "connectorProfileName",
        &mut out,
    );
    copy_str(obj, "connector_type", "connectorType", &mut out);
    if let Some(ipc) = lift_singleton(obj.get("incremental_pull_config")) {
        if !ipc.is_null() {
            out.insert(
                "incrementalPullConfig".to_string(),
                camelize_and_lift_singletons(ipc),
            );
        }
    }
    if let Some(scp) = tf_to_aws_connector_properties(obj.get("source_connector_properties")) {
        out.insert("sourceConnectorProperties".to_string(), scp);
    }
    Some(Value::Object(out))
}

/// Reshape the `destination_flow_config` block list into the AWS
/// `destinationFlowConfigList` shape.
fn tf_to_aws_destination_flow_config_list(value: Option<&Value>) -> Value {
    let arr = match value {
        Some(Value::Array(arr)) => arr,
        _ => return Value::Array(vec![]),
    };
    let mut out = Vec::with_capacity(arr.len());
    for entry in arr {
        let Some(obj) = entry.as_object() else {
            continue;
        };
        let mut item = Map::new();
        copy_str(obj, "api_version", "apiVersion", &mut item);
        copy_str(
            obj,
            "connector_profile_name",
            "connectorProfileName",
            &mut item,
        );
        copy_str(obj, "connector_type", "connectorType", &mut item);
        if let Some(dcp) =
            tf_to_aws_connector_properties(obj.get("destination_connector_properties"))
        {
            item.insert("destinationConnectorProperties".to_string(), dcp);
        }
        out.push(Value::Object(item));
    }
    Value::Array(out)
}

/// Reshape a connector-properties singleton block. Each entry inside is itself
/// a singleton block keyed by snake_case connector name.
fn tf_to_aws_connector_properties(value: Option<&Value>) -> Option<Value> {
    let lifted = lift_singleton(value)?;
    let obj = lifted.as_object()?;
    let mut out = Map::new();
    for (k, v) in obj {
        let lifted_inner = match lift_singleton(Some(v)) {
            Some(v) if !v.is_null() => v,
            _ => continue,
        };
        out.insert(
            connector_property_key(k).to_string(),
            camelize_and_lift_singletons(lifted_inner),
        );
    }
    if out.is_empty() {
        None
    } else {
        Some(Value::Object(out))
    }
}

/// Reshape the `task` block list into the AWS `tasks` shape.
fn tf_to_aws_tasks(value: Option<&Value>) -> Value {
    let arr = match value {
        Some(Value::Array(arr)) => arr,
        _ => return Value::Array(vec![]),
    };
    let mut out = Vec::with_capacity(arr.len());
    for entry in arr {
        let Some(obj) = entry.as_object() else {
            continue;
        };
        let mut item = Map::new();
        copy_str(obj, "task_type", "taskType", &mut item);
        copy_str(obj, "destination_field", "destinationField", &mut item);
        if let Some(sf) = obj.get("source_fields") {
            item.insert("sourceFields".to_string(), sf.clone());
        }
        if let Some(tp) = obj.get("task_properties") {
            // task_properties is a `map(string)` in the provider schema, so
            // it arrives as a JSON object already. Pass it through.
            if !tp.is_null() {
                item.insert("taskProperties".to_string(), tp.clone());
            }
        }
        if let Some(co) = lift_singleton(obj.get("connector_operator")) {
            if let Some(co_obj) = co.as_object() {
                let mut co_out = Map::new();
                for (k, v) in co_obj {
                    if let Some(s) = v.as_str() {
                        if !s.is_empty() {
                            co_out.insert(
                                connector_property_key(k).to_string(),
                                Value::String(s.to_string()),
                            );
                        }
                    }
                }
                if !co_out.is_empty() {
                    item.insert("connectorOperator".to_string(), Value::Object(co_out));
                }
            }
        }
        out.push(Value::Object(item));
    }
    Value::Array(out)
}

/// Copy a string-valued field from `src` into `dst` while renaming the key,
/// dropping null / non-string / empty values.
fn copy_str(src: &Map<String, Value>, src_key: &str, dst_key: &str, dst: &mut Map<String, Value>) {
    if let Some(s) = src.get(src_key).and_then(|v| v.as_str()) {
        if !s.is_empty() {
            dst.insert(dst_key.to_string(), Value::String(s.to_string()));
        }
    }
}

#[cfg(test)]
mod reshape_tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn lifts_singleton_arrays() {
        let v = json!([{"a": 1}]);
        assert_eq!(lift_singleton(Some(&v)), Some(json!({"a": 1})));
        let empty = json!([]);
        assert_eq!(lift_singleton(Some(&empty)), None);
        assert_eq!(lift_singleton(None), None);
    }

    #[test]
    fn camelizes_nested_keys() {
        let v = json!({"foo_bar": {"baz_qux": 1}, "list": [{"snake_case": "v"}]});
        assert_eq!(
            camelize_keys(v),
            json!({"fooBar": {"bazQux": 1}, "list": [{"snakeCase": "v"}]})
        );
    }

    #[test]
    fn reshapes_trigger_config() {
        let v = json!([{
            "trigger_type": "Scheduled",
            "trigger_properties": [{
                "scheduled": [{
                    "schedule_expression": "rate(1hours)",
                    "data_pull_mode": "Incremental",
                }],
            }],
        }]);
        let out = tf_to_aws_trigger_config(Some(&v)).unwrap();
        assert_eq!(out["triggerType"], "Scheduled");
        assert_eq!(
            out["triggerProperties"]["Scheduled"]["scheduleExpression"],
            "rate(1hours)"
        );
        assert_eq!(
            out["triggerProperties"]["Scheduled"]["dataPullMode"],
            "Incremental"
        );
    }

    #[test]
    fn reshapes_source_flow_config_with_s3() {
        let v = json!([{
            "connector_type": "S3",
            "source_connector_properties": [{
                "s3": [{
                    "bucket_name": "src-bucket",
                    "bucket_prefix": "in/",
                }],
            }],
        }]);
        let out = tf_to_aws_source_flow_config(Some(&v)).unwrap();
        assert_eq!(out["connectorType"], "S3");
        assert_eq!(
            out["sourceConnectorProperties"]["S3"]["bucketName"],
            "src-bucket"
        );
        assert_eq!(
            out["sourceConnectorProperties"]["S3"]["bucketPrefix"],
            "in/"
        );
    }

    #[test]
    fn reshapes_destination_flow_config_list_with_s3() {
        let v = json!([{
            "connector_type": "S3",
            "destination_connector_properties": [{
                "s3": [{
                    "bucket_name": "dst-bucket",
                    "s3_output_format_config": [{
                        "file_type": "JSON",
                    }],
                }],
            }],
        }]);
        let out = tf_to_aws_destination_flow_config_list(Some(&v));
        let arr = out.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["connectorType"], "S3");
        assert_eq!(
            arr[0]["destinationConnectorProperties"]["S3"]["bucketName"],
            "dst-bucket"
        );
        assert_eq!(
            arr[0]["destinationConnectorProperties"]["S3"]["s3OutputFormatConfig"]["fileType"],
            "JSON"
        );
    }

    #[test]
    fn reshapes_tasks() {
        let v = json!([{
            "task_type": "Map",
            "source_fields": ["id"],
            "destination_field": "id",
            "task_properties": {"DESTINATION_DATA_TYPE": "string"},
            "connector_operator": [{"s3": "NO_OP"}],
        }]);
        let out = tf_to_aws_tasks(Some(&v));
        let arr = out.as_array().unwrap();
        assert_eq!(arr[0]["taskType"], "Map");
        assert_eq!(arr[0]["destinationField"], "id");
        assert_eq!(arr[0]["sourceFields"][0], "id");
        assert_eq!(arr[0]["taskProperties"]["DESTINATION_DATA_TYPE"], "string");
        assert_eq!(arr[0]["connectorOperator"]["S3"], "NO_OP");
    }
}
