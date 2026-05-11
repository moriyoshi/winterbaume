//! Terraform converters for MemoryDB resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_memorydb::MemoryDbService;
use winterbaume_memorydb::views::{
    AclView, ClusterView, MemoryDbStateView, SnapshotView, SubnetGroupView, TagView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::memorydb as memorydb_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: memorydb_gen::ClusterTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_memorydb_cluster", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();

        let node_type = model
            .node_type
            .unwrap_or_else(|| "db.r6g.large".to_string());
        let num_shards = model.num_shards as i32;
        let num_replicas_per_shard = model.num_replicas_per_shard as i32;
        let description = model.description.unwrap_or_default();
        let engine = model.engine.unwrap_or_else(|| "redis".to_string());
        let engine_version = model.engine_version.unwrap_or_else(|| "7.0".to_string());
        let subnet_group_name = model
            .subnet_group_name
            .unwrap_or_else(|| "default".to_string());
        let acl_name = model.acl_name.unwrap_or_else(|| "open-access".to_string());
        let parameter_group_name = model
            .parameter_group_name
            .unwrap_or_else(|| "default".to_string());
        let tls_enabled = model.tls_enabled;
        let auto_minor_version_upgrade = model.auto_minor_version_upgrade;
        let maintenance_window = model
            .maintenance_window
            .unwrap_or_else(|| "wed:03:00-wed:04:00".to_string());
        let snapshot_retention_limit = model.snapshot_retention_limit as i32;
        let snapshot_window = model
            .snapshot_window
            .unwrap_or_else(|| "05:00-06:00".to_string());

        let security_group_ids: Vec<String> = attrs
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:memorydb:{}:{}:cluster/{}",
                region, ctx.default_account_id, name
            )
        });

        let cluster_view = ClusterView {
            name: name.clone(),
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
                m.insert(name, cluster_view);
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: memorydb_gen::SubnetGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_memorydb_subnet_group", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let description = model.description.unwrap_or_else(|| name.clone());

        let subnet_ids: Vec<String> = attrs
            .get("subnet_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:memorydb:{}:{}:subnetgroup/{}",
                region, ctx.default_account_id, name
            )
        });

        let vpc_id = model.vpc_id.unwrap_or_default();

        let subnet_group_view = SubnetGroupView {
            name: name.clone(),
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
                m.insert(name, subnet_group_view);
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: memorydb_gen::AclTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_memorydb_acl", e))?;

        let attrs = &instance.attributes;
        let acl_name = model.name.clone();

        let status = model.status.unwrap_or_else(|| "active".to_string());
        let minimum_engine_version = model
            .minimum_engine_version
            .unwrap_or_else(|| "6.2.6".to_string());

        let user_names: Vec<String> = attrs
            .get("user_names")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let arn = model.arn.unwrap_or_else(|| {
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
            acl_name: acl_name.clone(),
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
                m.insert(acl_name, acl_view);
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

// ---------------------------------------------------------------------------
// aws_memorydb_snapshot
// ---------------------------------------------------------------------------

/// Converts `aws_memorydb_snapshot` Terraform resources to/from MemoryDB state.
pub struct AwsMemoryDbSnapshotConverter {
    service: Arc<MemoryDbService>,
}

impl AwsMemoryDbSnapshotConverter {
    pub fn new(service: Arc<MemoryDbService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMemoryDbSnapshotConverter {
    fn resource_type(&self) -> &str {
        "aws_memorydb_snapshot"
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

impl AwsMemoryDbSnapshotConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: memorydb_gen::SnapshotTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_memorydb_snapshot", e))?;

        let name = model.name.clone();
        let cluster_name = model.cluster_name.clone();
        let source = model.source.unwrap_or_else(|| "manual".to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:memorydb:{}:{}:snapshot/{}",
                region, ctx.default_account_id, name
            )
        });

        let snapshot_view = SnapshotView {
            name: name.clone(),
            arn,
            status: "available".to_string(),
            source,
            cluster_name,
            cluster_description: String::new(),
            cluster_engine: "redis".to_string(),
            cluster_engine_version: "7.0".to_string(),
            cluster_node_type: "db.r6g.large".to_string(),
            cluster_num_shards: 1,
            cluster_subnet_group_name: "default".to_string(),
            cluster_snapshot_retention_limit: 0,
            cluster_snapshot_window: "05:00-06:00".to_string(),
            cluster_maintenance_window: "wed:03:00-wed:04:00".to_string(),
            cluster_parameter_group_name: "default".to_string(),
            kms_key_id: model.kms_key_arn,
        };

        let state_view = MemoryDbStateView {
            snapshots: {
                let mut m = HashMap::new();
                m.insert(name, snapshot_view);
                m
            },
            ..Default::default()
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
        for snap in view.snapshots.values() {
            let attrs = serde_json::json!({
                "id": snap.name,
                "name": snap.name,
                "arn": snap.arn,
                "cluster_name": snap.cluster_name,
                "kms_key_arn": snap.kms_key_id,
                "source": snap.source,
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: snap.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Warning-only converters
//
// `aws_memorydb_parameter_group`, `aws_memorydb_user`, and
// `aws_memorydb_multi_region_cluster` do not have a corresponding state
// slot in `winterbaume_memorydb`. Inject validates the TF attributes
// against the generated model and otherwise emits a warning.
// ---------------------------------------------------------------------------

macro_rules! memorydb_warning_only_converter {
    (
        struct_name = $struct_name:ident,
        resource_type = $resource_type:expr,
        model_type = $model_type:ident,
        warn_msg = $warn_msg:expr $(,)?
    ) => {
        pub struct $struct_name {
            #[allow(dead_code)]
            service: Arc<MemoryDbService>,
        }

        impl $struct_name {
            pub fn new(service: Arc<MemoryDbService>) -> Self {
                Self { service }
            }
        }

        impl TerraformResourceConverter for $struct_name {
            fn resource_type(&self) -> &str {
                $resource_type
            }

            fn inject<'a>(
                &'a self,
                instance: &'a ResourceInstance,
                ctx: &'a ConversionContext,
            ) -> Pin<
                Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>,
            > {
                Box::pin(async move { self.do_inject(instance, ctx).await })
            }

            fn extract<'a>(
                &'a self,
                _ctx: &'a ConversionContext,
            ) -> Pin<
                Box<
                    dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>>
                        + Send
                        + 'a,
                >,
            > {
                Box::pin(async move { Ok(vec![]) })
            }
        }

        impl $struct_name {
            async fn do_inject(
                &self,
                instance: &ResourceInstance,
                ctx: &ConversionContext,
            ) -> Result<ConversionResult, ConversionError> {
                let attrs = &instance.attributes;
                let region = extract_region(attrs, &ctx.default_region);
                let _model: memorydb_gen::$model_type = serde_json::from_value(attrs.clone())
                    .map_err(|e| classify_deserialize_error($resource_type, e))?;
                eprintln!("warning: {}: {}", $resource_type, $warn_msg);
                Ok(ConversionResult {
                    region,
                    warnings: vec![format!("{}: {}", $resource_type, $warn_msg)],
                })
            }
        }
    };
}

memorydb_warning_only_converter! {
    struct_name = AwsMemoryDbParameterGroupConverter,
    resource_type = "aws_memorydb_parameter_group",
    model_type = ParameterGroupTfModel,
    warn_msg = "no state slot in winterbaume_memorydb; inject is a no-op",
}

memorydb_warning_only_converter! {
    struct_name = AwsMemoryDbUserConverter,
    resource_type = "aws_memorydb_user",
    model_type = UserTfModel,
    warn_msg = "no state slot in winterbaume_memorydb; inject is a no-op",
}

memorydb_warning_only_converter! {
    struct_name = AwsMemoryDbMultiRegionClusterConverter,
    resource_type = "aws_memorydb_multi_region_cluster",
    model_type = MultiRegionClusterTfModel,
    warn_msg = "no state slot in winterbaume_memorydb; inject is a no-op",
}
