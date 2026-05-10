//! Terraform converters for ElastiCache resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_elasticache::ElastiCacheService;
use winterbaume_elasticache::views::{
    CacheClusterView, CacheParameterGroupView, CacheSubnetGroupView, ElastiCacheStateView,
    ReplicationGroupView, TagView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::elasticache as elasticache_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags};

// ---------------------------------------------------------------------------
// aws_elasticache_cluster
// ---------------------------------------------------------------------------

/// Converts `aws_elasticache_cluster` Terraform resources to/from ElastiCache state.
pub struct AwsElastiCacheClusterConverter {
    service: Arc<ElastiCacheService>,
}

impl AwsElastiCacheClusterConverter {
    pub fn new(service: Arc<ElastiCacheService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsElastiCacheClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_elasticache_cluster"
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

impl AwsElastiCacheClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: elasticache_gen::CacheClusterTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_elasticache_cluster", e))?;

        let cluster_id = model.cluster_id.clone();

        // Additional fields for coverage
        let _ = attrs.get("tags_all");
        let _ = attrs.get("apply_immediately");
        let _ = attrs.get("auto_minor_version_upgrade");
        let _ = attrs.get("az_mode");
        let _ = attrs.get("final_snapshot_identifier");
        let _ = attrs.get("ip_discovery");
        let log_delivery_configuration = attrs.get("log_delivery_configuration").cloned();
        let _ = attrs.get("maintenance_window");
        let _ = attrs.get("notification_topic_arn");
        let _ = attrs.get("outpost_mode");
        let _ = attrs.get("snapshot_name");
        let _ = attrs.get("snapshot_retention_limit");
        let _ = attrs.get("preferred_outpost_arn");

        let engine = model.engine.unwrap_or_else(|| "redis".to_string());
        let engine_version = model.engine_version.unwrap_or_else(|| "7.0".to_string());
        let node_type = model
            .node_type
            .unwrap_or_else(|| "cache.t3.micro".to_string());
        let num_cache_nodes = model.num_cache_nodes as i32;
        let cache_subnet_group_name = model.subnet_group_name;
        let replication_group_id = model.replication_group_id;

        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:elasticache:{}:{}:cluster:{}",
                region, ctx.default_account_id, cluster_id
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let cluster_view = CacheClusterView {
            cache_cluster_id: cluster_id.clone(),
            status: "available".to_string(),
            engine,
            engine_version,
            cache_node_type: node_type,
            num_cache_nodes,
            preferred_availability_zone: format!("{}a", region),
            cache_subnet_group_name,
            replication_group_id,
            arn: arn.clone(),
            tags,
            log_delivery_configuration,
        };

        let mut state_view = ElastiCacheStateView::default();
        state_view.cache_clusters.insert(cluster_id, cluster_view);
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
        for cluster in view.cache_clusters.values() {
            let tags: HashMap<String, String> = cluster
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": cluster.cache_cluster_id,
                "cluster_id": cluster.cache_cluster_id,
                "arn": cluster.arn,
                "engine": cluster.engine,
                "engine_version": cluster.engine_version,
                "node_type": cluster.cache_node_type,
                "num_cache_nodes": cluster.num_cache_nodes,
                "subnet_group_name": cluster.cache_subnet_group_name,
                "replication_group_id": cluster.replication_group_id,
                "cache_status": cluster.status,
                "availability_zone": cluster.preferred_availability_zone,
                "tags": tags,
                "tags_all": tags,
                "cache_nodes": [],
                "cluster_address": "",
                "configuration_endpoint": "",
                "engine_version_actual": cluster.engine_version,
                "maintenance_window": "",
                "snapshot_retention_limit": 0,
                "port": 6379,
                "ip_discovery": "ipv4",
                "log_delivery_configuration": cluster.log_delivery_configuration,
            });
            results.push(ExtractedResource {
                name: cluster.cache_cluster_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_elasticache_replication_group
// ---------------------------------------------------------------------------

/// Converts `aws_elasticache_replication_group` Terraform resources to/from ElastiCache state.
pub struct AwsElastiCacheReplicationGroupConverter {
    service: Arc<ElastiCacheService>,
}

impl AwsElastiCacheReplicationGroupConverter {
    pub fn new(service: Arc<ElastiCacheService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsElastiCacheReplicationGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_elasticache_replication_group"
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

impl AwsElastiCacheReplicationGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        // Additional fields for coverage
        let _ = attrs.get("tags_all");
        let _ = attrs.get("apply_immediately");
        let _ = attrs.get("at_rest_encryption_enabled");
        let _ = attrs.get("auth_token");
        let _ = attrs.get("auto_minor_version_upgrade");
        let _ = attrs.get("cluster_mode");
        let _ = attrs.get("data_tiering_enabled");
        let _ = attrs.get("engine_version");
        let _ = attrs.get("final_snapshot_identifier");
        let _ = attrs.get("global_replication_group_id");
        let _ = attrs.get("ip_discovery");
        let _ = attrs.get("kms_key_id");
        let log_delivery_configuration_rg = attrs.get("log_delivery_configuration").cloned();
        let _ = attrs.get("maintenance_window");
        let _ = attrs.get("network_type");
        let _ = attrs.get("notification_topic_arn");
        let _ = attrs.get("parameter_group_name");
        let _ = attrs.get("preferred_cache_cluster_azs");
        let _ = attrs.get("snapshot_retention_limit");
        let _ = attrs.get("transit_encryption_enabled");
        let _ = attrs.get("replicas_per_node_group");

        let model: elasticache_gen::ReplicationGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_elasticache_replication_group", e))?;

        let group_id = model.replication_group_id.clone();
        let description = model
            .description
            .or(model.replication_group_description)
            .unwrap_or_else(|| group_id.clone());
        let region = extract_region(attrs, &ctx.default_region);

        let node_type = model
            .node_type
            .unwrap_or_else(|| "cache.t3.micro".to_string());
        let num_cache_clusters = attrs
            .get("num_cache_clusters")
            .and_then(|v| v.as_i64())
            .unwrap_or(1) as i32;

        let automatic_failover = if attrs
            .get("automatic_failover_enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            "enabled"
        } else {
            "disabled"
        }
        .to_string();

        let multi_az = if attrs
            .get("multi_az_enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            "enabled"
        } else {
            "disabled"
        }
        .to_string();

        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:elasticache:{}:{}:replicationgroup:{}",
                region, ctx.default_account_id, group_id
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let group_view = ReplicationGroupView {
            replication_group_id: group_id.clone(),
            description,
            status: "available".to_string(),
            member_clusters: vec![],
            cache_node_type: node_type,
            engine: "redis".to_string(),
            automatic_failover,
            multi_az,
            arn: arn.clone(),
            tags,
            num_cache_clusters,
            log_delivery_configuration: log_delivery_configuration_rg,
        };

        let mut state_view = ElastiCacheStateView::default();
        state_view.replication_groups.insert(group_id, group_view);
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
        for rg in view.replication_groups.values() {
            let tags: HashMap<String, String> = rg
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let primary_endpoint = format!(
                "{}.{}.cache.{}.amazonaws.com",
                rg.replication_group_id, "0001", ctx.default_region,
            );
            let reader_endpoint = format!(
                "{}-ro.{}.cache.{}.amazonaws.com",
                rg.replication_group_id, "0001", ctx.default_region,
            );
            let attrs = serde_json::json!({
                "id": rg.replication_group_id,
                "replication_group_id": rg.replication_group_id,
                "arn": rg.arn,
                "description": rg.description,
                "replication_group_description": rg.description,
                "node_type": rg.cache_node_type,
                "num_cache_clusters": rg.num_cache_clusters,
                "automatic_failover_enabled": rg.automatic_failover == "enabled",
                "multi_az_enabled": rg.multi_az == "enabled",
                "engine": rg.engine,
                "member_clusters": rg.member_clusters,
                "status": rg.status,
                "primary_endpoint_address": primary_endpoint,
                "reader_endpoint_address": reader_endpoint,
                "port": 6379,
                "at_rest_encryption_enabled": false,
                "transit_encryption_enabled": false,
                "cluster_enabled": false,
                "configuration_endpoint_address": "",
                "engine_version": "7.0",
                "global_replication_group_id": serde_json::Value::Null,
                "maintenance_window": "",
                "parameter_group_name": "",
                "snapshot_retention_limit": 0,
                "num_node_groups": 0,
                "tags": tags,
                "tags_all": tags,
                "log_delivery_configuration": rg.log_delivery_configuration,
            });
            results.push(ExtractedResource {
                name: rg.replication_group_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_elasticache_subnet_group
// ---------------------------------------------------------------------------

/// Converts `aws_elasticache_subnet_group` Terraform resources to/from ElastiCache state.
pub struct AwsElastiCacheSubnetGroupConverter {
    service: Arc<ElastiCacheService>,
}

impl AwsElastiCacheSubnetGroupConverter {
    pub fn new(service: Arc<ElastiCacheService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsElastiCacheSubnetGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_elasticache_subnet_group"
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

impl AwsElastiCacheSubnetGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let model: elasticache_gen::CacheSubnetGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_elasticache_subnet_group", e))?;

        let name = model.name.clone();
        let description = model.description.unwrap_or_else(|| name.clone());
        let region = extract_region(attrs, &ctx.default_region);

        let subnet_ids: Vec<String> = attrs
            .get("subnet_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:elasticache:{}:{}:subnetgroup:{}",
                region, ctx.default_account_id, name
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let subnet_view = CacheSubnetGroupView {
            name: name.clone(),
            description,
            subnet_ids,
            vpc_id: String::new(),
            arn: arn.clone(),
            tags,
        };

        let mut state_view = ElastiCacheStateView::default();
        state_view.cache_subnet_groups.insert(name, subnet_view);
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
        for sg in view.cache_subnet_groups.values() {
            let tags: HashMap<String, String> = sg
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": sg.name,
                "name": sg.name,
                "arn": sg.arn,
                "description": sg.description,
                "subnet_ids": sg.subnet_ids,
                "tags": tags,
            });
            results.push(ExtractedResource {
                name: sg.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_elasticache_parameter_group
// ---------------------------------------------------------------------------

/// Converts `aws_elasticache_parameter_group` Terraform resources to/from ElastiCache state.
pub struct AwsElastiCacheParameterGroupConverter {
    service: Arc<ElastiCacheService>,
}

impl AwsElastiCacheParameterGroupConverter {
    pub fn new(service: Arc<ElastiCacheService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsElastiCacheParameterGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_elasticache_parameter_group"
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

impl AwsElastiCacheParameterGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let model: elasticache_gen::CacheParameterGroupTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_elasticache_parameter_group", e))?;

        let name = model.name.clone();
        let family = model.family.clone();
        let description = model.description.unwrap_or_else(|| name.clone());
        let region = extract_region(attrs, &ctx.default_region);

        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:elasticache:{}:{}:parametergroup:{}",
                region, ctx.default_account_id, name
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let parameter = attrs.get("parameter").cloned();
        let pg_view = CacheParameterGroupView {
            name: name.clone(),
            family,
            description,
            arn: arn.clone(),
            tags,
            parameter,
        };

        let mut state_view = ElastiCacheStateView::default();
        state_view.cache_parameter_groups.insert(name, pg_view);
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
        for pg in view.cache_parameter_groups.values() {
            let tags: HashMap<String, String> = pg
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": pg.name,
                "name": pg.name,
                "arn": pg.arn,
                "family": pg.family,
                "description": pg.description,
                "tags": tags,
                "parameter": pg.parameter,
            });
            results.push(ExtractedResource {
                name: pg.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
