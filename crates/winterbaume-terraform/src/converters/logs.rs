//! Terraform converters for CloudWatch Logs resources.
//!
//! `LogGroupTfModel` and `LogStreamTfModel` are generated from
//! `specs/logs.toml`. The ARN templates, the synthesised
//! `creation_time`, the `Option<i64>` `retention_in_days` raw read, and
//! the parent log-group placeholder fallback are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_cloudwatchlogs::CloudWatchLogsService;
use winterbaume_cloudwatchlogs::views::{
    AccountPolicyView, DeliveryDestinationConfigurationView, DeliveryDestinationPolicyView,
    DeliveryDestinationView, DeliverySourceView, DeliveryView, DestinationView, IndexPolicyView,
    LogAnomalyDetectorView, LogGroupView, LogStreamView, LogsStateView, MetricFilterView,
    MetricTransformationView, ResourcePolicyView, SubscriptionFilterView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::logs as logs_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_group
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_log_group` Terraform resources to/from CloudWatch Logs state.
pub struct AwsCloudwatchLogGroupConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogGroupConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_group"
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

impl AwsCloudwatchLogGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: logs_gen::LogGroupTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_cloudwatch_log_group", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:logs:{}:{}:log-group:{}",
                region, ctx.default_account_id, name
            )
        });

        // Option<i64> not in spec vocabulary — read raw.
        let retention_in_days = attrs
            .get("retention_in_days")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let group_view = LogGroupView {
            name: name.clone(),
            arn,
            creation_time: Utc::now().timestamp_millis(),
            retention_in_days,
            streams: HashMap::new(),
            tags: model.tags,
            kms_key_id: model.kms_key_id,
            data_protection_policy: None,
            deletion_protection_enabled: false,
        };

        let state_view = minimal_logs_state_view(group_view);
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
        for group in view.log_groups.values() {
            let attrs = serde_json::json!({
                "id": group.name,
                "name": group.name,
                "arn": group.arn,
                "retention_in_days": group.retention_in_days,
                "kms_key_id": group.kms_key_id,
                "tags": group.tags,
            });
            results.push(ExtractedResource {
                name: group.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_stream
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_log_stream` Terraform resources to/from CloudWatch Logs state.
pub struct AwsCloudwatchLogStreamConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogStreamConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogStreamConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_stream"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cloudwatch_log_group"]
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

impl AwsCloudwatchLogStreamConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: logs_gen::LogStreamTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_cloudwatch_log_stream", e))?;

        let name = model.name.clone();
        let log_group_name = model.log_group_name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:logs:{}:{}:log-group:{}:log-stream:{}",
                region, ctx.default_account_id, log_group_name, name
            )
        });

        let stream_view = LogStreamView {
            name: name.clone(),
            arn: arn.clone(),
            creation_time: Utc::now().timestamp_millis(),
            first_event_timestamp: None,
            last_event_timestamp: None,
            upload_sequence_token: String::new(),
        };

        // Snapshot, add stream to the parent log group, restore.
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if let Some(group) = state_view.log_groups.get_mut(&log_group_name) {
            group.streams.insert(name, stream_view);
        } else {
            // Parent group not injected yet: create a placeholder.
            let group_arn = format!(
                "arn:aws:logs:{}:{}:log-group:{}",
                region, ctx.default_account_id, log_group_name
            );
            let mut group = LogGroupView {
                name: log_group_name.clone(),
                arn: group_arn,
                creation_time: Utc::now().timestamp_millis(),
                retention_in_days: None,
                streams: HashMap::new(),
                tags: HashMap::new(),
                kms_key_id: None,
                data_protection_policy: None,
                deletion_protection_enabled: false,
            };
            group.streams.insert(name, stream_view);
            state_view.log_groups.insert(log_group_name, group);
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
        for group in view.log_groups.values() {
            for stream in group.streams.values() {
                let attrs = serde_json::json!({
                    "id": stream.name,
                    "name": stream.name,
                    "log_group_name": group.name,
                    "arn": stream.arn,
                });
                results.push(ExtractedResource {
                    name: stream.name.clone(),
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

fn minimal_logs_state_view(group: LogGroupView) -> LogsStateView {
    let mut log_groups = HashMap::new();
    log_groups.insert(group.name.clone(), group);
    LogsStateView {
        log_groups,
        ..empty_logs_state_view()
    }
}

fn empty_logs_state_view() -> LogsStateView {
    LogsStateView {
        log_groups: HashMap::new(),
        metric_filters: HashMap::new(),
        subscription_filters: HashMap::new(),
        resource_policies: HashMap::new(),
        destinations: HashMap::new(),
        export_tasks: HashMap::new(),
        delivery_sources: HashMap::new(),
        delivery_destinations: HashMap::new(),
        delivery_destination_policies: HashMap::new(),
        deliveries: HashMap::new(),
        account_policies: HashMap::new(),
        query_definitions: HashMap::new(),
        anomaly_detectors: HashMap::new(),
        anomalies: HashMap::new(),
        index_policies: HashMap::new(),
        transformers: HashMap::new(),
        integrations: HashMap::new(),
        import_tasks: HashMap::new(),
        scheduled_queries: HashMap::new(),
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_account_policy
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_log_account_policy` Terraform resources to/from
/// CloudWatch Logs state.
pub struct AwsCloudwatchLogAccountPolicyConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogAccountPolicyConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogAccountPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_account_policy"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: logs_gen::LogAccountPolicyTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_cloudwatch_log_account_policy", e)
                })?;

            // Account policies are keyed by `{policy_type}:{policy_name}` to
            // match the state-layer convention.
            let key = format!("{}:{}", model.policy_type, model.policy_name);
            let policy_view = AccountPolicyView {
                policy_name: model.policy_name.clone(),
                policy_document: model.policy_document,
                policy_type: model.policy_type,
                scope: model.scope,
                selection_criteria: model.selection_criteria,
                account_id: ctx.default_account_id.clone(),
                last_updated_time: Utc::now().timestamp_millis(),
            };

            let mut state_view = empty_logs_state_view();
            state_view.account_policies.insert(key, policy_view);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for policy in view.account_policies.values() {
                let attrs = serde_json::json!({
                    "id": policy.policy_name,
                    "policy_name": policy.policy_name,
                    "policy_document": policy.policy_document,
                    "policy_type": policy.policy_type,
                    "scope": policy.scope,
                    "selection_criteria": policy.selection_criteria,
                });
                results.push(ExtractedResource {
                    name: policy.policy_name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_anomaly_detector
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_log_anomaly_detector` Terraform resources to/from
/// CloudWatch Logs state.
pub struct AwsCloudwatchLogAnomalyDetectorConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogAnomalyDetectorConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogAnomalyDetectorConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_anomaly_detector"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: logs_gen::LogAnomalyDetectorTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_cloudwatch_log_anomaly_detector", e)
                })?;

            // `log_group_arn_list` is a Vec<String> — out of spec vocabulary,
            // read raw.
            let log_group_arn_list: Vec<String> = instance
                .attributes
                .get("log_group_arn_list")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();

            let detector_name = model.detector_name.unwrap_or_default();
            let arn = model.arn.unwrap_or_else(|| {
                format!(
                    "arn:aws:logs:{}:{}:anomaly-detector:{}",
                    region, ctx.default_account_id, detector_name
                )
            });
            let now = Utc::now().timestamp_millis();
            let status = if model.enabled {
                "ENABLED".to_string()
            } else {
                "PAUSED".to_string()
            };

            let detector_view = LogAnomalyDetectorView {
                anomaly_detector_arn: arn.clone(),
                detector_name,
                log_group_arn_list,
                evaluation_frequency: model.evaluation_frequency,
                filter_pattern: model.filter_pattern,
                anomaly_visibility_time: Some(model.anomaly_visibility_time),
                status,
                creation_time: now,
                last_modified_time: now,
                kms_key_id: model.kms_key_id,
            };

            let mut state_view = empty_logs_state_view();
            state_view.anomaly_detectors.insert(arn, detector_view);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for d in view.anomaly_detectors.values() {
                let attrs = serde_json::json!({
                    "id": d.anomaly_detector_arn,
                    "arn": d.anomaly_detector_arn,
                    "detector_name": d.detector_name,
                    "log_group_arn_list": d.log_group_arn_list,
                    "evaluation_frequency": d.evaluation_frequency,
                    "filter_pattern": d.filter_pattern,
                    "anomaly_visibility_time": d.anomaly_visibility_time,
                    "kms_key_id": d.kms_key_id,
                    "enabled": d.status == "ENABLED",
                });
                results.push(ExtractedResource {
                    name: d.detector_name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_data_protection_policy
// ---------------------------------------------------------------------------
// Modifier resource: snapshot+mutate the parent log group's
// `data_protection_policy` field.

/// Converts `aws_cloudwatch_log_data_protection_policy` Terraform resources
/// to/from CloudWatch Logs state.
pub struct AwsCloudwatchLogDataProtectionPolicyConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogDataProtectionPolicyConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogDataProtectionPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_data_protection_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cloudwatch_log_group"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: logs_gen::LogDataProtectionPolicyTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_cloudwatch_log_data_protection_policy", e)
                })?;

            let mut state_view = self
                .service
                .snapshot(&ctx.default_account_id, &region)
                .await;
            let mut warnings = vec![];
            if let Some(group) = state_view.log_groups.get_mut(&model.log_group_name) {
                group.data_protection_policy = Some(model.policy_document);
            } else {
                // Parent log group not present: create a placeholder.
                let arn = format!(
                    "arn:aws:logs:{}:{}:log-group:{}",
                    region, ctx.default_account_id, model.log_group_name
                );
                let group = LogGroupView {
                    name: model.log_group_name.clone(),
                    arn,
                    creation_time: Utc::now().timestamp_millis(),
                    retention_in_days: None,
                    streams: HashMap::new(),
                    tags: HashMap::new(),
                    kms_key_id: None,
                    data_protection_policy: Some(model.policy_document),
                    deletion_protection_enabled: false,
                };
                state_view
                    .log_groups
                    .insert(model.log_group_name.clone(), group);
                warnings.push(format!(
                    "aws_cloudwatch_log_data_protection_policy: parent log group '{}' not found; created placeholder",
                    model.log_group_name
                ));
            }
            self.service
                .restore(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult { region, warnings })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for group in view.log_groups.values() {
                if let Some(doc) = &group.data_protection_policy {
                    let attrs = serde_json::json!({
                        "id": group.name,
                        "log_group_name": group.name,
                        "policy_document": doc,
                    });
                    results.push(ExtractedResource {
                        name: group.name.clone(),
                        account_id: ctx.default_account_id.clone(),
                        region: ctx.default_region.clone(),
                        attributes: attrs,
                    });
                }
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_delivery
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_log_delivery` Terraform resources to/from
/// CloudWatch Logs state.
pub struct AwsCloudwatchLogDeliveryConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogDeliveryConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogDeliveryConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_delivery"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_cloudwatch_log_delivery_source",
            "aws_cloudwatch_log_delivery_destination",
        ]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: logs_gen::LogDeliveryTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_cloudwatch_log_delivery", e))?;

            let id = model
                .id
                .clone()
                .unwrap_or_else(|| uuid_like_id(&model.delivery_source_name));
            let delivery_view = DeliveryView {
                id: id.clone(),
                source: model.delivery_source_name,
                destination: model.delivery_destination_arn,
                tags: model.tags,
            };

            let mut state_view = empty_logs_state_view();
            state_view.deliveries.insert(id, delivery_view);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for d in view.deliveries.values() {
                let arn = format!(
                    "arn:aws:logs:{}:{}:delivery:{}",
                    ctx.default_region, ctx.default_account_id, d.id
                );
                let attrs = serde_json::json!({
                    "id": d.id,
                    "arn": arn,
                    "delivery_source_name": d.source,
                    "delivery_destination_arn": d.destination,
                    "tags": d.tags,
                });
                results.push(ExtractedResource {
                    name: d.id.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_delivery_destination
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_log_delivery_destination` Terraform resources
/// to/from CloudWatch Logs state.
pub struct AwsCloudwatchLogDeliveryDestinationConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogDeliveryDestinationConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogDeliveryDestinationConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_delivery_destination"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: logs_gen::LogDeliveryDestinationTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_cloudwatch_log_delivery_destination", e)
                })?;

            // `delivery_destination_configuration` is a nested block (a list
            // with one map carrying `destination_resource_arn`). Read raw.
            let destination_resource_arn = instance
                .attributes
                .get("delivery_destination_configuration")
                .and_then(|v| {
                    // Accept either a list (TF SDK v2 style) or a map.
                    if let Some(arr) = v.as_array() {
                        arr.iter()
                            .next()
                            .and_then(|first| first.get("destination_resource_arn"))
                            .and_then(|x| x.as_str())
                            .map(|s| s.to_string())
                    } else {
                        v.get("destination_resource_arn")
                            .and_then(|x| x.as_str())
                            .map(|s| s.to_string())
                    }
                });

            let arn = model.arn.unwrap_or_else(|| {
                format!(
                    "arn:aws:logs:{}:{}:delivery-destination:{}",
                    region, ctx.default_account_id, model.name
                )
            });

            let dest_view = DeliveryDestinationView {
                name: model.name.clone(),
                arn,
                delivery_destination_type: model.delivery_destination_type,
                output_format: model.output_format,
                delivery_destination_configuration: destination_resource_arn.map(|arn| {
                    DeliveryDestinationConfigurationView {
                        destination_resource_arn: arn,
                    }
                }),
            };

            let mut state_view = empty_logs_state_view();
            state_view
                .delivery_destinations
                .insert(model.name, dest_view);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for d in view.delivery_destinations.values() {
                let cfg = d.delivery_destination_configuration.as_ref().map(|c| {
                    serde_json::json!([{
                        "destination_resource_arn": c.destination_resource_arn,
                    }])
                });
                let attrs = serde_json::json!({
                    "id": d.name,
                    "name": d.name,
                    "arn": d.arn,
                    "delivery_destination_type": d.delivery_destination_type,
                    "output_format": d.output_format,
                    "delivery_destination_configuration": cfg,
                });
                results.push(ExtractedResource {
                    name: d.name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_delivery_destination_policy
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_log_delivery_destination_policy` Terraform
/// resources to/from CloudWatch Logs state.
pub struct AwsCloudwatchLogDeliveryDestinationPolicyConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogDeliveryDestinationPolicyConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogDeliveryDestinationPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_delivery_destination_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cloudwatch_log_delivery_destination"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: logs_gen::LogDeliveryDestinationPolicyTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_cloudwatch_log_delivery_destination_policy", e)
                })?;

            let policy_view = DeliveryDestinationPolicyView {
                delivery_destination_name: model.delivery_destination_name.clone(),
                policy: model.delivery_destination_policy,
            };

            let mut state_view = empty_logs_state_view();
            state_view
                .delivery_destination_policies
                .insert(model.delivery_destination_name, policy_view);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for p in view.delivery_destination_policies.values() {
                let attrs = serde_json::json!({
                    "id": p.delivery_destination_name,
                    "delivery_destination_name": p.delivery_destination_name,
                    "delivery_destination_policy": p.policy,
                });
                results.push(ExtractedResource {
                    name: p.delivery_destination_name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_delivery_source
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_log_delivery_source` Terraform resources to/from
/// CloudWatch Logs state.
pub struct AwsCloudwatchLogDeliverySourceConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogDeliverySourceConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogDeliverySourceConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_delivery_source"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: logs_gen::LogDeliverySourceTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_cloudwatch_log_delivery_source", e)
                })?;

            // `resource_arn` is singular on the TF side; the view stores a Vec.
            let resource_arns: Vec<String> =
                model.resource_arn.map(|a| vec![a]).unwrap_or_default();

            let source_view = DeliverySourceView {
                name: model.name.clone(),
                log_type: model.log_type,
                service: model.service,
                resource_arns,
            };

            let mut state_view = empty_logs_state_view();
            state_view.delivery_sources.insert(model.name, source_view);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for s in view.delivery_sources.values() {
                let arn = format!(
                    "arn:aws:logs:{}:{}:delivery-source:{}",
                    ctx.default_region, ctx.default_account_id, s.name
                );
                let attrs = serde_json::json!({
                    "id": s.name,
                    "name": s.name,
                    "arn": arn,
                    "log_type": s.log_type,
                    "service": s.service,
                    "resource_arn": s.resource_arns.first(),
                });
                results.push(ExtractedResource {
                    name: s.name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_destination
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_log_destination` Terraform resources to/from
/// CloudWatch Logs state.
pub struct AwsCloudwatchLogDestinationConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogDestinationConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogDestinationConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_destination"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: logs_gen::LogDestinationTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_cloudwatch_log_destination", e))?;

            let arn = model.arn.unwrap_or_else(|| {
                format!(
                    "arn:aws:logs:{}:{}:destination:{}",
                    region, ctx.default_account_id, model.name
                )
            });

            let dest_view = DestinationView {
                destination_name: model.name.clone(),
                target_arn: model.target_arn,
                role_arn: model.role_arn,
                access_policy: None,
                arn,
                creation_time: Utc::now().timestamp_millis(),
                tags: model.tags,
            };

            let mut state_view = empty_logs_state_view();
            state_view.destinations.insert(model.name, dest_view);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for d in view.destinations.values() {
                let attrs = serde_json::json!({
                    "id": d.destination_name,
                    "name": d.destination_name,
                    "arn": d.arn,
                    "role_arn": d.role_arn,
                    "target_arn": d.target_arn,
                    "tags": d.tags,
                });
                results.push(ExtractedResource {
                    name: d.destination_name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_destination_policy
// ---------------------------------------------------------------------------
// Modifier resource: snapshot+mutate the parent destination's
// `access_policy` field.

/// Converts `aws_cloudwatch_log_destination_policy` Terraform resources to/from
/// CloudWatch Logs state.
pub struct AwsCloudwatchLogDestinationPolicyConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogDestinationPolicyConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogDestinationPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_destination_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cloudwatch_log_destination"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: logs_gen::LogDestinationPolicyTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_cloudwatch_log_destination_policy", e)
                })?;

            let mut state_view = self
                .service
                .snapshot(&ctx.default_account_id, &region)
                .await;
            let mut warnings = vec![];
            if let Some(dest) = state_view.destinations.get_mut(&model.destination_name) {
                dest.access_policy = Some(model.access_policy);
            } else {
                // Parent destination not present: create a placeholder so the
                // policy is not silently dropped.
                let arn = format!(
                    "arn:aws:logs:{}:{}:destination:{}",
                    region, ctx.default_account_id, model.destination_name
                );
                let dest = DestinationView {
                    destination_name: model.destination_name.clone(),
                    target_arn: String::new(),
                    role_arn: String::new(),
                    access_policy: Some(model.access_policy),
                    arn,
                    creation_time: Utc::now().timestamp_millis(),
                    tags: HashMap::new(),
                };
                state_view
                    .destinations
                    .insert(model.destination_name.clone(), dest);
                warnings.push(format!(
                    "aws_cloudwatch_log_destination_policy: parent destination '{}' not found; created placeholder",
                    model.destination_name
                ));
            }
            self.service
                .restore(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult { region, warnings })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for d in view.destinations.values() {
                if let Some(policy) = &d.access_policy {
                    let attrs = serde_json::json!({
                        "id": d.destination_name,
                        "destination_name": d.destination_name,
                        "access_policy": policy,
                    });
                    results.push(ExtractedResource {
                        name: d.destination_name.clone(),
                        account_id: ctx.default_account_id.clone(),
                        region: ctx.default_region.clone(),
                        attributes: attrs,
                    });
                }
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_index_policy
// ---------------------------------------------------------------------------
// Note: state keys `index_policies` by `log_group_identifier`. The TF
// attribute is `log_group_name` (per provider docs); we treat it as the
// identifier (name or ARN both accepted).

/// Converts `aws_cloudwatch_log_index_policy` Terraform resources to/from
/// CloudWatch Logs state.
pub struct AwsCloudwatchLogIndexPolicyConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogIndexPolicyConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogIndexPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_index_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cloudwatch_log_group"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: logs_gen::LogIndexPolicyTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_cloudwatch_log_index_policy", e)
                })?;

            let identifier = model.log_group_name.clone();
            let policy_view = IndexPolicyView {
                policy_name: format!("{}-index-policy", identifier),
                log_group_identifier: identifier.clone(),
                policy_document: model.policy_document,
                last_update_time: Utc::now().timestamp_millis(),
            };

            let mut state_view = empty_logs_state_view();
            state_view.index_policies.insert(identifier, policy_view);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for p in view.index_policies.values() {
                let attrs = serde_json::json!({
                    "id": p.log_group_identifier,
                    "log_group_name": p.log_group_identifier,
                    "policy_document": p.policy_document,
                });
                results.push(ExtractedResource {
                    name: p.log_group_identifier.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_metric_filter
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_log_metric_filter` Terraform resources to/from
/// CloudWatch Logs state.
pub struct AwsCloudwatchLogMetricFilterConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogMetricFilterConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogMetricFilterConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_metric_filter"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cloudwatch_log_group"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: logs_gen::LogMetricFilterTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_cloudwatch_log_metric_filter", e)
                })?;

            // `metric_transformation` is a TF nested block: a list of one
            // map carrying `name`, `namespace`, `value`. Read raw.
            let metric_transformations: Vec<MetricTransformationView> = instance
                .attributes
                .get("metric_transformation")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|t| {
                            let obj = t.as_object()?;
                            Some(MetricTransformationView {
                                metric_namespace: obj
                                    .get("namespace")
                                    .and_then(|x| x.as_str())
                                    .unwrap_or_default()
                                    .to_string(),
                                metric_name: obj
                                    .get("name")
                                    .and_then(|x| x.as_str())
                                    .unwrap_or_default()
                                    .to_string(),
                                metric_value: obj
                                    .get("value")
                                    .and_then(|x| x.as_str())
                                    .unwrap_or_default()
                                    .to_string(),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            let key = format!("{}:{}", model.log_group_name, model.name);
            let filter_view = MetricFilterView {
                filter_name: model.name,
                log_group_name: model.log_group_name,
                filter_pattern: model.pattern,
                metric_transformations,
                creation_time: Utc::now().timestamp_millis(),
            };

            let mut state_view = empty_logs_state_view();
            state_view.metric_filters.insert(key, filter_view);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for f in view.metric_filters.values() {
                let transformations: Vec<serde_json::Value> = f
                    .metric_transformations
                    .iter()
                    .map(|t| {
                        serde_json::json!({
                            "name": t.metric_name,
                            "namespace": t.metric_namespace,
                            "value": t.metric_value,
                        })
                    })
                    .collect();
                let attrs = serde_json::json!({
                    "id": f.filter_name,
                    "name": f.filter_name,
                    "log_group_name": f.log_group_name,
                    "pattern": f.filter_pattern,
                    "metric_transformation": transformations,
                });
                results.push(ExtractedResource {
                    name: f.filter_name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_resource_policy
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_log_resource_policy` Terraform resources to/from
/// CloudWatch Logs state.
pub struct AwsCloudwatchLogResourcePolicyConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogResourcePolicyConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogResourcePolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_resource_policy"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: logs_gen::LogResourcePolicyTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_cloudwatch_log_resource_policy", e)
                })?;

            let policy_view = ResourcePolicyView {
                policy_name: model.policy_name.clone(),
                policy_document: model.policy_document,
                last_updated_time: Utc::now().timestamp_millis(),
            };

            let mut state_view = empty_logs_state_view();
            state_view
                .resource_policies
                .insert(model.policy_name, policy_view);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for p in view.resource_policies.values() {
                let attrs = serde_json::json!({
                    "id": p.policy_name,
                    "policy_name": p.policy_name,
                    "policy_document": p.policy_document,
                });
                results.push(ExtractedResource {
                    name: p.policy_name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_log_subscription_filter
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_log_subscription_filter` Terraform resources
/// to/from CloudWatch Logs state.
pub struct AwsCloudwatchLogSubscriptionFilterConverter {
    service: Arc<CloudWatchLogsService>,
}

impl AwsCloudwatchLogSubscriptionFilterConverter {
    pub fn new(service: Arc<CloudWatchLogsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchLogSubscriptionFilterConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_log_subscription_filter"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cloudwatch_log_group"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: logs_gen::LogSubscriptionFilterTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_cloudwatch_log_subscription_filter", e)
                })?;

            let key = format!("{}:{}", model.log_group_name, model.name);
            let filter_view = SubscriptionFilterView {
                filter_name: model.name,
                log_group_name: model.log_group_name,
                filter_pattern: model.filter_pattern,
                destination_arn: model.destination_arn,
                role_arn: model.role_arn,
                creation_time: Utc::now().timestamp_millis(),
            };

            let mut state_view = empty_logs_state_view();
            state_view.subscription_filters.insert(key, filter_view);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for f in view.subscription_filters.values() {
                let attrs = serde_json::json!({
                    "id": f.filter_name,
                    "name": f.filter_name,
                    "log_group_name": f.log_group_name,
                    "filter_pattern": f.filter_pattern,
                    "destination_arn": f.destination_arn,
                    "role_arn": f.role_arn,
                });
                results.push(ExtractedResource {
                    name: f.filter_name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Derive a stable, deterministic-ish ID from a seed string. CloudWatch Logs
/// delivery IDs are 12-character lowercase alphanumeric in AWS; for our
/// purposes any non-empty string is acceptable as long as the same seed
/// produces the same value.
fn uuid_like_id(seed: &str) -> String {
    let mut hasher: u64 = 0xcbf29ce484222325;
    for b in seed.as_bytes() {
        hasher ^= *b as u64;
        hasher = hasher.wrapping_mul(0x100000001b3);
    }
    format!("{:012x}", hasher)
}
