//! Terraform converter for S3 Control resources.
//!
//! `AccessPointTfModel` and `BucketTfModel` are generated from
//! `specs/s3control.toml`. ARN templates, the constants
//! (`network_origin = "Internet"`, default `public_access_block_enabled
//! = true`), and the `tags` HashMap-to-`Vec<(String, String)>`
//! conversion are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_s3control::S3ControlService;
use winterbaume_s3control::views::{AccessPointView, OutpostsBucketView, S3ControlStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::s3control as s3control_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_s3control_access_point
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_access_point` Terraform resources to/from S3 Control state.
pub struct AwsS3controlAccessPointConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlAccessPointConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlAccessPointConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_access_point"
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

impl AwsS3controlAccessPointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::AccessPointTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_s3control_access_point", e))?;

        let account_id = model
            .account_id
            .clone()
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let alias = model.alias.clone().unwrap_or_default();
        let arn = model.arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:s3:{}:{}:accesspoint/{}",
                region, account_id, model.name
            )
        });
        let network_origin = model
            .network_origin
            .clone()
            .unwrap_or_else(|| "Internet".to_string());
        let creation_date = model.creation_date.clone().unwrap_or_default();

        let ap_view = AccessPointView {
            name: model.name.clone(),
            bucket: model.bucket,
            account_id: account_id.clone(),
            region: region.clone(),
            alias,
            arn,
            network_origin,
            vpc_id: model.vpc_id,
            block_public_acls: model.block_public_acls,
            ignore_public_acls: model.ignore_public_acls,
            block_public_policy: model.block_public_policy,
            restrict_public_buckets: model.restrict_public_buckets,
            creation_date,
            policy: model.policy,
        };

        let mut state_view = S3ControlStateView::default();
        state_view.access_points.insert(model.name, ap_view);
        self.service.merge(&account_id, &region, state_view).await?;

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
        for ap in view.access_points.values() {
            let attrs = serde_json::json!({
                "id": ap.name,
                "name": ap.name,
                "bucket": ap.bucket,
                "account_id": ap.account_id,
                "region": ap.region,
                "alias": ap.alias,
                "arn": ap.arn,
                "network_origin": ap.network_origin,
                "vpc_id": ap.vpc_id,
                "block_public_acls": ap.block_public_acls,
                "ignore_public_acls": ap.ignore_public_acls,
                "block_public_policy": ap.block_public_policy,
                "restrict_public_buckets": ap.restrict_public_buckets,
                "creation_date": ap.creation_date,
                "policy": ap.policy,
            });
            results.push(ExtractedResource {
                name: ap.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_bucket
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_bucket` Terraform resources to/from S3 Control state.
pub struct AwsS3controlBucketConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlBucketConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlBucketConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_bucket"
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

impl AwsS3controlBucketConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::BucketTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_s3control_bucket", e))?;

        let arn = model.arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:s3-outposts:{}:{}:outpost/{}/bucket/{}",
                region, ctx.default_account_id, model.outpost_id, model.bucket
            )
        });
        let creation_date = model.creation_date.clone().unwrap_or_default();
        // public_access_block_enabled is not a typed model field because the
        // TF default is `true` (`bool` types in the spec default to `false`).
        let public_access_block_enabled = instance
            .attributes
            .get("public_access_block_enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let tags: Vec<(String, String)> = model.tags.into_iter().collect();

        let bucket_view = OutpostsBucketView {
            name: model.bucket.clone(),
            arn,
            outpost_id: model.outpost_id,
            creation_date,
            public_access_block_enabled,
            policy: model.policy,
            tags,
        };

        let mut state_view = S3ControlStateView::default();
        state_view
            .outposts_buckets
            .insert(model.bucket, bucket_view);
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
        for bucket in view.outposts_buckets.values() {
            let tags_map: HashMap<String, String> = bucket.tags.iter().cloned().collect();
            let attrs = serde_json::json!({
                "id": bucket.arn,
                "bucket": bucket.name,
                "arn": bucket.arn,
                "outpost_id": bucket.outpost_id,
                "creation_date": bucket.creation_date,
                "public_access_block_enabled": bucket.public_access_block_enabled,
                "policy": bucket.policy,
                "tags": tags_map,
            });
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
