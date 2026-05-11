//! Terraform converters for CloudWatch resources.
//!
//! `MetricAlarmTfModel` is generated from `specs/cloudwatch.toml`. The
//! ARN template, `state_value` / `state_reason` constants, the `f64`
//! `threshold` field, the action lists, and the `dimensions` map block
//! are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_cloudwatch::CloudWatchService;
use winterbaume_cloudwatch::views::{
    CloudwatchStateView, CompositeAlarmView, DashboardView, DimensionView, MetricAlarmView,
    MetricStreamView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::cloudwatch as cloudwatch_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_cloudwatch_metric_alarm
// ---------------------------------------------------------------------------

/// Converts `aws_cloudwatch_metric_alarm` Terraform resources to/from CloudWatch state.
pub struct AwsCloudwatchMetricAlarmConverter {
    service: Arc<CloudWatchService>,
}

impl AwsCloudwatchMetricAlarmConverter {
    pub fn new(service: Arc<CloudWatchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchMetricAlarmConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_metric_alarm"
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

impl AwsCloudwatchMetricAlarmConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cloudwatch_gen::MetricAlarmTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudwatch_metric_alarm", e))?;

        let attrs = &instance.attributes;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:cloudwatch:{}:{}:alarm:{}",
                region, ctx.default_account_id, model.alarm_name
            )
        });

        // f64 threshold not in spec vocabulary — read raw.
        let threshold = attrs
            .get("threshold")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let alarm_actions: Vec<String> = attrs
            .get("alarm_actions")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let ok_actions: Vec<String> = attrs
            .get("ok_actions")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let insufficient_data_actions: Vec<String> = attrs
            .get("insufficient_data_actions")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        // dimensions is a map in Terraform: { "InstanceId" = "i-xxx" }
        let dimensions: Vec<DimensionView> = attrs
            .get("dimensions")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| {
                        v.as_str().map(|s| DimensionView {
                            name: k.clone(),
                            value: s.to_string(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let alarm_view = MetricAlarmView {
            alarm_name: model.alarm_name.clone(),
            alarm_arn: arn,
            metric_name: model.metric_name,
            namespace: model.namespace,
            threshold,
            comparison_operator: model.comparison_operator,
            evaluation_periods: model.evaluation_periods,
            period: model.period,
            statistic: model.statistic.unwrap_or_else(|| "Average".to_string()),
            state_value: "OK".to_string(),
            state_reason: String::new(),
            actions_enabled: model.actions_enabled,
            alarm_description: model.alarm_description,
            alarm_actions,
            ok_actions,
            insufficient_data_actions,
            dimensions,
            unit: model.unit,
        };

        let mut state_view = CloudwatchStateView {
            alarms: HashMap::new(),
            composite_alarms: HashMap::new(),
            dashboards: HashMap::new(),
            insight_rules: HashMap::new(),
            resource_tags: HashMap::new(),
            anomaly_detectors: vec![],
            metric_streams: HashMap::new(),
            alarm_mute_rules: HashMap::new(),
            ..Default::default()
        };
        state_view.alarms.insert(model.alarm_name, alarm_view);
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
        for alarm in view.alarms.values() {
            let dimensions: HashMap<String, String> = alarm
                .dimensions
                .iter()
                .map(|d| (d.name.clone(), d.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": alarm.alarm_name,
                "alarm_name": alarm.alarm_name,
                "arn": alarm.alarm_arn,
                "metric_name": alarm.metric_name,
                "namespace": alarm.namespace,
                "threshold": alarm.threshold,
                "comparison_operator": alarm.comparison_operator,
                "evaluation_periods": alarm.evaluation_periods,
                "period": alarm.period,
                "statistic": alarm.statistic,
                "actions_enabled": alarm.actions_enabled,
                "alarm_description": alarm.alarm_description,
                "alarm_actions": alarm.alarm_actions,
                "ok_actions": alarm.ok_actions,
                "insufficient_data_actions": alarm.insufficient_data_actions,
                "dimensions": dimensions,
                "unit": alarm.unit,
            });
            results.push(ExtractedResource {
                name: alarm.alarm_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_composite_alarm
// ---------------------------------------------------------------------------

pub struct AwsCloudwatchCompositeAlarmConverter {
    service: Arc<CloudWatchService>,
}

impl AwsCloudwatchCompositeAlarmConverter {
    pub fn new(service: Arc<CloudWatchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchCompositeAlarmConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_composite_alarm"
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

impl AwsCloudwatchCompositeAlarmConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: cloudwatch_gen::CompositeAlarmTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_cloudwatch_composite_alarm", e))?;

        let alarm_name = model.alarm_name;
        let alarm_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:cloudwatch:{}:{}:alarm:{}",
                region, ctx.default_account_id, alarm_name
            )
        });

        let extract_str_arr = |key: &str| -> Vec<String> {
            attrs
                .get(key)
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default()
        };

        let view = CompositeAlarmView {
            alarm_name: alarm_name.clone(),
            alarm_arn,
            alarm_rule: model.alarm_rule,
            alarm_description: model.alarm_description,
            actions_enabled: model.actions_enabled,
            alarm_actions: extract_str_arr("alarm_actions"),
            ok_actions: extract_str_arr("ok_actions"),
            insufficient_data_actions: extract_str_arr("insufficient_data_actions"),
            state_value: "OK".to_string(),
            state_reason: "Unchecked: Initial alarm creation".to_string(),
        };

        let mut state_view = CloudwatchStateView::default();
        state_view.composite_alarms.insert(alarm_name, view);
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
        for a in view.composite_alarms.values() {
            let attrs = serde_json::json!({
                "id": a.alarm_name,
                "alarm_name": a.alarm_name,
                "arn": a.alarm_arn,
                "alarm_rule": a.alarm_rule,
                "alarm_description": a.alarm_description,
                "actions_enabled": a.actions_enabled,
                "alarm_actions": a.alarm_actions,
                "ok_actions": a.ok_actions,
                "insufficient_data_actions": a.insufficient_data_actions,
            });
            results.push(ExtractedResource {
                name: a.alarm_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_dashboard
// ---------------------------------------------------------------------------

pub struct AwsCloudwatchDashboardConverter {
    service: Arc<CloudWatchService>,
}

impl AwsCloudwatchDashboardConverter {
    pub fn new(service: Arc<CloudWatchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchDashboardConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_dashboard"
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

impl AwsCloudwatchDashboardConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cloudwatch_gen::DashboardTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudwatch_dashboard", e))?;

        let name = model.dashboard_name;
        let arn = model.dashboard_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:cloudwatch::{}:dashboard/{}",
                ctx.default_account_id, name
            )
        });

        let view = DashboardView {
            dashboard_name: name.clone(),
            dashboard_body: model.dashboard_body,
            dashboard_arn: arn,
            last_modified: chrono::Utc::now().to_rfc3339(),
        };

        let mut state_view = CloudwatchStateView::default();
        state_view.dashboards.insert(name, view);
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
        for d in view.dashboards.values() {
            let attrs = serde_json::json!({
                "id": d.dashboard_name,
                "dashboard_name": d.dashboard_name,
                "dashboard_body": d.dashboard_body,
                "dashboard_arn": d.dashboard_arn,
            });
            results.push(ExtractedResource {
                name: d.dashboard_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudwatch_metric_stream
// ---------------------------------------------------------------------------

pub struct AwsCloudwatchMetricStreamConverter {
    service: Arc<CloudWatchService>,
}

impl AwsCloudwatchMetricStreamConverter {
    pub fn new(service: Arc<CloudWatchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudwatchMetricStreamConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudwatch_metric_stream"
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

impl AwsCloudwatchMetricStreamConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cloudwatch_gen::MetricStreamTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudwatch_metric_stream", e))?;

        let name = model.name;
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:cloudwatch:{}:{}:metric-stream/{}",
                region, ctx.default_account_id, name
            )
        });
        let state = model.state.unwrap_or_else(|| "running".to_string());
        let now = chrono::Utc::now().timestamp();

        let view = MetricStreamView {
            name: name.clone(),
            arn,
            firehose_arn: model.firehose_arn,
            role_arn: model.role_arn,
            state,
            output_format: model.output_format,
            include_filters: vec![],
            exclude_filters: vec![],
            creation_date: now,
            last_update_date: now,
        };

        let mut state_view = CloudwatchStateView::default();
        state_view.metric_streams.insert(name, view);
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
        for ms in view.metric_streams.values() {
            let attrs = serde_json::json!({
                "id": ms.name,
                "name": ms.name,
                "arn": ms.arn,
                "firehose_arn": ms.firehose_arn,
                "role_arn": ms.role_arn,
                "state": ms.state,
                "output_format": ms.output_format,
                "creation_date": ms.creation_date,
                "last_update_date": ms.last_update_date,
            });
            results.push(ExtractedResource {
                name: ms.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
