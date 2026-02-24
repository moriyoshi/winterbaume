//! Terraform converters for MemoryDB resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_memorydb::MemoryDbService;
use winterbaume_memorydb::views::{
    AclView, ClusterView, MemoryDbStateView, SubnetGroupView, TagView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{
    extract_region, extract_tags, optional_bool, optional_i64, optional_str, require_str,
};

// ---------------------------------------------------------------------------
// aws_memorydb_cluster
// ---------------------------------------------------------------------------

/// Converts `aws_memorydb_cluster` Terraform resources to/from MemoryDB state.
pub struct AwsMemoryDbClusterConverter {
    service: Arc<MemoryDbService>,
}

impl AwsMemoryDbClusterConverter {
    pub fn new(service: Arc<MemoryDbService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMemoryDbClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_memorydb_cluster"
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

impl AwsMemoryDbClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_memorydb_cluster")?;
        let region = extract_region(attrs, &ctx.default_region);

        let node_type =
            optional_str(attrs, "node_type").unwrap_or_else(|| "db.r6g.large".to_string());
        let num_shards = optional_i64(attrs, "num_shards").unwrap_or(1) as i32;
        let num_replicas_per_shard =
            optional_i64(attrs, "num_replicas_per_shard").unwrap_or(1) as i32;
        let description = optional_str(attrs, "description").unwrap_or_default();
        let engine = optional_str(attrs, "engine").unwrap_or_else(|| "redis".to_string());
        let engine_version =
            optional_str(attrs, "engine_version").unwrap_or_else(|| "7.0".to_string());
        let subnet_group_name =
            optional_str(attrs, "subnet_group_name").unwrap_or_else(|| "default".to_string());
        let acl_name = optional_str(attrs, "acl_name").unwrap_or_else(|| "open-access".to_string());
        let parameter_group_name =
            optional_str(attrs, "parameter_group_name").unwrap_or_else(|| "default".to_string());
        let tls_enabled = optional_bool(attrs, "tls_enabled").unwrap_or(true);
        let auto_minor_version_upgrade =
            optional_bool(attrs, "auto_minor_version_upgrade").unwrap_or(true);
        let maintenance_window = optional_str(attrs, "maintenance_window")
            .unwrap_or_else(|| "wed:03:00-wed:04:00".to_string());
        let snapshot_retention_limit =
            optional_i64(attrs, "snapshot_retention_limit").unwrap_or(0) as i32;
        let snapshot_window =
            optional_str(attrs, "snapshot_window").unwrap_or_else(|| "05:00-06:00".to_string());
        let _final_snapshot_name = optional_str(attrs, "final_snapshot_name");
        let _snapshot_name = optional_str(attrs, "snapshot_name");

        // Read tags_all and merge with any other tag sources
        let _tags_all = extract_tags(attrs);

        let security_group_ids: Vec<String> = attrs
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:memorydb:{}:{}:cluster/{}",
                region, ctx.default_account_id, name
            )
        });

        let cluster_view = ClusterView {
            name: name.to_string(),
            arn,
            status: "available".to_string(),
            node_type,
            num_shards,
            num_replicas_per_shard,
            description,
            engine,
            engine_version,
            subnet_group_name,
            security_group_ids,
            maintenance_window,
            snapshot_retention_limit,
            snapshot_window,
            acl_name,
            parameter_group_name,
            tls_enabled,
            auto_minor_version_upgrade,
            creation_time: None,
        };

        let state_view = MemoryDbStateView {
            clusters: {
                let mut m = HashMap::new();
                m.insert(name.to_string(), cluster_view);
                m
            },
            snapshots: HashMap::new(),
            subnet_groups: HashMap::new(),
            acls: HashMap::new(),
            tags: HashMap::new(),
        };
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
            let attrs = serde_json::json!({
                "id": cluster.name,
                "name": cluster.name,
                "arn": cluster.arn,
                "status": cluster.status,
                "node_type": cluster.node_type,
                "num_shards": cluster.num_shards,
                "num_replicas_per_shard": cluster.num_replicas_per_shard,
                "description": cluster.description,
                "engine": cluster.engine,
                "engine_version": cluster.engine_version,
                "subnet_group_name": cluster.subnet_group_name,
                "security_group_ids": cluster.security_group_ids,
                "maintenance_window": cluster.maintenance_window,
                "snapshot_retention_limit": cluster.snapshot_retention_limit,
                "snapshot_window": cluster.snapshot_window,
                "acl_name": cluster.acl_name,
                "parameter_group_name": cluster.parameter_group_name,
                "tls_enabled": cluster.tls_enabled,
                "auto_minor_version_upgrade": cluster.auto_minor_version_upgrade,
                "creation_time": cluster.creation_time,
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: cluster.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_memorydb_subnet_group
// ---------------------------------------------------------------------------

/// Converts `aws_memorydb_subnet_group` Terraform resources to/from MemoryDB state.
pub struct AwsMemoryDbSubnetGroupConverter {
    service: Arc<MemoryDbService>,
}

impl AwsMemoryDbSubnetGroupConverter {
    pub fn new(service: Arc<MemoryDbService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMemoryDbSubnetGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_memorydb_subnet_group"
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

impl AwsMemoryDbSubnetGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_memorydb_subnet_group")?;
        let description = optional_str(attrs, "description").unwrap_or_else(|| name.to_string());
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

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:memorydb:{}:{}:subnetgroup/{}",
                region, ctx.default_account_id, name
            )
        });

        let vpc_id = optional_str(attrs, "vpc_id").unwrap_or_default();

        let subnet_group_view = SubnetGroupView {
            name: name.to_string(),
            arn,
            description,
            vpc_id,
            subnet_ids,
        };

        let state_view = MemoryDbStateView {
            clusters: HashMap::new(),
            snapshots: HashMap::new(),
            subnet_groups: {
                let mut m = HashMap::new();
                m.insert(name.to_string(), subnet_group_view);
                m
            },
            acls: HashMap::new(),
            tags: HashMap::new(),
        };
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
                "id": sg.name,
                "name": sg.name,
                "arn": sg.arn,
                "description": sg.description,
                "vpc_id": sg.vpc_id,
                "subnet_ids": sg.subnet_ids,
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
// aws_memorydb_acl
// ---------------------------------------------------------------------------

/// Converts `aws_memorydb_acl` Terraform resources to/from MemoryDB state.
pub struct AwsMemoryDbAclConverter {
    service: Arc<MemoryDbService>,
}

impl AwsMemoryDbAclConverter {
    pub fn new(service: Arc<MemoryDbService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMemoryDbAclConverter {
    fn resource_type(&self) -> &str {
        "aws_memorydb_acl"
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

impl AwsMemoryDbAclConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let acl_name = require_str(attrs, "name", "aws_memorydb_acl")?;
        let region = extract_region(attrs, &ctx.default_region);

        let status = optional_str(attrs, "status").unwrap_or_else(|| "active".to_string());
        let minimum_engine_version =
            optional_str(attrs, "minimum_engine_version").unwrap_or_else(|| "6.2.6".to_string());

        let user_names: Vec<String> = attrs
            .get("user_names")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:memorydb:{}:{}:acl/{}",
                region, ctx.default_account_id, acl_name
            )
        });

        let tags: Vec<TagView> = attrs
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| TagView {
                        key: k.clone(),
                        value: v.as_str().unwrap_or("").to_string(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let acl_view = AclView {
            acl_name: acl_name.to_string(),
            status,
            user_names,
            minimum_engine_version,
            arn,
            tags,
        };

        let state_view = MemoryDbStateView {
            clusters: HashMap::new(),
            snapshots: HashMap::new(),
            subnet_groups: HashMap::new(),
            acls: {
                let mut m = HashMap::new();
                m.insert(acl_name.to_string(), acl_view);
                m
            },
            tags: HashMap::new(),
        };
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
        for acl in view.acls.values() {
            let tags_map: serde_json::Map<String, serde_json::Value> = acl
                .tags
                .iter()
                .map(|t| (t.key.clone(), serde_json::Value::String(t.value.clone())))
                .collect();
            let attrs = serde_json::json!({
                "id": acl.acl_name,
                "name": acl.acl_name,
                "arn": acl.arn,
                "status": acl.status,
                "user_names": acl.user_names,
                "minimum_engine_version": acl.minimum_engine_version,
                "tags": tags_map,
            });
            results.push(ExtractedResource {
                name: acl.acl_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
