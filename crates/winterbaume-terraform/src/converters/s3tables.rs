//! Terraform converters for S3 Tables resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_s3tables::S3TablesService;
use winterbaume_s3tables::views::{NamespaceView, S3TablesStateView, TableBucketView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_s3tables_table_bucket
// ---------------------------------------------------------------------------

pub struct AwsS3tablesTableBucketConverter {
    service: Arc<S3TablesService>,
}

impl AwsS3tablesTableBucketConverter {
    pub fn new(service: Arc<S3TablesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3tablesTableBucketConverter {
    fn resource_type(&self) -> &str {
        "aws_s3tables_table_bucket"
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

impl AwsS3tablesTableBucketConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_s3tables_table_bucket")?;
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:s3tables:{}:{}:bucket/{}",
                region, ctx.default_account_id, name
            )
        });
        let owner_account_id = optional_str(attrs, "owner_account_id")
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let tags = extract_tags(attrs);

        let encryption_sse_algorithm = optional_str(attrs, "encryption_sse_algorithm");
        let encryption_kms_key_arn = optional_str(attrs, "encryption_kms_key_arn");
        let maintenance_config: HashMap<String, String> = attrs
            .get("maintenance_config")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        let metrics_config = optional_str(attrs, "metrics_config");
        let policy = optional_str(attrs, "policy");
        let storage_class = optional_str(attrs, "storage_class");
        let replication_config = optional_str(attrs, "replication_config");

        let tb_view = TableBucketView {
            name: name.to_string(),
            arn: arn.clone(),
            owner_account_id,
            created_at: optional_str(attrs, "created_at")
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            tags,
            encryption_sse_algorithm,
            encryption_kms_key_arn,
            maintenance_config,
            metrics_config,
            policy,
            storage_class,
            replication_config,
        };

        let mut state_view = S3TablesStateView::default();
        state_view.table_buckets.insert(arn, tb_view);
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
        for tb in view.table_buckets.values() {
            let attrs = serde_json::json!({
                "id": tb.arn,
                "arn": tb.arn,
                "name": tb.name,
                "owner_account_id": tb.owner_account_id,
                "created_at": tb.created_at,
                "tags": tb.tags,
                "encryption_sse_algorithm": tb.encryption_sse_algorithm,
                "encryption_kms_key_arn": tb.encryption_kms_key_arn,
                "maintenance_config": tb.maintenance_config,
                "metrics_config": tb.metrics_config,
                "policy": tb.policy,
                "storage_class": tb.storage_class,
                "replication_config": tb.replication_config,
            });
            results.push(ExtractedResource {
                name: tb.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_s3tables_namespace
// ---------------------------------------------------------------------------

pub struct AwsS3tablesNamespaceConverter {
    service: Arc<S3TablesService>,
}

impl AwsS3tablesNamespaceConverter {
    pub fn new(service: Arc<S3TablesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3tablesNamespaceConverter {
    fn resource_type(&self) -> &str {
        "aws_s3tables_namespace"
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

impl AwsS3tablesNamespaceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let namespace_name = require_str(attrs, "namespace", "aws_s3tables_namespace")?;
        let table_bucket_arn = require_str(attrs, "table_bucket_arn", "aws_s3tables_namespace")?;
        let owner_account_id = optional_str(attrs, "owner_account_id")
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let created_by =
            optional_str(attrs, "created_by").unwrap_or_else(|| ctx.default_account_id.clone());
        let tags = extract_tags(attrs);

        let namespace_list: Vec<String> = attrs
            .get("namespace_list")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_else(|| vec![namespace_name.to_string()]);

        let ns_view = NamespaceView {
            table_bucket_arn: table_bucket_arn.to_string(),
            namespace: namespace_list,
            name: namespace_name.to_string(),
            owner_account_id,
            created_at: optional_str(attrs, "created_at")
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            created_by,
            tags,
        };

        let mut state_view = S3TablesStateView::default();
        state_view.namespaces.push(ns_view);
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
        for ns in &view.namespaces {
            let attrs = serde_json::json!({
                "id": format!("{}/{}", ns.table_bucket_arn, ns.name),
                "namespace": ns.name,
                "namespace_list": ns.namespace,
                "table_bucket_arn": ns.table_bucket_arn,
                "owner_account_id": ns.owner_account_id,
                "created_at": ns.created_at,
                "created_by": ns.created_by,
                "tags": ns.tags,
            });
            results.push(ExtractedResource {
                name: ns.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
