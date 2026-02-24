//! Terraform converter for S3 Control resources.

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
use crate::util::{extract_region, optional_bool, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_s3control_access_point")?;
        let bucket = require_str(attrs, "bucket", "aws_s3control_access_point")?;
        let region = extract_region(attrs, &ctx.default_region);
        let account_id =
            optional_str(attrs, "account_id").unwrap_or_else(|| ctx.default_account_id.clone());
        let alias = optional_str(attrs, "alias").unwrap_or_default();
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!("arn:aws:s3:{}:{}:accesspoint/{}", region, account_id, name)
        });
        let network_origin =
            optional_str(attrs, "network_origin").unwrap_or_else(|| "Internet".to_string());
        let vpc_id = optional_str(attrs, "vpc_id");
        let block_public_acls = optional_bool(attrs, "block_public_acls").unwrap_or(false);
        let ignore_public_acls = optional_bool(attrs, "ignore_public_acls").unwrap_or(false);
        let block_public_policy = optional_bool(attrs, "block_public_policy").unwrap_or(false);
        let restrict_public_buckets =
            optional_bool(attrs, "restrict_public_buckets").unwrap_or(false);
        let creation_date = optional_str(attrs, "creation_date").unwrap_or_default();
        let policy = optional_str(attrs, "policy");

        let ap_view = AccessPointView {
            name: name.to_string(),
            bucket: bucket.to_string(),
            account_id: account_id.clone(),
            region: region.clone(),
            alias,
            arn,
            network_origin,
            vpc_id,
            block_public_acls,
            ignore_public_acls,
            block_public_policy,
            restrict_public_buckets,
            creation_date,
            policy,
        };

        let mut state_view = S3ControlStateView::default();
        state_view.access_points.insert(name.to_string(), ap_view);
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
        let attrs = &instance.attributes;
        let name = require_str(attrs, "bucket", "aws_s3control_bucket")?;
        let outpost_id = require_str(attrs, "outpost_id", "aws_s3control_bucket")?;
        let region = extract_region(attrs, &ctx.default_region);
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:s3-outposts:{}:{}:outpost/{}/bucket/{}",
                region, ctx.default_account_id, outpost_id, name
            )
        });
        let creation_date = optional_str(attrs, "creation_date").unwrap_or_default();
        let public_access_block_enabled =
            optional_bool(attrs, "public_access_block_enabled").unwrap_or(true);
        let policy = optional_str(attrs, "policy");

        let tags = if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
            obj.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect()
        } else {
            vec![]
        };

        let bucket_view = OutpostsBucketView {
            name: name.to_string(),
            arn,
            outpost_id: outpost_id.to_string(),
            creation_date,
            public_access_block_enabled,
            policy,
            tags,
        };

        let mut state_view = S3ControlStateView::default();
        state_view
            .outposts_buckets
            .insert(name.to_string(), bucket_view);
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
