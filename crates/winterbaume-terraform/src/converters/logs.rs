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
use winterbaume_cloudwatchlogs::views::{LogGroupView, LogStreamView, LogsStateView};
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
