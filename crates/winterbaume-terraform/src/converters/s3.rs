//! Terraform converter for `aws_s3_bucket` resources.
//!
//! `BucketStateTfModel` is generated from `specs/s3.toml`. The bucket
//! name fallback chain (`bucket` -> `id`), the ARN / domain name
//! templates, the `force_destroy` discard, and the
//! `request_payment_payer` default are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_s3::S3Service;
use winterbaume_s3::views::{BucketStateView, S3StateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::s3 as s3_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags};

/// Converts `aws_s3_bucket` Terraform resources to/from S3 service state.
pub struct AwsS3BucketConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketConverter {
    /// Create a new converter backed by the given S3 service.
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3BucketConverter {
    fn resource_type(&self) -> &str {
        "aws_s3_bucket"
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

impl AwsS3BucketConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: s3_gen::BucketStateTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_s3_bucket", e))?;

        // bucket has a fallback chain (`bucket` -> `id`); both are spec-`string?`.
        let bucket_name = model
            .bucket
            .clone()
            .or_else(|| model.id.clone())
            .ok_or_else(|| ConversionError::MissingAttribute {
                resource_type: "aws_s3_bucket".to_string(),
                attribute: "bucket".to_string(),
            })?;

        // force_destroy is accepted but not stored (Terraform-only lifecycle flag).
        let _force_destroy = attrs.get("force_destroy").and_then(|v| v.as_bool());

        let bucket_view = BucketStateView {
            name: bucket_name.clone(),
            region: region.clone(),
            creation_date: model.creation_date,
            tags: extract_tags(attrs),
            acl: model.acl,
            policy: model.policy,
            versioning_status: model.versioning.or(model.versioning_status),
            accelerate_status: model.acceleration_status,
            request_payment_payer: model
                .request_payer
                .unwrap_or_else(|| "BucketOwner".to_string()),
            encryption_configuration: model.server_side_encryption_configuration,
            logging_configuration: model.logging,
            cors_configuration: model.cors_rule,
            lifecycle_configuration: model.lifecycle_rule,
            website_configuration: model.website,
            replication_configuration: model.replication_configuration,
            object_lock_configuration: model.object_lock_configuration,
            notification_configuration: model.notification_configuration,
            ownership_controls: model.ownership_controls,
            ..Default::default()
        };

        let mut state_view = S3StateView {
            buckets: HashMap::new(),
        };
        state_view.buckets.insert(bucket_name, bucket_view);

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
        for bucket in view.buckets.values() {
            let mut attrs = serde_json::json!({
                "id": bucket.name,
                "bucket": bucket.name,
                "region": bucket.region,
                "arn": format!("arn:aws:s3:::{}", bucket.name),
                "bucket_domain_name": format!("{}.s3.amazonaws.com", bucket.name),
                "bucket_regional_domain_name": format!(
                    "{}.s3.{}.amazonaws.com", bucket.name, bucket.region
                ),
                "request_payer": bucket.request_payment_payer,
            });
            let obj = attrs.as_object_mut().unwrap();
            if !bucket.tags.is_empty() {
                obj.insert("tags".to_string(), serde_json::json!(bucket.tags));
                obj.insert("tags_all".to_string(), serde_json::json!(bucket.tags));
            }
            if let Some(ref v) = bucket.acl {
                obj.insert("acl".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.policy {
                obj.insert("policy".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.versioning_status {
                obj.insert("versioning".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.accelerate_status {
                obj.insert("acceleration_status".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.encryption_configuration {
                obj.insert(
                    "server_side_encryption_configuration".to_string(),
                    serde_json::json!(v),
                );
            }
            if let Some(ref v) = bucket.logging_configuration {
                obj.insert("logging".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.cors_configuration {
                obj.insert("cors_rule".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.lifecycle_configuration {
                obj.insert("lifecycle_rule".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.website_configuration {
                obj.insert("website".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.replication_configuration {
                obj.insert(
                    "replication_configuration".to_string(),
                    serde_json::json!(v),
                );
            }
            if let Some(ref v) = bucket.object_lock_configuration {
                obj.insert(
                    "object_lock_configuration".to_string(),
                    serde_json::json!(v),
                );
            }
            if let Some(ref v) = bucket.ownership_controls {
                obj.insert("ownership_controls".to_string(), serde_json::json!(v));
            }
            let attrs = attrs;
            results.push(ExtractedResource {
                name: bucket.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
