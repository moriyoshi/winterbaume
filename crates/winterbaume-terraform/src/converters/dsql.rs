//! Terraform converter for DSQL resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_dsql::DsqlService;
use winterbaume_dsql::views::{ClusterView, DsqlStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_bool, optional_str};

// ---------------------------------------------------------------------------
// aws_dsql_cluster
// ---------------------------------------------------------------------------

/// Converts `aws_dsql_cluster` Terraform resources to/from DSQL state.
pub struct AwsDsqlClusterConverter {
    service: Arc<DsqlService>,
}

impl AwsDsqlClusterConverter {
    pub fn new(service: Arc<DsqlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDsqlClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_dsql_cluster"
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

impl AwsDsqlClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let _tags_all = attrs.get("tags_all");
        let _linked_cluster_arns = attrs.get("linked_cluster_arns");
        let _kms_encryption_key = attrs.get("kms_encryption_key");
        let _multi_region_properties = attrs.get("multi_region_properties");
        let _endpoint = optional_str(attrs, "endpoint");
        let _witness_region = optional_str(attrs, "witness_region");
        let deletion_protection_enabled =
            optional_bool(attrs, "deletion_protection_enabled").unwrap_or(false);

        let identifier =
            optional_str(attrs, "id").unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:dsql:{}:{}:cluster/{}",
                region, ctx.default_account_id, identifier
            )
        });

        let cluster_view = ClusterView {
            identifier: identifier.clone(),
            arn,
            status: "ACTIVE".to_string(),
            creation_time: chrono::Utc::now(),
            deletion_protection_enabled,
            tags: extract_tags(attrs),
        };

        let mut state_view = DsqlStateView {
            clusters: HashMap::new(),
            client_tokens: HashMap::new(),
        };
        state_view.clusters.insert(identifier, cluster_view);
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
        for cluster in view.clusters.values() {
            let endpoint = format!("{}.dsql.{}.on.aws", cluster.identifier, ctx.default_region);
            let attrs = serde_json::json!({
                "id": cluster.identifier,
                "arn": cluster.arn,
                "deletion_protection_enabled": cluster.deletion_protection_enabled,
                "tags": cluster.tags,
                "tags_all": cluster.tags,
                "endpoint": endpoint,
                "witness_region": serde_json::Value::Null,
                "vpc_endpoint_service_name": "",
            });
            results.push(ExtractedResource {
                name: cluster.identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
