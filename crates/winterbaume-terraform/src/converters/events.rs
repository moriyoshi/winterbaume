//! Terraform converters for EventBridge resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_eventbridge::EventBridgeService;
use winterbaume_eventbridge::views::{
    ApiDestinationView, ArchiveView, ConnectionView, EndpointView, EventBusView, EventsStateView,
    RuleView, TagView, TargetView,
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
// aws_cloudwatch_event_api_destination
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_event_api_destination` Terraform resources to/from EventBridge state.
pub struct AwsCloudwatchEventApiDestinationConverter {
    service: Arc<EventBridgeService>,
}

impl AwsCloudwatchEventApiDestinationConverter {
    pub fn new(service: Arc<EventBridgeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchEventApiDestinationConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_event_api_destination"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cloudwatch_event_connection"]
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

impl AwsCloudwatchEventApiDestinationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: events_gen::EventApiDestinationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_cloudwatch_event_api_destination", e)
            })?;

        let name = model.name.clone();
        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:events:{}:{}:api-destination/{}",
                region, ctx.default_account_id, name
            )
        });

        let api_dest_view = ApiDestinationView {
            name: name.clone(),
            arn,
            description: model.description,
            connection_arn: model.connection_arn,
            invocation_endpoint: model.invocation_endpoint,
            http_method: model.http_method,
            invocation_rate_limit: model.invocation_rate_limit_per_second,
            state: "ACTIVE".to_string(),
            creation_time: Utc::now().timestamp() as f64,
        };

        let mut api_destinations = HashMap::new();
        api_destinations.insert(name, api_dest_view);
        let state_view = EventsStateView {
            api_destinations,
            ..Default::default()
        };
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
        for dest in view.api_destinations.values() {
            let attrs = serde_json::json!({
                "id": dest.name,
                "name": dest.name,
                "arn": dest.arn,
                "description": dest.description,
                "connection_arn": dest.connection_arn,
                "invocation_endpoint": dest.invocation_endpoint,
                "http_method": dest.http_method,
                "invocation_rate_limit_per_second": dest.invocation_rate_limit,
            });
            results.push(ExtractedResource {
                name: dest.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_event_archive
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_event_archive` Terraform resources to/from EventBridge state.
pub struct AwsCloudwatchEventArchiveConverter {
    service: Arc<EventBridgeService>,
}

impl AwsCloudwatchEventArchiveConverter {
    pub fn new(service: Arc<EventBridgeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchEventArchiveConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_event_archive"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cloudwatch_event_bus"]
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

impl AwsCloudwatchEventArchiveConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: events_gen::EventArchiveTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudwatch_event_archive", e))?;

        let name = model.name.clone();
        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:events:{}:{}:archive/{}",
                region, ctx.default_account_id, name
            )
        });

        let archive_view = ArchiveView {
            name: name.clone(),
            arn,
            event_source_arn: model.event_source_arn,
            description: model.description,
            event_pattern: model.event_pattern,
            retention_days: model.retention_days,
            state: "ENABLED".to_string(),
            creation_time: Utc::now().timestamp() as f64,
            size_bytes: 0,
        };

        let mut archives = HashMap::new();
        archives.insert(name, archive_view);
        let state_view = EventsStateView {
            archives,
            ..Default::default()
        };
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
        for archive in view.archives.values() {
            let attrs = serde_json::json!({
                "id": archive.name,
                "name": archive.name,
                "arn": archive.arn,
                "event_source_arn": archive.event_source_arn,
                "description": archive.description,
                "event_pattern": archive.event_pattern,
                "retention_days": archive.retention_days,
            });
            results.push(ExtractedResource {
                name: archive.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_event_bus_policy
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_event_bus_policy` Terraform resources to/from EventBridge state.
///
/// Sets the resource-based policy on an existing event bus. The event bus
/// must exist (the converter creates a placeholder bus if needed so order
/// independence is preserved).
pub struct AwsCloudwatchEventBusPolicyConverter {
    service: Arc<EventBridgeService>,
}

impl AwsCloudwatchEventBusPolicyConverter {
    pub fn new(service: Arc<EventBridgeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchEventBusPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_event_bus_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cloudwatch_event_bus"]
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

impl AwsCloudwatchEventBusPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: events_gen::EventBusPolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudwatch_event_bus_policy", e))?;

        let event_bus_name = model
            .event_bus_name
            .clone()
            .unwrap_or_else(|| "default".to_string());

        // Snapshot, set policy on the bus (or create a placeholder), restore.
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if let Some(bus) = state_view.event_buses.get_mut(&event_bus_name) {
            bus.policy = Some(model.policy);
        } else {
            let bus_arn = format!(
                "arn:aws:events:{}:{}:event-bus/{}",
                region, ctx.default_account_id, event_bus_name
            );
            state_view.event_buses.insert(
                event_bus_name.clone(),
                EventBusView {
                    name: event_bus_name.clone(),
                    arn: bus_arn,
                    policy: Some(model.policy),
                    ..Default::default()
                },
            );
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
        for bus in view.event_buses.values() {
            let Some(policy) = &bus.policy else { continue };
            let attrs = serde_json::json!({
                "id": bus.name,
                "event_bus_name": bus.name,
                "policy": policy,
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
// aws_cloudwatch_event_connection
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_event_connection` Terraform resources to/from EventBridge state.
///
/// The nested `auth_parameters` block carries credential material that
/// winterbaume's state model stores as a JSON-encoded string. The converter
/// passes the raw nested block through to keep round-tripping intact without
/// inventing schema for every authorisation type.
pub struct AwsCloudwatchEventConnectionConverter {
    service: Arc<EventBridgeService>,
}

impl AwsCloudwatchEventConnectionConverter {
    pub fn new(service: Arc<EventBridgeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchEventConnectionConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_event_connection"
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

impl AwsCloudwatchEventConnectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: events_gen::EventConnectionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudwatch_event_connection", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:events:{}:{}:connection/{}",
                region, ctx.default_account_id, name
            )
        });

        // Capture the nested `auth_parameters` block verbatim as a JSON string
        // so the state retains whatever credential payload Terraform recorded.
        let auth_parameters = attrs
            .get("auth_parameters")
            .map(|v| v.to_string())
            .unwrap_or_else(|| "{}".to_string());

        let connection_view = ConnectionView {
            name: name.clone(),
            arn,
            state: "AUTHORIZED".to_string(),
            description: model.description,
            authorization_type: model.authorization_type,
            auth_parameters,
            creation_time: Utc::now().timestamp() as f64,
        };

        let mut connections = HashMap::new();
        connections.insert(name, connection_view);
        let state_view = EventsStateView {
            connections,
            ..Default::default()
        };
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
        for conn in view.connections.values() {
            let auth_params: serde_json::Value =
                serde_json::from_str(&conn.auth_parameters).unwrap_or(serde_json::Value::Null);
            let attrs = serde_json::json!({
                "id": conn.name,
                "name": conn.name,
                "arn": conn.arn,
                "description": conn.description,
                "authorization_type": conn.authorization_type,
                "auth_parameters": auth_params,
            });
            results.push(ExtractedResource {
                name: conn.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_event_endpoint
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_event_endpoint` Terraform resources to/from EventBridge state.
///
/// `routing_config`, `replication_config`, and `event_bus` are nested blocks
/// in Terraform; winterbaume's state stores them as JSON-serialised strings
/// (the former two) and a list of ARNs (the latter). The converter projects
/// the nested blocks straight into those fields.
pub struct AwsCloudwatchEventEndpointConverter {
    service: Arc<EventBridgeService>,
}

impl AwsCloudwatchEventEndpointConverter {
    pub fn new(service: Arc<EventBridgeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchEventEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_event_endpoint"
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

impl AwsCloudwatchEventEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: events_gen::EventEndpointTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudwatch_event_endpoint", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:events:{}:{}:endpoint/{}",
                region, ctx.default_account_id, name
            )
        });

        let routing_config = attrs.get("routing_config").map(|v| v.to_string());
        let replication_config = attrs.get("replication_config").map(|v| v.to_string());

        // `event_bus` is a list of nested blocks each with `event_bus_arn`.
        let event_buses: Vec<String> = attrs
            .get("event_bus")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|b| {
                        b.get("event_bus_arn")
                            .and_then(|v| v.as_str())
                            .map(str::to_string)
                    })
                    .collect()
            })
            .unwrap_or_default();

        let endpoint_url = model.endpoint_url.unwrap_or_default();
        let endpoint_id = attrs
            .get("endpoint_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let now = Utc::now().timestamp() as f64;
        let endpoint_view = EndpointView {
            name: name.clone(),
            arn,
            description: model.description,
            routing_config,
            replication_config,
            event_buses,
            role_arn: model.role_arn,
            state: "ACTIVE".to_string(),
            endpoint_id,
            endpoint_url,
            creation_time: now,
            last_modified_time: now,
        };

        let mut endpoints = HashMap::new();
        endpoints.insert(name, endpoint_view);
        let state_view = EventsStateView {
            endpoints,
            ..Default::default()
        };
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
        for endpoint in view.endpoints.values() {
            let event_bus_blocks: Vec<serde_json::Value> = endpoint
                .event_buses
                .iter()
                .map(|arn| serde_json::json!({"event_bus_arn": arn}))
                .collect();
            let routing_config: serde_json::Value = endpoint
                .routing_config
                .as_deref()
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or(serde_json::Value::Null);
            let replication_config: serde_json::Value = endpoint
                .replication_config
                .as_deref()
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or(serde_json::Value::Null);
            let attrs = serde_json::json!({
                "id": endpoint.name,
                "name": endpoint.name,
                "arn": endpoint.arn,
                "description": endpoint.description,
                "role_arn": endpoint.role_arn,
                "endpoint_id": endpoint.endpoint_id,
                "endpoint_url": endpoint.endpoint_url,
                "event_bus": event_bus_blocks,
                "routing_config": routing_config,
                "replication_config": replication_config,
            });
            results.push(ExtractedResource {
                name: endpoint.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_event_permission
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_event_permission` Terraform resources to/from EventBridge state.
///
/// Each permission adds (or replaces) a single statement keyed by `statement_id`
/// in the target event bus's resource policy. Extract is best-effort: it walks
/// the parsed policy on each bus and emits one resource per statement.
pub struct AwsCloudwatchEventPermissionConverter {
    service: Arc<EventBridgeService>,
}

impl AwsCloudwatchEventPermissionConverter {
    pub fn new(service: Arc<EventBridgeService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchEventPermissionConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_event_permission"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cloudwatch_event_bus"]
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

impl AwsCloudwatchEventPermissionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: events_gen::EventPermissionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudwatch_event_permission", e))?;

        let event_bus_name = model
            .event_bus_name
            .clone()
            .unwrap_or_else(|| "default".to_string());
        let action = model
            .action
            .unwrap_or_else(|| "events:PutEvents".to_string());

        // Snapshot, mutate the bus policy, restore.
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        let bus_arn = state_view
            .event_buses
            .get(&event_bus_name)
            .map(|b| b.arn.clone())
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:events:{}:{}:event-bus/{}",
                    region, ctx.default_account_id, event_bus_name
                )
            });

        // Optional `condition` nested block: { type, key, value }.
        let condition = instance
            .attributes
            .get("condition")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned();

        let mut statement = serde_json::json!({
            "Sid": model.statement_id,
            "Effect": "Allow",
            "Principal": if model.principal == "*" {
                serde_json::json!("*")
            } else {
                serde_json::json!({"AWS": format!("arn:aws:iam::{}:root", model.principal)})
            },
            "Action": action,
            "Resource": bus_arn,
        });
        if let Some(cond) = condition
            && let (Some(key), Some(value), Some(ty)) = (
                cond.get("key").and_then(|v| v.as_str()),
                cond.get("value").and_then(|v| v.as_str()),
                cond.get("type").and_then(|v| v.as_str()),
            )
        {
            statement["Condition"] = serde_json::json!({
                ty: { key: value }
            });
        }

        let bus = state_view
            .event_buses
            .entry(event_bus_name.clone())
            .or_insert_with(|| EventBusView {
                name: event_bus_name.clone(),
                arn: bus_arn.clone(),
                policy: None,
                ..Default::default()
            });

        let mut policy_doc = bus
            .policy
            .as_deref()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
            .unwrap_or_else(|| {
                serde_json::json!({
                    "Version": "2012-10-17",
                    "Statement": [],
                })
            });
        if let Some(statements) = policy_doc
            .get_mut("Statement")
            .and_then(|v| v.as_array_mut())
        {
            let sid = model.statement_id.as_str();
            if let Some(pos) = statements
                .iter()
                .position(|s| s.get("Sid").and_then(|v| v.as_str()) == Some(sid))
            {
                statements[pos] = statement;
            } else {
                statements.push(statement);
            }
        }
        bus.policy = Some(policy_doc.to_string());

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
        for bus in view.event_buses.values() {
            let Some(policy_str) = &bus.policy else {
                continue;
            };
            let Ok(policy_doc) = serde_json::from_str::<serde_json::Value>(policy_str) else {
                continue;
            };
            let Some(statements) = policy_doc.get("Statement").and_then(|v| v.as_array()) else {
                continue;
            };
            for stmt in statements {
                let Some(sid) = stmt.get("Sid").and_then(|v| v.as_str()) else {
                    continue;
                };
                let action = stmt
                    .get("Action")
                    .and_then(|v| v.as_str())
                    .unwrap_or("events:PutEvents")
                    .to_string();
                let principal = match stmt.get("Principal") {
                    Some(serde_json::Value::String(s)) => s.clone(),
                    Some(serde_json::Value::Object(o)) => o
                        .get("AWS")
                        .and_then(|v| v.as_str())
                        .map(|aws| {
                            aws.strip_prefix("arn:aws:iam::")
                                .and_then(|rest| rest.strip_suffix(":root"))
                                .unwrap_or(aws)
                                .to_string()
                        })
                        .unwrap_or_default(),
                    _ => String::new(),
                };
                let attrs = serde_json::json!({
                    "id": format!("{}/{}", bus.name, sid),
                    "event_bus_name": bus.name,
                    "principal": principal,
                    "action": action,
                    "statement_id": sid,
                });
                results.push(ExtractedResource {
                    name: format!("{}/{}", bus.name, sid),
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
