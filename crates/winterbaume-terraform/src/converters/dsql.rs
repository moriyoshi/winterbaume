//! Terraform converter for DSQL resources.
//!
//! `ClusterTfModel` is generated from `specs/dsql.toml`. The identifier
//! synthesis (UUID when missing), ARN template, and the `creation_time`
//! / `status` constants are wired up here.

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
use crate::generated::dsql as dsql_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: dsql_gen::ClusterTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_dsql_cluster", e))?;

        let identifier = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = model.arn.unwrap_or_else(|| {
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
            deletion_protection_enabled: model.deletion_protection_enabled,
            tags: model.tags,
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
