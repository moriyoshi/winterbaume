//! Terraform converters for EventBridge resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_eventbridge::EventBridgeService;
use winterbaume_eventbridge::views::{
    EventBusView, EventsStateView, RuleView, TagView, TargetView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::events as events_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_cloudwatch_event_bus  (also aws_cloudwatch_event_bus / aws_eventbridge_event_bus)
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_event_bus` Terraform resources to/from EventBridge state.
pub struct AwsCloudwatchEventBusConverter {
    service: Arc<EventBridgeService>,
}

impl AwsCloudwatchEventBusConverter {
    pub fn new(service: Arc<EventBridgeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchEventBusConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_event_bus"
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

impl AwsCloudwatchEventBusConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: events_gen::EventBusTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudwatch_event_bus", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();

        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:events:{}:{}:event-bus/{}",
                region, ctx.default_account_id, name
            )
        });

        let dead_letter_config = attrs.get("dead_letter_config").cloned();
        let bus_view = EventBusView {
            name: name.clone(),
            arn: arn.clone(),
            policy: model.policy,
            dead_letter_config,
            ..Default::default()
        };

        // Tags
        let mut tag_map: HashMap<String, String> = HashMap::new();
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tag_map.insert(k.clone(), s.to_string());
                }
            }
        }
        if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tag_map.insert(k.clone(), s.to_string());
                }
            }
        }

        let tags: Vec<TagView> = tag_map
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let mut state_view = minimal_events_state_view_with_bus(bus_view);
        if !tags.is_empty() {
            state_view.tags.insert(arn, tags);
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
        for bus in view.event_buses.values() {
            // Skip the default event bus (created implicitly by AWS)
            if bus.name == "default" {
                continue;
            }
            let bus_tags: HashMap<String, String> = view
                .tags
                .get(&bus.arn)
                .map(|tags| {
                    tags.iter()
                        .map(|t| (t.key.clone(), t.value.clone()))
                        .collect()
                })
                .unwrap_or_default();
            let attrs = serde_json::json!({
                "id": bus.name,
                "name": bus.name,
                "arn": bus.arn,
                "policy": bus.policy,
                "event_source_name": null,
                "tags": bus_tags,
                "dead_letter_config": bus.dead_letter_config,
            });
            results.push(ExtractedResource {
                name: bus.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_event_rule
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_event_rule` Terraform resources to/from EventBridge state.
pub struct AwsCloudwatchEventRuleConverter {
    service: Arc<EventBridgeService>,
}

impl AwsCloudwatchEventRuleConverter {
    pub fn new(service: Arc<EventBridgeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchEventRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_event_rule"
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

impl AwsCloudwatchEventRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: events_gen::EventRuleTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudwatch_event_rule", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();

        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:events:{}:{}:rule/{}",
                region, ctx.default_account_id, name
            )
        });

        let event_bus_name = model
            .event_bus_name
            .unwrap_or_else(|| "default".to_string());

        // Tags
        let tags: Vec<TagView> = {
            let mut map: HashMap<String, String> = HashMap::new();
            if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
                for (k, v) in obj {
                    if let Some(s) = v.as_str() {
                        map.insert(k.clone(), s.to_string());
                    }
                }
            }
            if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
                for (k, v) in obj {
                    if let Some(s) = v.as_str() {
                        map.insert(k.clone(), s.to_string());
                    }
                }
            }
            map.into_iter()
                .map(|(k, v)| TagView { key: k, value: v })
                .collect()
        };

        let rule_view = RuleView {
            name: name.clone(),
            arn,
            event_pattern: model.event_pattern,
            schedule_expression: model.schedule_expression,
            state: model.state.unwrap_or_else(|| "ENABLED".to_string()),
            description: model.description,
            event_bus_name,
            targets: vec![],
            tags,
        };

        let state_view = minimal_events_state_view_with_rule(rule_view);
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
            let tags: HashMap<String, String> = rule
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": rule.name,
                "name": rule.name,
                "arn": rule.arn,
                "event_pattern": rule.event_pattern,
                "schedule_expression": rule.schedule_expression,
                "state": rule.state,
                "description": rule.description,
                "event_bus_name": rule.event_bus_name,
                "tags": tags,
            });
            results.push(ExtractedResource {
                name: rule.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_event_target
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_event_target` Terraform resources to/from EventBridge state.
pub struct AwsCloudwatchEventTargetConverter {
    service: Arc<EventBridgeService>,
}

impl AwsCloudwatchEventTargetConverter {
    pub fn new(service: Arc<EventBridgeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchEventTargetConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_event_target"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cloudwatch_event_rule"]
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

impl AwsCloudwatchEventTargetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: events_gen::EventTargetTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudwatch_event_target", e))?;

        let attrs = &instance.attributes;
        let rule_name = model.rule.clone();
        let target_id = model.target_id.clone();
        let target_arn = model.arn.clone();

        let _tags_all = attrs.get("tags_all");
        let _retry_policy = attrs.get("retry_policy");

        // Collect all nested target configuration blocks into `extra`.
        let extra = {
            let mut extra_map = serde_json::Map::new();
            for key in &[
                "input_transformer",
                "dead_letter_config",
                "run_command_targets",
                "ecs_target",
                "batch_target",
                "http_target",
                "kinesis_target",
                "sqs_target",
                "redshift_target",
                "sagemaker_pipeline_target",
                "event_bridge_event_bus_parameters",
            ] {
                if let Some(v) = attrs.get(*key) {
                    extra_map.insert(key.to_string(), v.clone());
                }
            }
            if extra_map.is_empty() {
                None
            } else {
                Some(serde_json::Value::Object(extra_map))
            }
        };

        let target_view = TargetView {
            id: target_id.clone(),
            arn: target_arn.clone(),
            input: model.input,
            input_path: model.input_path,
            extra,
        };

        // Snapshot, add target to the rule, restore.
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if let Some(rule) = state_view.rules.get_mut(&rule_name) {
            // Remove any existing target with the same ID to avoid duplicates.
            rule.targets.retain(|t| t.id != target_id);
            rule.targets.push(target_view);
        } else {
            // Rule not injected yet; create a placeholder rule.
            let rule_arn = format!(
                "arn:aws:events:{}:{}:rule/{}",
                region, ctx.default_account_id, rule_name
            );
            let rule_view = RuleView {
                name: rule_name.clone(),
                arn: rule_arn,
                event_pattern: None,
                schedule_expression: None,
                state: "ENABLED".to_string(),
                description: None,
                event_bus_name: model
                    .event_bus_name
                    .clone()
                    .unwrap_or_else(|| "default".to_string()),
                targets: vec![target_view],
                tags: vec![],
            };
            state_view.rules.insert(rule_name.clone(), rule_view);
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
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
            for target in &rule.targets {
                let id = format!("{}/{}", rule.name, target.id);
                // Merge extra nested blocks back into attributes.
                let mut attr_map = serde_json::json!({
                    "id": id,
                    "rule": rule.name,
                    "target_id": target.id,
                    "arn": target.arn,
                    "input": target.input,
                    "input_path": target.input_path,
                    "event_bus_name": rule.event_bus_name,
                    "retry_policy": null,
                    "dead_letter_config": null,
                    "ecs_target": [],
                    "batch_target": [],
                });
                if let Some(extra) = &target.extra {
                    if let (Some(map), Some(extra_map)) =
                        (attr_map.as_object_mut(), extra.as_object())
                    {
                        for (k, v) in extra_map {
                            map.insert(k.clone(), v.clone());
                        }
                    }
                }
                let attrs = attr_map;
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
// Helpers
// ---------------------------------------------------------------------------

fn minimal_events_state_view_with_rule(rule: RuleView) -> EventsStateView {
    let mut rules = HashMap::new();
    rules.insert(rule.name.clone(), rule);
    EventsStateView {
        rules,
        event_buses: HashMap::new(),
        archives: HashMap::new(),
        connections: HashMap::new(),
        api_destinations: HashMap::new(),
        replays: HashMap::new(),
        partner_event_sources: HashMap::new(),
        tags: HashMap::new(),
        ..Default::default()
    }
}

fn minimal_events_state_view_with_bus(bus: EventBusView) -> EventsStateView {
    let mut event_buses = HashMap::new();
    event_buses.insert(bus.name.clone(), bus);
    EventsStateView {
        rules: HashMap::new(),
        event_buses,
        archives: HashMap::new(),
        connections: HashMap::new(),
        api_destinations: HashMap::new(),
        replays: HashMap::new(),
        partner_event_sources: HashMap::new(),
        tags: HashMap::new(),
        ..Default::default()
    }
}
