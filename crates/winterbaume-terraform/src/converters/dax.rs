//! Terraform converters for DAX resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_dax::DaxService;
use winterbaume_dax::views::{
    DaxClusterView, DaxParameterGroupView, DaxStateView, DaxSubnetGroupView, DaxTagView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_i64, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_dax_cluster
// ---------------------------------------------------------------------------

/// Converts `aws_dax_cluster` Terraform resources to/from DAX state.
pub struct AwsDaxClusterConverter {
    service: Arc<DaxService>,
}

impl AwsDaxClusterConverter {
    pub fn new(service: Arc<DaxService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDaxClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_dax_cluster"
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

impl AwsDaxClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let cluster_name = require_str(attrs, "cluster_name", "aws_dax_cluster")?;
        let region = extract_region(attrs, &ctx.default_region);

        let node_type =
            optional_str(attrs, "node_type").unwrap_or_else(|| "dax.r4.large".to_string());
        let iam_role_arn = optional_str(attrs, "iam_role_arn").unwrap_or_default();
        let replication_factor = optional_i64(attrs, "replication_factor").unwrap_or(1) as i32;
        let description = optional_str(attrs, "description").unwrap_or_default();
        let sse_enabled = attrs
            .get("server_side_encryption")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|obj| obj.get("enabled"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let cluster_endpoint_encryption_type =
            optional_str(attrs, "cluster_endpoint_encryption_type")
                .unwrap_or_else(|| "NONE".to_string());

        let cluster_arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:dax:{}:{}:cache/{}",
                region, ctx.default_account_id, cluster_name
            )
        });

        let _notification_topic_arn = optional_str(attrs, "notification_topic_arn");
        let _parameter_group_name = optional_str(attrs, "parameter_group_name");
        let _subnet_group_name = optional_str(attrs, "subnet_group_name");
        let mut tag_map = extract_tags(attrs);
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tag_map.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }
        let tags: Vec<DaxTagView> = tag_map
            .into_iter()
            .map(|(k, v)| DaxTagView { key: k, value: v })
            .collect();

        let cluster_view = DaxClusterView {
            cluster_name: cluster_name.to_string(),
            cluster_arn,
            node_type,
            status: "available".to_string(),
            description,
            iam_role_arn,
            replication_factor,
            sse_enabled,
            cluster_endpoint_encryption_type,
            tags,
        };

        let mut state_view = DaxStateView::default();
        state_view
            .clusters
            .insert(cluster_name.to_string(), cluster_view);
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
            let tags: HashMap<String, String> = cluster
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": cluster.cluster_name,
                "cluster_name": cluster.cluster_name,
                "arn": cluster.cluster_arn,
                "node_type": cluster.node_type,
                "status": cluster.status,
                "description": cluster.description,
                "iam_role_arn": cluster.iam_role_arn,
                "replication_factor": cluster.replication_factor,
                "server_side_encryption": [{
                    "enabled": cluster.sse_enabled,
                }],
                "cluster_endpoint_encryption_type": cluster.cluster_endpoint_encryption_type,
                "parameter_group_name": "",
                "tags": tags,
                "tags_all": tags,
            });
            results.push(ExtractedResource {
                name: cluster.cluster_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_dax_subnet_group
// ---------------------------------------------------------------------------

/// Converts `aws_dax_subnet_group` Terraform resources to/from DAX state.
pub struct AwsDaxSubnetGroupConverter {
    service: Arc<DaxService>,
}

impl AwsDaxSubnetGroupConverter {
    pub fn new(service: Arc<DaxService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDaxSubnetGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_dax_subnet_group"
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

impl AwsDaxSubnetGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_dax_subnet_group")?;
        let region = extract_region(attrs, &ctx.default_region);

        let description = optional_str(attrs, "description").unwrap_or_default();
        let subnet_ids: Vec<String> = attrs
            .get("subnet_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let vpc_id = optional_str(attrs, "vpc_id");

        let view = DaxSubnetGroupView {
            subnet_group_name: name.to_string(),
            description,
            subnet_ids,
            vpc_id,
        };

        let mut state_view = DaxStateView::default();
        state_view.subnet_groups.insert(name.to_string(), view);
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
        for sg in view.subnet_groups.values() {
            let attrs = serde_json::json!({
                "id": sg.subnet_group_name,
                "name": sg.subnet_group_name,
                "description": sg.description,
                "subnet_ids": sg.subnet_ids,
                "vpc_id": sg.vpc_id,
            });
            results.push(ExtractedResource {
                name: sg.subnet_group_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_dax_parameter_group
// ---------------------------------------------------------------------------

/// Converts `aws_dax_parameter_group` Terraform resources to/from DAX state.
pub struct AwsDaxParameterGroupConverter {
    service: Arc<DaxService>,
}

impl AwsDaxParameterGroupConverter {
    pub fn new(service: Arc<DaxService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDaxParameterGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_dax_parameter_group"
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

impl AwsDaxParameterGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_dax_parameter_group")?;
        let region = extract_region(attrs, &ctx.default_region);

        let description = optional_str(attrs, "description").unwrap_or_default();

        let view = DaxParameterGroupView {
            parameter_group_name: name.to_string(),
            description,
        };

        let mut state_view = DaxStateView::default();
        state_view.parameter_groups.insert(name.to_string(), view);
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
        for pg in view.parameter_groups.values() {
            let attrs = serde_json::json!({
                "id": pg.parameter_group_name,
                "name": pg.parameter_group_name,
                "description": pg.description,
            });
            results.push(ExtractedResource {
                name: pg.parameter_group_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
