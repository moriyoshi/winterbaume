//! Terraform converters for S3 Tables resources.
//!
//! `TableBucketTfModel` and `NamespaceTfModel` are generated from
//! `specs/s3tables.toml`. The ARN template, the `created_at` constant,
//! the raw `maintenance_config` (HashMap<String,String>), and the
//! `namespace_list` raw read are wired up here.

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
use crate::generated::s3tables as s3tables_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3tables_gen::TableBucketTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_s3tables_table_bucket", e))?;

        let attrs = &instance.attributes;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:s3tables:{}:{}:bucket/{}",
                region, ctx.default_account_id, name
            )
        });
        let owner_account_id = model
            .owner_account_id
            .unwrap_or_else(|| ctx.default_account_id.clone());

        // HashMap<String,String> not in spec vocabulary — read raw.
        let maintenance_config: HashMap<String, String> = attrs
            .get("maintenance_config")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let tb_view = TableBucketView {
            name: name.clone(),
            arn: arn.clone(),
            owner_account_id,
            created_at: model
                .created_at
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            tags: model.tags,
            encryption_sse_algorithm: model.encryption_sse_algorithm,
            encryption_kms_key_arn: model.encryption_kms_key_arn,
            maintenance_config,
            metrics_config: model.metrics_config,
            policy: model.policy,
            storage_class: model.storage_class,
            replication_config: model.replication_config,
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3tables_gen::NamespaceTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_s3tables_namespace", e))?;

        let attrs = &instance.attributes;

        let namespace_name = model.namespace.clone();
        let owner_account_id = model
            .owner_account_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let created_by = model
            .created_by
            .unwrap_or_else(|| ctx.default_account_id.clone());

        // Vec<String> not in spec vocabulary — read raw.
        let namespace_list: Vec<String> = attrs
            .get("namespace_list")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_else(|| vec![namespace_name.clone()]);

        let ns_view = NamespaceView {
            table_bucket_arn: model.table_bucket_arn,
            namespace: namespace_list,
            name: namespace_name,
            owner_account_id,
            created_at: model
                .created_at
                .unwrap_or_else(|| "2023-01-01T00:00:00Z".to_string()),
            created_by,
            tags: model.tags,
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
