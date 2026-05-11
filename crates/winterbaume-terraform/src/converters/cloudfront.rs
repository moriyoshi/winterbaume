//! Terraform converters for CloudFront resources.
//!
//! `DistributionTfModel` is generated from `specs/cloudfront.toml`. The
//! synthesised distribution id (UUID-derived `E...` string), the ARN
//! template, the cloudfront.net domain template, the `origin` /
//! `default_cache_behavior` nested-block parsing, the
//! `custom_error_response` / `logging_config` / `ordered_cache_behavior`
//! / `origin_group` raw-JSON capture, the `caller_reference` /
//! `created_at` / `etag` / `status` constants, and the `us-east-1`
//! canonical region are wired up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_cloudfront::CloudFrontService;
use winterbaume_cloudfront::model::{
    CachePolicyConfig, ContinuousDeploymentPolicyConfig, FieldLevelEncryptionConfig,
    FieldLevelEncryptionProfileConfig, FunctionConfig, OriginRequestPolicyConfig,
    ResponseHeadersPolicyConfig, VpcOriginEndpointConfig,
};
use winterbaume_cloudfront::views::{
    CachePolicyView, CloudFrontFunctionView, CloudFrontStateView, ContinuousDeploymentPolicyView,
    DefaultCacheBehaviorView, DistributionView, FieldLevelEncryptionProfileView,
    FieldLevelEncryptionView, KeyGroupView, KeyValueStoreView, MonitoringSubscriptionView, OaiView,
    OriginAccessControlView, OriginRequestPolicyView, OriginView, PublicKeyView,
    RealtimeLogConfigView, ResponseHeadersPolicyView, VpcOriginView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::cloudfront as cloudfront_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        // CloudFront is global; use us-east-1 as the canonical region.
        let region = extract_region(&instance.attributes, "us-east-1");
        let model: cloudfront_gen::DistributionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudfront_distribution", e))?;

        let attrs = &instance.attributes;

        let dist_id = model.id.unwrap_or_else(|| {
            format!(
                "E{}",
                &uuid::Uuid::new_v4().to_string()[..13]
                    .to_uppercase()
                    .replace('-', "")
            )
        });

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:cloudfront::{}:distribution/{}",
                ctx.default_account_id, dist_id
            )
        });
        let domain_name = model
            .domain_name
            .unwrap_or_else(|| format!("{}.cloudfront.net", dist_id.to_lowercase()));

        // Parse origins array (nested block).
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

        // default_cache_behavior nested block.
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
            enabled: model.enabled,
            tags: model.tags,
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

/// Generate a synthetic CloudFront resource ID with the given prefix letter,
/// mirroring AWS-style `Exxxxxxxxxxxxx` / `Kxxxxxxxxxxxxx` identifiers.
fn synthetic_cf_id(prefix: char) -> String {
    format!(
        "{}{}",
        prefix,
        &uuid::Uuid::new_v4().to_string()[..13]
            .to_uppercase()
            .replace('-', "")
    )
}

// ---------------------------------------------------------------------------
// aws_cloudfront_cache_policy
// ---------------------------------------------------------------------------

pub struct AwsCloudfrontCachePolicyConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontCachePolicyConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontCachePolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_cache_policy"
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

impl AwsCloudfrontCachePolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let model: cloudfront_gen::CachePolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudfront_cache_policy", e))?;

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let etag = model
            .etag
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let config = CachePolicyConfig {
            comment: model.comment,
            default_t_t_l: Some(model.default_ttl),
            max_t_t_l: Some(model.max_ttl),
            min_t_t_l: model.min_ttl,
            name: model.name,
            parameters_in_cache_key_and_forwarded_to_origin: None,
        };

        let view = CachePolicyView {
            id: id.clone(),
            last_modified_time: Some(Utc::now().to_rfc3339()),
            config,
            etag,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.cache_policies.insert(id, view);
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
        for cp in view.cache_policies.values() {
            let attrs = serde_json::json!({
                "id": cp.id,
                "name": cp.config.name,
                "comment": cp.config.comment,
                "default_ttl": cp.config.default_t_t_l.unwrap_or(86400),
                "max_ttl": cp.config.max_t_t_l.unwrap_or(31536000),
                "min_ttl": cp.config.min_t_t_l,
                "etag": cp.etag,
            });
            results.push(ExtractedResource {
                name: cp.config.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudfront_continuous_deployment_policy
// ---------------------------------------------------------------------------

pub struct AwsCloudfrontContinuousDeploymentPolicyConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontContinuousDeploymentPolicyConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontContinuousDeploymentPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_continuous_deployment_policy"
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

impl AwsCloudfrontContinuousDeploymentPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let model: cloudfront_gen::ContinuousDeploymentPolicyTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_cloudfront_continuous_deployment_policy", e)
            })?;

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let etag = model
            .etag
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let config = ContinuousDeploymentPolicyConfig {
            enabled: model.enabled,
            ..Default::default()
        };

        let view = ContinuousDeploymentPolicyView {
            id: id.clone(),
            last_modified_time: Some(Utc::now().to_rfc3339()),
            config,
            etag,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.continuous_deployment_policies.insert(id, view);
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
        for cdp in view.continuous_deployment_policies.values() {
            let attrs = serde_json::json!({
                "id": cdp.id,
                "enabled": cdp.config.enabled,
                "etag": cdp.etag,
            });
            results.push(ExtractedResource {
                name: cdp.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudfront_field_level_encryption_config
// ---------------------------------------------------------------------------

pub struct AwsCloudfrontFieldLevelEncryptionConfigConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontFieldLevelEncryptionConfigConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontFieldLevelEncryptionConfigConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_field_level_encryption_config"
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

impl AwsCloudfrontFieldLevelEncryptionConfigConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let model: cloudfront_gen::FieldLevelEncryptionConfigTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_cloudfront_field_level_encryption_config", e)
            })?;

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let etag = model
            .etag
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let config = FieldLevelEncryptionConfig {
            caller_reference: uuid::Uuid::new_v4().to_string(),
            comment: model.comment,
            ..Default::default()
        };

        let view = FieldLevelEncryptionView {
            id: id.clone(),
            last_modified_time: Some(Utc::now().to_rfc3339()),
            config,
            etag,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.field_level_encryptions.insert(id, view);
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
        for fle in view.field_level_encryptions.values() {
            let attrs = serde_json::json!({
                "id": fle.id,
                "comment": fle.config.comment,
                "etag": fle.etag,
            });
            results.push(ExtractedResource {
                name: fle.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudfront_field_level_encryption_profile
// ---------------------------------------------------------------------------

pub struct AwsCloudfrontFieldLevelEncryptionProfileConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontFieldLevelEncryptionProfileConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontFieldLevelEncryptionProfileConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_field_level_encryption_profile"
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

impl AwsCloudfrontFieldLevelEncryptionProfileConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let model: cloudfront_gen::FieldLevelEncryptionProfileTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_cloudfront_field_level_encryption_profile", e)
            })?;

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let etag = model
            .etag
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let config = FieldLevelEncryptionProfileConfig {
            caller_reference: uuid::Uuid::new_v4().to_string(),
            name: model.name,
            comment: model.comment,
            ..Default::default()
        };

        let view = FieldLevelEncryptionProfileView {
            id: id.clone(),
            last_modified_time: Some(Utc::now().to_rfc3339()),
            config,
            etag,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.field_level_encryption_profiles.insert(id, view);
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
        for fle in view.field_level_encryption_profiles.values() {
            let attrs = serde_json::json!({
                "id": fle.id,
                "name": fle.config.name,
                "comment": fle.config.comment,
                "etag": fle.etag,
            });
            results.push(ExtractedResource {
                name: fle.config.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudfront_function
// ---------------------------------------------------------------------------

pub struct AwsCloudfrontFunctionConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontFunctionConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontFunctionConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_function"
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

impl AwsCloudfrontFunctionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let model: cloudfront_gen::FunctionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudfront_function", e))?;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:cloudfront::{}:function/{}",
                ctx.default_account_id, model.name
            )
        });
        let status = model.status.unwrap_or_else(|| {
            if model.publish {
                "LIVE".to_string()
            } else {
                "UNPUBLISHED".to_string()
            }
        });
        let etag = model
            .etag
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let config = FunctionConfig {
            comment: model.comment.unwrap_or_default(),
            key_value_store_associations: None,
            runtime: model.runtime,
        };

        let view = CloudFrontFunctionView {
            name: model.name.clone(),
            arn,
            status,
            created_time: Some(Utc::now().to_rfc3339()),
            last_modified_time: Some(Utc::now().to_rfc3339()),
            config,
            code: model.code.into_bytes(),
            etag,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.functions.insert(model.name, view);
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
        for f in view.functions.values() {
            let code = String::from_utf8(f.code.clone()).unwrap_or_default();
            let attrs = serde_json::json!({
                "id": f.name,
                "name": f.name,
                "arn": f.arn,
                "status": f.status,
                "runtime": f.config.runtime,
                "comment": f.config.comment,
                "code": code,
                "etag": f.etag,
            });
            results.push(ExtractedResource {
                name: f.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudfront_key_group
// ---------------------------------------------------------------------------

pub struct AwsCloudfrontKeyGroupConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontKeyGroupConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontKeyGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_key_group"
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

impl AwsCloudfrontKeyGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let attrs = &instance.attributes;
        let model: cloudfront_gen::KeyGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudfront_key_group", e))?;

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let etag = model
            .etag
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        // items nested array: list of public-key IDs.
        let items: Vec<String> = attrs
            .get("items")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let view = KeyGroupView {
            id: id.clone(),
            name: model.name,
            comment: model.comment,
            items,
            last_modified_time: Some(Utc::now().to_rfc3339()),
            etag,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.key_groups.insert(id, view);
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
        for kg in view.key_groups.values() {
            let attrs = serde_json::json!({
                "id": kg.id,
                "name": kg.name,
                "comment": kg.comment,
                "items": kg.items,
                "etag": kg.etag,
            });
            results.push(ExtractedResource {
                name: kg.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudfront_key_value_store
// ---------------------------------------------------------------------------

pub struct AwsCloudfrontKeyValueStoreConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontKeyValueStoreConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontKeyValueStoreConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_key_value_store"
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

impl AwsCloudfrontKeyValueStoreConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let model: cloudfront_gen::KeyValueStoreTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudfront_key_value_store", e))?;

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:cloudfront::{}:key-value-store/{}",
                ctx.default_account_id, id
            )
        });
        let etag = model
            .etag
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let view = KeyValueStoreView {
            name: model.name.clone(),
            arn,
            id,
            comment: model.comment,
            last_modified_time: Some(Utc::now().to_rfc3339()),
            status: "READY".to_string(),
            etag,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.key_value_stores.insert(model.name, view);
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
        for kvs in view.key_value_stores.values() {
            let attrs = serde_json::json!({
                "id": kvs.id,
                "name": kvs.name,
                "arn": kvs.arn,
                "comment": kvs.comment,
                "status": kvs.status,
                "etag": kvs.etag,
            });
            results.push(ExtractedResource {
                name: kvs.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudfront_monitoring_subscription
// ---------------------------------------------------------------------------
//
// Per-distribution sidecar resource: keyed by `distribution_id` on the
// CloudFrontStateView. Uses snapshot+mutate to avoid clobbering other
// monitoring subscriptions in the same scope.

pub struct AwsCloudfrontMonitoringSubscriptionConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontMonitoringSubscriptionConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontMonitoringSubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_monitoring_subscription"
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

impl AwsCloudfrontMonitoringSubscriptionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let attrs = &instance.attributes;
        let model: cloudfront_gen::MonitoringSubscriptionTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_cloudfront_monitoring_subscription", e)
            })?;

        // Pull the nested status out of monitoring_subscription { realtime_metrics_subscription_config { realtime_metrics_subscription_status } }.
        let status = attrs
            .get("monitoring_subscription")
            .and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .and_then(|ms| ms.get("realtime_metrics_subscription_config"))
            .and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .and_then(|rmsc| rmsc.get("realtime_metrics_subscription_status"))
            .and_then(|v| v.as_str())
            .unwrap_or("Enabled")
            .to_string();

        let view = MonitoringSubscriptionView {
            distribution_id: model.distribution_id.clone(),
            realtime_metrics_subscription_status: status,
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot
            .monitoring_subscriptions
            .insert(model.distribution_id, view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
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
        for ms in view.monitoring_subscriptions.values() {
            let attrs = serde_json::json!({
                "id": ms.distribution_id,
                "distribution_id": ms.distribution_id,
                "monitoring_subscription": [{
                    "realtime_metrics_subscription_config": [{
                        "realtime_metrics_subscription_status": ms.realtime_metrics_subscription_status,
                    }]
                }],
            });
            results.push(ExtractedResource {
                name: ms.distribution_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudfront_origin_access_control
// ---------------------------------------------------------------------------

pub struct AwsCloudfrontOriginAccessControlConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontOriginAccessControlConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontOriginAccessControlConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_origin_access_control"
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

impl AwsCloudfrontOriginAccessControlConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let model: cloudfront_gen::OriginAccessControlTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_cloudfront_origin_access_control", e)
            })?;

        let id = model.id.unwrap_or_else(|| synthetic_cf_id('E'));
        let etag = model
            .etag
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let view = OriginAccessControlView {
            id: id.clone(),
            name: model.name,
            description: model.description,
            origin_access_control_origin_type: model.origin_access_control_origin_type,
            signing_behavior: model.signing_behavior,
            signing_protocol: model.signing_protocol,
            etag,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.origin_access_controls.insert(id, view);
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
        for oac in view.origin_access_controls.values() {
            let attrs = serde_json::json!({
                "id": oac.id,
                "name": oac.name,
                "description": oac.description,
                "origin_access_control_origin_type": oac.origin_access_control_origin_type,
                "signing_behavior": oac.signing_behavior,
                "signing_protocol": oac.signing_protocol,
                "etag": oac.etag,
            });
            results.push(ExtractedResource {
                name: oac.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudfront_origin_access_identity
// ---------------------------------------------------------------------------

pub struct AwsCloudfrontOriginAccessIdentityConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontOriginAccessIdentityConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontOriginAccessIdentityConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_origin_access_identity"
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

impl AwsCloudfrontOriginAccessIdentityConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let model: cloudfront_gen::OriginAccessIdentityTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_cloudfront_origin_access_identity", e)
            })?;

        let id = model.id.unwrap_or_else(|| synthetic_cf_id('E'));
        let etag = model
            .etag
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let caller_reference = model
            .caller_reference
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let s3_canonical_user_id = model.s3_canonical_user_id.unwrap_or_else(|| {
            // Synthetic 64-hex stand-in.
            uuid::Uuid::new_v4().to_string().replace('-', "").repeat(2)
        });

        let view = OaiView {
            id: id.clone(),
            caller_reference,
            comment: model.comment.unwrap_or_default(),
            s3_canonical_user_id,
            etag,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.oais.insert(id, view);
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
        for oai in view.oais.values() {
            let iam_arn = format!(
                "arn:aws:iam::cloudfront:user/CloudFront Origin Access Identity {}",
                oai.id
            );
            let cloudfront_access_identity_path =
                format!("origin-access-identity/cloudfront/{}", oai.id);
            let attrs = serde_json::json!({
                "id": oai.id,
                "comment": oai.comment,
                "caller_reference": oai.caller_reference,
                "s3_canonical_user_id": oai.s3_canonical_user_id,
                "etag": oai.etag,
                "iam_arn": iam_arn,
                "cloudfront_access_identity_path": cloudfront_access_identity_path,
            });
            results.push(ExtractedResource {
                name: oai.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudfront_origin_request_policy
// ---------------------------------------------------------------------------

pub struct AwsCloudfrontOriginRequestPolicyConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontOriginRequestPolicyConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontOriginRequestPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_origin_request_policy"
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

impl AwsCloudfrontOriginRequestPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let model: cloudfront_gen::OriginRequestPolicyTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_cloudfront_origin_request_policy", e)
            })?;

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let etag = model
            .etag
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let config = OriginRequestPolicyConfig {
            name: model.name,
            comment: model.comment,
            ..Default::default()
        };

        let view = OriginRequestPolicyView {
            id: id.clone(),
            last_modified_time: Some(Utc::now().to_rfc3339()),
            config,
            etag,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.origin_request_policies.insert(id, view);
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
        for orp in view.origin_request_policies.values() {
            let attrs = serde_json::json!({
                "id": orp.id,
                "name": orp.config.name,
                "comment": orp.config.comment,
                "etag": orp.etag,
            });
            results.push(ExtractedResource {
                name: orp.config.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudfront_public_key
// ---------------------------------------------------------------------------

pub struct AwsCloudfrontPublicKeyConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontPublicKeyConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontPublicKeyConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_public_key"
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

impl AwsCloudfrontPublicKeyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let model: cloudfront_gen::PublicKeyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudfront_public_key", e))?;

        let id = model.id.unwrap_or_else(|| synthetic_cf_id('K'));
        let etag = model
            .etag
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let caller_reference = model
            .caller_reference
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let view = PublicKeyView {
            id: id.clone(),
            name: model.name,
            caller_reference,
            encoded_key: model.encoded_key,
            comment: model.comment,
            created_time: Some(Utc::now().to_rfc3339()),
            etag,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.public_keys.insert(id, view);
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
        for pk in view.public_keys.values() {
            let attrs = serde_json::json!({
                "id": pk.id,
                "name": pk.name,
                "caller_reference": pk.caller_reference,
                "encoded_key": pk.encoded_key,
                "comment": pk.comment,
                "etag": pk.etag,
            });
            results.push(ExtractedResource {
                name: pk.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudfront_realtime_log_config
// ---------------------------------------------------------------------------

pub struct AwsCloudfrontRealtimeLogConfigConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontRealtimeLogConfigConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontRealtimeLogConfigConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_realtime_log_config"
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

impl AwsCloudfrontRealtimeLogConfigConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let attrs = &instance.attributes;
        let model: cloudfront_gen::RealtimeLogConfigTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudfront_realtime_log_config", e))?;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:cloudfront::{}:realtime-log-config/{}",
                ctx.default_account_id, model.name
            )
        });

        // fields is a top-level set/list in the TF schema.
        let fields: Vec<String> = attrs
            .get("fields")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let view = RealtimeLogConfigView {
            name: model.name,
            arn: arn.clone(),
            sampling_rate: model.sampling_rate,
            end_points: vec![],
            fields,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.realtime_log_configs.insert(arn, view);
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
        for r in view.realtime_log_configs.values() {
            let attrs = serde_json::json!({
                "id": r.arn,
                "arn": r.arn,
                "name": r.name,
                "sampling_rate": r.sampling_rate,
                "fields": r.fields,
            });
            results.push(ExtractedResource {
                name: r.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudfront_response_headers_policy
// ---------------------------------------------------------------------------

pub struct AwsCloudfrontResponseHeadersPolicyConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontResponseHeadersPolicyConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontResponseHeadersPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_response_headers_policy"
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

impl AwsCloudfrontResponseHeadersPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let model: cloudfront_gen::ResponseHeadersPolicyTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_cloudfront_response_headers_policy", e)
            })?;

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let etag = model
            .etag
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let config = ResponseHeadersPolicyConfig {
            name: model.name,
            comment: model.comment,
            ..Default::default()
        };

        let view = ResponseHeadersPolicyView {
            id: id.clone(),
            last_modified_time: Some(Utc::now().to_rfc3339()),
            config,
            etag,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.response_headers_policies.insert(id, view);
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
        for rhp in view.response_headers_policies.values() {
            let attrs = serde_json::json!({
                "id": rhp.id,
                "name": rhp.config.name,
                "comment": rhp.config.comment,
                "etag": rhp.etag,
            });
            results.push(ExtractedResource {
                name: rhp.config.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudfront_vpc_origin
// ---------------------------------------------------------------------------

pub struct AwsCloudfrontVpcOriginConverter {
    service: Arc<CloudFrontService>,
}

impl AwsCloudfrontVpcOriginConverter {
    pub fn new(service: Arc<CloudFrontService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudfrontVpcOriginConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudfront_vpc_origin"
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

impl AwsCloudfrontVpcOriginConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, "us-east-1");
        let attrs = &instance.attributes;
        let model: cloudfront_gen::VpcOriginTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudfront_vpc_origin", e))?;
        let _ = model.tags;

        let id = model.id.unwrap_or_else(|| synthetic_cf_id('V'));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:cloudfront::{}:vpcorigin/{}",
                ctx.default_account_id, id
            )
        });
        let etag = model
            .etag
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        // Pull a thin VpcOriginEndpointConfig out of vpc_origin_endpoint_config[0].
        let mut config = VpcOriginEndpointConfig::default();
        if let Some(cfg) = attrs
            .get("vpc_origin_endpoint_config")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
        {
            if let Some(s) = cfg.get("arn").and_then(|v| v.as_str()) {
                config.arn = s.to_string();
            }
            if let Some(n) = cfg.get("http_port").and_then(|v| v.as_i64()) {
                config.h_t_t_p_port = n as i32;
            }
            if let Some(n) = cfg.get("https_port").and_then(|v| v.as_i64()) {
                config.h_t_t_p_s_port = n as i32;
            }
            if let Some(s) = cfg.get("name").and_then(|v| v.as_str()) {
                config.name = s.to_string();
            }
            if let Some(s) = cfg.get("origin_protocol_policy").and_then(|v| v.as_str()) {
                config.origin_protocol_policy = s.to_string();
            }
        }

        let now = Utc::now().to_rfc3339();
        let view = VpcOriginView {
            id: id.clone(),
            arn,
            account_id: ctx.default_account_id.clone(),
            created_time: Some(now.clone()),
            last_modified_time: Some(now),
            status: "Deployed".to_string(),
            config,
            etag,
        };

        let mut state_view = CloudFrontStateView::default();
        state_view.vpc_origins.insert(id, view);
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
        for vo in view.vpc_origins.values() {
            let cfg = serde_json::json!([{
                "arn": vo.config.arn,
                "http_port": vo.config.h_t_t_p_port,
                "https_port": vo.config.h_t_t_p_s_port,
                "name": vo.config.name,
                "origin_protocol_policy": vo.config.origin_protocol_policy,
            }]);
            let attrs = serde_json::json!({
                "id": vo.id,
                "arn": vo.arn,
                "etag": vo.etag,
                "vpc_origin_endpoint_config": cfg,
            });
            results.push(ExtractedResource {
                name: vo.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
