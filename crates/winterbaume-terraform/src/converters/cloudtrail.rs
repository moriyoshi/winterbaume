//! Terraform converter for CloudTrail resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_cloudtrail::CloudTrailService;
use winterbaume_cloudtrail::views::{CloudTrailStateView, TrailView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_bool, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_cloudtrail
// ---------------------------------------------------------------------------

pub struct AwsCloudtrailConverter {
    service: Arc<CloudTrailService>,
}

impl AwsCloudtrailConverter {
    pub fn new(service: Arc<CloudTrailService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudtrailConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudtrail"
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

impl AwsCloudtrailConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_cloudtrail")?;
        let s3_bucket = require_str(attrs, "s3_bucket_name", "aws_cloudtrail")?;
        let s3_key_prefix = optional_str(attrs, "s3_key_prefix").unwrap_or_default();
        let trail_arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:cloudtrail:{}:{}:trail/{}",
                region, ctx.default_account_id, name
            )
        });
        let tags = extract_tags(attrs);
        let advanced_event_selectors = attrs.get("advanced_event_selector").cloned();

        // Parse event_selector nested blocks for round-tripping.
        let event_selectors: Vec<winterbaume_cloudtrail::views::EventSelectorView> = attrs
            .get("event_selector")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|es| winterbaume_cloudtrail::views::EventSelectorView {
                        read_write_type: es
                            .get("read_write_type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("All")
                            .to_string(),
                        include_management_events: es
                            .get("include_management_events")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true),
                        data_resources: es
                            .get("data_resource")
                            .and_then(|v| v.as_array())
                            .map(|dr_arr| {
                                dr_arr
                                    .iter()
                                    .map(|dr| winterbaume_cloudtrail::views::DataResourceView {
                                        r#type: dr
                                            .get("type")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                        values: dr
                                            .get("values")
                                            .and_then(|v| v.as_array())
                                            .map(|a| {
                                                a.iter()
                                                    .filter_map(|v| {
                                                        v.as_str().map(|s| s.to_string())
                                                    })
                                                    .collect()
                                            })
                                            .unwrap_or_default(),
                                    })
                                    .collect()
                            })
                            .unwrap_or_default(),
                        exclude_management_event_sources: es
                            .get("exclude_management_event_sources")
                            .and_then(|v| v.as_array())
                            .map(|a| {
                                a.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Parse insight_selector nested blocks.
        let insight_selectors: Vec<winterbaume_cloudtrail::views::InsightSelectorView> = attrs
            .get("insight_selector")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|is| winterbaume_cloudtrail::views::InsightSelectorView {
                        insight_type: is
                            .get("insight_type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let view = TrailView {
            name: name.to_string(),
            s3_bucket_name: s3_bucket.to_string(),
            s3_key_prefix,
            include_global_service_events: optional_bool(attrs, "include_global_service_events")
                .unwrap_or(true),
            is_multi_region_trail: optional_bool(attrs, "is_multi_region_trail").unwrap_or(false),
            trail_arn,
            home_region: region.clone(),
            is_logging: optional_bool(attrs, "enable_logging").unwrap_or(true),
            latest_delivery_time: None,
            tags,
            event_selectors,
            insight_selectors,
            advanced_event_selectors,
        };

        let mut state_view = CloudTrailStateView {
            trails: HashMap::new(),
            event_data_stores: HashMap::new(),
        };
        state_view.trails.insert(name.to_string(), view);
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
        for trail in view.trails.values() {
            let event_selector_json: Vec<serde_json::Value> = trail
                .event_selectors
                .iter()
                .map(|es| {
                    serde_json::json!({
                        "read_write_type": es.read_write_type,
                        "include_management_events": es.include_management_events,
                        "data_resource": es.data_resources.iter().map(|dr| serde_json::json!({
                            "type": dr.r#type,
                            "values": dr.values,
                        })).collect::<Vec<_>>(),
                        "exclude_management_event_sources": es.exclude_management_event_sources,
                    })
                })
                .collect();
            let insight_selector_json: Vec<serde_json::Value> = trail
                .insight_selectors
                .iter()
                .map(|is| serde_json::json!({"insight_type": is.insight_type}))
                .collect();
            let attrs = serde_json::json!({
                "id": trail.name,
                "name": trail.name,
                "arn": trail.trail_arn,
                "s3_bucket_name": trail.s3_bucket_name,
                "s3_key_prefix": trail.s3_key_prefix,
                "include_global_service_events": trail.include_global_service_events,
                "is_multi_region_trail": trail.is_multi_region_trail,
                "enable_logging": trail.is_logging,
                "home_region": trail.home_region,
                "tags": trail.tags,
                "event_selector": event_selector_json,
                "insight_selector": insight_selector_json,
                "advanced_event_selector": trail.advanced_event_selectors,
            });
            results.push(ExtractedResource {
                name: trail.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
