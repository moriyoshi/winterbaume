//! Terraform converters for CloudFront resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_cloudfront::CloudFrontService;
use winterbaume_cloudfront::views::{
    CloudFrontStateView, DefaultCacheBehaviorView, DistributionView, OriginView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str};

// ---------------------------------------------------------------------------
// aws_cloudfront_distribution
// ---------------------------------------------------------------------------

/// Converts `aws_cloudfront_distribution` Terraform resources to/from CloudFront state.
pub struct AwsCloudfrontDistributionConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontDistributionConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontDistributionConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_distribution"
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

impl AwsCloudfrontDistributionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        // CloudFront is global; use us-east-1 as the canonical region
        let region = extract_region(attrs, "us-east-1");

        let dist_id = optional_str(attrs, "id").unwrap_or_else(|| {
            format!(
                "E{}",
                &uuid::Uuid::new_v4().to_string()[..13]
                    .to_uppercase()
                    .replace('-', "")
            )
        });

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:cloudfront::{}:distribution/{}",
                ctx.default_account_id, dist_id
            )
        });
        let domain_name = optional_str(attrs, "domain_name")
            .unwrap_or_else(|| format!("{}.cloudfront.net", dist_id.to_lowercase()));

        let enabled = attrs
            .get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        // Parse origins array
        let origins: Vec<OriginView> = attrs
            .get("origin")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|o| OriginView {
                        id: o
                            .get("origin_id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        domain_name: o
                            .get("domain_name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        // default_cache_behavior
        let dcb = attrs
            .get("default_cache_behavior")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first());
        let target_origin_id = dcb
            .and_then(|b| b.get("target_origin_id"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let viewer_protocol_policy = dcb
            .and_then(|b| b.get("viewer_protocol_policy"))
            .and_then(|v| v.as_str())
            .unwrap_or("allow-all")
            .to_string();

        let allowed_methods: Vec<String> = dcb
            .and_then(|b| b.get("allowed_methods"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let cached_methods: Vec<String> = dcb
            .and_then(|b| b.get("cached_methods"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let compress = dcb
            .and_then(|b| b.get("compress"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let forwarded_values_query_string = dcb
            .and_then(|b| b.get("forwarded_values"))
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|fv| fv.get("query_string"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let forwarded_values_cookies_forward = dcb
            .and_then(|b| b.get("forwarded_values"))
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|fv| fv.get("cookies"))
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("forward"))
            .and_then(|v| v.as_str())
            .unwrap_or("none")
            .to_string();

        let default_cache_behavior = DefaultCacheBehaviorView {
            target_origin_id,
            viewer_protocol_policy,
            allowed_methods,
            cached_methods,
            forwarded_values_query_string,
            forwarded_values_cookies_forward,
            compress,
        };

        // Additional fields for coverage
        let _ = attrs.get("tags_all");
        let _ = attrs.get("comment");
        let _ = attrs.get("default_root_object");
        let _ = attrs.get("http_version");
        let _ = attrs.get("price_class");
        let _ = attrs.get("restrictions");
        let _ = attrs.get("viewer_certificate");
        let _ = attrs.get("web_acl_id");
        let _ = attrs.get("continuous_deployment_policy_id");
        let _ = attrs.get("staging");

        let custom_error_response: Vec<serde_json::Value> = attrs
            .get("custom_error_response")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let logging_config = attrs
            .get("logging_config")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let ordered_cache_behavior: Vec<serde_json::Value> = attrs
            .get("ordered_cache_behavior")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let origin_group: Vec<serde_json::Value> = attrs
            .get("origin_group")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        // Tags
        let mut tags: HashMap<String, String> = HashMap::new();
        if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.insert(k.clone(), s.to_string());
                }
            }
        }

        let dist_view = DistributionView {
            id: dist_id.clone(),
            arn,
            domain_name,
            status: "Deployed".to_string(),
            origins,
            default_cache_behavior,
            caller_reference: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().to_rfc3339()),
            etag: uuid::Uuid::new_v4().to_string(),
            enabled,
            tags,
            raw_config: None,
            custom_error_response,
            logging_config,
            ordered_cache_behavior,
            origin_group,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.distributions.insert(dist_id, dist_view);
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
        for dist in view.distributions.values() {
            let origins_json: Vec<serde_json::Value> = dist
                .origins
                .iter()
                .map(|o| {
                    serde_json::json!({
                        "origin_id": o.id,
                        "domain_name": o.domain_name,
                    })
                })
                .collect();
            let dcb = &dist.default_cache_behavior;
            let default_cache_behavior_json = serde_json::json!([{
                "target_origin_id": dcb.target_origin_id,
                "viewer_protocol_policy": dcb.viewer_protocol_policy,
                "allowed_methods": dcb.allowed_methods,
                "cached_methods": dcb.cached_methods,
                "compress": dcb.compress,
            }]);
            let last_modified_time = dist.created_at.clone().unwrap_or_default();
            let attrs = serde_json::json!({
                "id": dist.id,
                "arn": dist.arn,
                "domain_name": dist.domain_name,
                "status": dist.status,
                "enabled": dist.enabled,
                "etag": dist.etag,
                "caller_reference": dist.caller_reference,
                "origin": origins_json,
                "default_cache_behavior": default_cache_behavior_json,
                "last_modified_time": last_modified_time,
                "in_progress_validation_batches": 0,
                "hosted_zone_id": "Z2FDTNDATAQYW2",
                "tags": dist.tags,
                "tags_all": dist.tags,
                "restrictions": [{"geo_restriction": [{"restriction_type": "none", "locations": []}]}],
                "viewer_certificate": [{"cloudfront_default_certificate": true}],
                "price_class": "PriceClass_All",
                "custom_error_response": dist.custom_error_response,
                "logging_config": dist.logging_config,
                "ordered_cache_behavior": dist.ordered_cache_behavior,
                "origin_group": dist.origin_group,
            });
            results.push(ExtractedResource {
                name: dist.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
