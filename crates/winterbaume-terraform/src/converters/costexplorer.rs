//! Terraform converters for Cost Explorer resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_costexplorer::CostExplorerService;
use winterbaume_costexplorer::views::{
    AnomalyMonitorView, AnomalySubscriptionView, CostExplorerStateView, SubscriberView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_ce_anomaly_monitor
// ---------------------------------------------------------------------------

pub struct AwsCeAnomalyMonitorConverter {
    service: Arc<CostExplorerService>,
}

impl AwsCeAnomalyMonitorConverter {
    pub fn new(service: Arc<CostExplorerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCeAnomalyMonitorConverter {
    fn resource_type(&self) -> &str {
        "aws_ce_anomaly_monitor"
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

impl AwsCeAnomalyMonitorConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_ce_anomaly_monitor")?;
        let monitor_type = require_str(attrs, "monitor_type", "aws_ce_anomaly_monitor")?;
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:ce:{}:{}:anomalymonitor/{}",
                region, ctx.default_account_id, name
            )
        });
        let monitor_dimension = optional_str(attrs, "monitor_dimension");

        let _tags = extract_tags(attrs);

        let monitor_view = AnomalyMonitorView {
            monitor_arn: arn.clone(),
            monitor_name: name.to_string(),
            monitor_type: monitor_type.to_string(),
            monitor_dimension,
            creation_date: optional_str(attrs, "creation_date")
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            last_updated_date: optional_str(attrs, "last_updated_date")
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            last_evaluated_date: optional_str(attrs, "last_evaluated_date"),
        };

        let mut state_view = CostExplorerStateView::default();
        state_view.anomaly_monitors.insert(arn, monitor_view);
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
        for m in view.anomaly_monitors.values() {
            let attrs = serde_json::json!({
                "id": m.monitor_arn,
                "arn": m.monitor_arn,
                "name": m.monitor_name,
                "monitor_type": m.monitor_type,
                "monitor_dimension": m.monitor_dimension,
                "creation_date": m.creation_date,
                "last_updated_date": m.last_updated_date,
                "last_evaluated_date": m.last_evaluated_date,
            });
            results.push(ExtractedResource {
                name: m.monitor_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ce_anomaly_subscription
// ---------------------------------------------------------------------------

pub struct AwsCeAnomalySubscriptionConverter {
    service: Arc<CostExplorerService>,
}

impl AwsCeAnomalySubscriptionConverter {
    pub fn new(service: Arc<CostExplorerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCeAnomalySubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_ce_anomaly_subscription"
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

impl AwsCeAnomalySubscriptionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_ce_anomaly_subscription")?;
        let frequency = require_str(attrs, "frequency", "aws_ce_anomaly_subscription")?;
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:ce:{}:{}:anomalysubscription/{}",
                region, ctx.default_account_id, name
            )
        });

        let monitor_arn_list: Vec<String> = attrs
            .get("monitor_arn_list")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let subscribers: Vec<SubscriberView> = attrs
            .get("subscriber")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|s| SubscriberView {
                        address: s.get("address").and_then(|v| v.as_str()).map(String::from),
                        status: s.get("status").and_then(|v| v.as_str()).map(String::from),
                        subscriber_type: s.get("type").and_then(|v| v.as_str()).map(String::from),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let threshold = attrs.get("threshold").and_then(|v| v.as_f64());
        let account_id =
            optional_str(attrs, "account_id").unwrap_or_else(|| ctx.default_account_id.clone());

        let sub_view = AnomalySubscriptionView {
            subscription_arn: arn.clone(),
            subscription_name: name.to_string(),
            account_id,
            monitor_arn_list,
            subscribers,
            frequency: frequency.to_string(),
            threshold,
        };

        let mut state_view = CostExplorerStateView::default();
        state_view.anomaly_subscriptions.insert(arn, sub_view);
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
        for s in view.anomaly_subscriptions.values() {
            let subscribers_json: Vec<serde_json::Value> = s
                .subscribers
                .iter()
                .map(|sub| {
                    serde_json::json!({
                        "address": sub.address,
                        "status": sub.status,
                        "type": sub.subscriber_type,
                    })
                })
                .collect();
            let attrs = serde_json::json!({
                "id": s.subscription_arn,
                "arn": s.subscription_arn,
                "name": s.subscription_name,
                "account_id": s.account_id,
                "monitor_arn_list": s.monitor_arn_list,
                "subscriber": subscribers_json,
                "frequency": s.frequency,
                "threshold": s.threshold,
            });
            results.push(ExtractedResource {
                name: s.subscription_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
