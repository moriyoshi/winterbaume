//! Terraform converters for CloudWatch resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_cloudwatch::CloudWatchService;
use winterbaume_cloudwatch::views::{CloudwatchStateView, DimensionView, MetricAlarmView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let alarm_name = require_str(attrs, "alarm_name", "aws_cloudwatch_metric_alarm")?;
        let metric_name = require_str(attrs, "metric_name", "aws_cloudwatch_metric_alarm")?;
        let namespace = require_str(attrs, "namespace", "aws_cloudwatch_metric_alarm")?;
        let comparison_operator =
            require_str(attrs, "comparison_operator", "aws_cloudwatch_metric_alarm")?;

        let threshold = attrs
            .get("threshold")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let evaluation_periods = attrs
            .get("evaluation_periods")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as u32;
        let period = attrs.get("period").and_then(|v| v.as_u64()).unwrap_or(60) as u32;
        let statistic = optional_str(attrs, "statistic").unwrap_or_else(|| "Average".to_string());

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:cloudwatch:{}:{}:alarm:{}",
                region, ctx.default_account_id, alarm_name
            )
        });

        let actions_enabled = attrs
            .get("actions_enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

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
            alarm_name: alarm_name.to_string(),
            alarm_arn: arn,
            metric_name: metric_name.to_string(),
            namespace: namespace.to_string(),
            threshold,
            comparison_operator: comparison_operator.to_string(),
            evaluation_periods,
            period,
            statistic,
            state_value: "OK".to_string(),
            state_reason: String::new(),
            actions_enabled,
            alarm_description: optional_str(attrs, "alarm_description"),
            alarm_actions,
            ok_actions,
            insufficient_data_actions,
            dimensions,
            unit: optional_str(attrs, "unit"),
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
        state_view.alarms.insert(alarm_name.to_string(), alarm_view);
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
