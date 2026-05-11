//! Terraform converters for Neptune resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_neptune::NeptuneService;
use winterbaume_neptune::views::{
    DbClusterEndpointView, DbClusterParameterGroupView, DbClusterSnapshotView, DbClusterView,
    DbInstanceView, DbParameterGroupView, DbSubnetGroupView, EventSubscriptionView,
    GlobalClusterView, NeptuneStateView, ParameterView, ServerlessV2ScalingConfigurationView,
    TagView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::neptune as neptune_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags, optional_i64};

// ---------------------------------------------------------------------------
// aws_neptune_cluster
// ---------------------------------------------------------------------------

/// Converts `aws_neptune_cluster` Terraform resources to/from Neptune state.
pub struct AwsNeptuneClusterConverter {
    service: Arc<NeptuneService>,
}

impl AwsNeptuneClusterConverter {
    pub fn new(service: Arc<NeptuneService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNeptuneClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_neptune_cluster"
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

impl AwsNeptuneClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: neptune_gen::DbClusterTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_neptune_cluster", e))?;

        let attrs = &instance.attributes;
        let identifier = model.cluster_identifier.clone();

        let engine = model.engine.unwrap_or_else(|| "neptune".to_string());
        let engine_version = model.engine_version;
        let port = optional_i64(attrs, "port").map(|v| v as i32);
        let master_username = model.master_username;
        let database_name = model.database_name;
        let db_subnet_group_name = model.neptune_subnet_group_name;
        let db_cluster_parameter_group = model.neptune_cluster_parameter_group_name;
        let kms_key_id = model.kms_key_arn;
        let engine_mode = model.engine_mode;
        let storage_encrypted = model.storage_encrypted;
        let copy_tags_to_snapshot = model.copy_tags_to_snapshot;
        let deletion_protection = model.deletion_protection;
        let backup_retention_period = model.backup_retention_period as i32;

        // Merge tags_all
        let mut tag_map = extract_tags(attrs);
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tag_map.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let vpc_security_group_ids: Vec<String> = attrs
            .get("vpc_security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let availability_zones: Vec<String> = attrs
            .get("availability_zones")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:cluster:{}",
                region, ctx.default_account_id, identifier
            )
        });

        let endpoint = model.endpoint.unwrap_or_else(|| {
            format!(
                "{}.cluster-xxxxxxxxxx.{}.neptune.amazonaws.com",
                identifier, region
            )
        });
        let reader_endpoint = model.reader_endpoint.unwrap_or_else(|| {
            format!(
                "{}.cluster-ro-xxxxxxxxxx.{}.neptune.amazonaws.com",
                identifier, region
            )
        });

        let tags: Vec<TagView> = tag_map
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let associated_roles: Vec<String> = attrs
            .get("iam_roles")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        // Parse serverless_v2_scaling_configuration block
        let serverless_v2_scaling_configuration = attrs
            .get("serverless_v2_scaling_configuration")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|obj| obj.as_object())
            .and_then(|obj| {
                let min = obj.get("min_capacity")?.as_f64()?;
                let max = obj.get("max_capacity")?.as_f64()?;
                Some(ServerlessV2ScalingConfigurationView {
                    min_capacity: min,
                    max_capacity: max,
                })
            });

        let cluster_view = DbClusterView {
            identifier: identifier.clone(),
            engine,
            engine_version,
            status: "available".to_string(),
            endpoint: Some(endpoint),
            reader_endpoint: Some(reader_endpoint),
            port,
            master_username,
            database_name,
            db_subnet_group_name,
            vpc_security_group_ids,
            availability_zones,
            arn: arn.clone(),
            tags,
            cluster_create_time: None,
            multi_az: false,
            storage_encrypted,
            kms_key_id,
            db_cluster_parameter_group,
            engine_mode,
            copy_tags_to_snapshot,
            deletion_protection,
            backup_retention_period,
            members: vec![],
            associated_roles,
            serverless_v2_scaling_configuration,
        };

        let mut state_view = NeptuneStateView::default();
        state_view.db_clusters.insert(identifier, cluster_view);
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
        for cluster in view.db_clusters.values() {
            let tags: HashMap<String, String> = cluster
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let members: Vec<serde_json::Value> = cluster
                .members
                .iter()
                .map(|m| {
                    serde_json::json!({
                        "db_instance_identifier": m.db_instance_identifier,
                        "is_cluster_writer": m.is_cluster_writer,
                        "db_cluster_parameter_group_status": m.db_cluster_parameter_group_status,
                        "promotion_tier": m.promotion_tier,
                    })
                })
                .collect();
            let serverless_v2 = cluster
                .serverless_v2_scaling_configuration
                .as_ref()
                .map(|c| {
                    serde_json::json!([{
                        "min_capacity": c.min_capacity,
                        "max_capacity": c.max_capacity,
                    }])
                })
                .unwrap_or(serde_json::json!([]));

            let attrs = serde_json::json!({
                "id": cluster.identifier,
                "cluster_identifier": cluster.identifier,
                "arn": cluster.arn,
                "engine": cluster.engine,
                "engine_version": cluster.engine_version,
                "status": cluster.status,
                "endpoint": cluster.endpoint,
                "reader_endpoint": cluster.reader_endpoint,
                "port": cluster.port,
                "master_username": cluster.master_username,
                "database_name": cluster.database_name,
                "neptune_subnet_group_name": cluster.db_subnet_group_name,
                "vpc_security_group_ids": cluster.vpc_security_group_ids,
                "availability_zones": cluster.availability_zones,
                "cluster_create_time": cluster.cluster_create_time,
                "storage_encrypted": cluster.storage_encrypted,
                "kms_key_arn": cluster.kms_key_id,
                "neptune_cluster_parameter_group_name": cluster.db_cluster_parameter_group,
                "engine_mode": cluster.engine_mode,
                "copy_tags_to_snapshot": cluster.copy_tags_to_snapshot,
                "deletion_protection": cluster.deletion_protection,
                "backup_retention_period": cluster.backup_retention_period,
                "cluster_members": members,
                "iam_roles": cluster.associated_roles,
                "serverless_v2_scaling_configuration": serverless_v2,
                "tags": tags,
                "tags_all": tags,
                "skip_final_snapshot": false,
                "apply_immediately": false,
                "preferred_backup_window": null,
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

// ---------------------------------------------------------------------------
// aws_neptune_cluster_instance
// ---------------------------------------------------------------------------

/// Converts `aws_neptune_cluster_instance` Terraform resources to/from Neptune state.
pub struct AwsNeptuneClusterInstanceConverter {
    service: Arc<NeptuneService>,
}

impl AwsNeptuneClusterInstanceConverter {
    pub fn new(service: Arc<NeptuneService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNeptuneClusterInstanceConverter {
    fn resource_type(&self) -> &str {
        "aws_neptune_cluster_instance"
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

impl AwsNeptuneClusterInstanceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: neptune_gen::DbInstanceTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_neptune_cluster_instance", e))?;

        let attrs = &instance.attributes;
        let identifier = model.identifier.clone();

        let db_instance_class = model
            .instance_class
            .unwrap_or_else(|| "db.r5.large".to_string());
        let engine = model.engine.unwrap_or_else(|| "neptune".to_string());
        let engine_version = model
            .engine_version
            .unwrap_or_else(|| "1.2.0.2".to_string());
        let db_subnet_group_name = model.neptune_subnet_group_name;
        let db_cluster_identifier = model.cluster_identifier;
        let availability_zone = model.availability_zone;
        let auto_minor_version_upgrade = model.auto_minor_version_upgrade;
        let publicly_accessible = model.publicly_accessible;

        let vpc_security_group_ids: Vec<String> = attrs
            .get("vpc_security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let db_parameter_group_names: Vec<String> = model
            .neptune_parameter_group_name
            .map(|n| vec![n])
            .unwrap_or_default();

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:db:{}",
                region, ctx.default_account_id, identifier
            )
        });

        let endpoint_address = model.endpoint.unwrap_or_else(|| {
            format!("{}.xxxxxxxxxx.{}.neptune.amazonaws.com", identifier, region)
        });

        let port = optional_i64(attrs, "port").map(|v| v as i32);
        let storage_encrypted = model.storage_encrypted;
        let kms_key_id = model.kms_key_arn;

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let instance_view = DbInstanceView {
            identifier: identifier.clone(),
            db_instance_class,
            engine,
            engine_version,
            status: "available".to_string(),
            endpoint_address: Some(endpoint_address),
            port,
            db_subnet_group_name,
            vpc_security_group_ids,
            availability_zone,
            auto_minor_version_upgrade,
            backup_retention_period: 1,
            db_cluster_identifier,
            arn: arn.clone(),
            tags,
            instance_create_time: None,
            storage_encrypted,
            kms_key_id,
            publicly_accessible,
            deletion_protection: false,
            db_parameter_group_names,
            multi_az: false,
        };

        let mut state_view = NeptuneStateView::default();
        state_view.db_instances.insert(identifier, instance_view);
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
        for inst in view.db_instances.values() {
            let tags: HashMap<String, String> = inst
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": inst.identifier,
                "identifier": inst.identifier,
                "arn": inst.arn,
                "instance_class": inst.db_instance_class,
                "engine": inst.engine,
                "engine_version": inst.engine_version,
                "status": inst.status,
                "endpoint": inst.endpoint_address,
                "port": inst.port,
                "neptune_subnet_group_name": inst.db_subnet_group_name,
                "vpc_security_group_ids": inst.vpc_security_group_ids,
                "availability_zone": inst.availability_zone,
                "auto_minor_version_upgrade": inst.auto_minor_version_upgrade,
                "cluster_identifier": inst.db_cluster_identifier,
                "instance_create_time": inst.instance_create_time,
                "storage_encrypted": inst.storage_encrypted,
                "kms_key_arn": inst.kms_key_id,
                "publicly_accessible": inst.publicly_accessible,
                "neptune_parameter_group_name": inst.db_parameter_group_names.first(),
                "tags": tags,
                "tags_all": tags,
            });
            results.push(ExtractedResource {
                name: inst.identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_neptune_subnet_group
// ---------------------------------------------------------------------------

/// Converts `aws_neptune_subnet_group` Terraform resources to/from Neptune state.
pub struct AwsNeptuneSubnetGroupConverter {
    service: Arc<NeptuneService>,
}

impl AwsNeptuneSubnetGroupConverter {
    pub fn new(service: Arc<NeptuneService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNeptuneSubnetGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_neptune_subnet_group"
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

impl AwsNeptuneSubnetGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: neptune_gen::DbSubnetGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_neptune_subnet_group", e))?;

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
                "arn:aws:rds:{}:{}:subgrp:{}",
                region, ctx.default_account_id, name
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let subnet_group_view = DbSubnetGroupView {
            name: name.clone(),
            description,
            vpc_id: model.vpc_id,
            subnet_ids,
            status: "Complete".to_string(),
            arn: arn.clone(),
            tags,
        };

        let mut state_view = NeptuneStateView::default();
        state_view.db_subnet_groups.insert(name, subnet_group_view);
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
        for sg in view.db_subnet_groups.values() {
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
                "vpc_id": sg.vpc_id,
                "subnet_ids": sg.subnet_ids,
                "status": sg.status,
                "tags": tags,
                "tags_all": tags,
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
// aws_neptune_parameter_group
// ---------------------------------------------------------------------------

/// Converts `aws_neptune_parameter_group` Terraform resources to/from Neptune state.
pub struct AwsNeptuneParameterGroupConverter {
    service: Arc<NeptuneService>,
}

impl AwsNeptuneParameterGroupConverter {
    pub fn new(service: Arc<NeptuneService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNeptuneParameterGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_neptune_parameter_group"
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

impl AwsNeptuneParameterGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: neptune_gen::DbParameterGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_neptune_parameter_group", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let family = model.family.clone();
        let description = model.description.unwrap_or_else(|| name.clone());

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:pg:{}",
                region, ctx.default_account_id, name
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        // Parse parameter blocks
        let parameters: Vec<ParameterView> = attrs
            .get("parameter")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        let obj = item.as_object()?;
                        let param_name = obj.get("name")?.as_str()?.to_string();
                        let value = obj.get("value")?.as_str()?.to_string();
                        let apply_method = obj
                            .get("apply_method")
                            .and_then(|v| v.as_str())
                            .map(String::from);
                        Some(ParameterView {
                            name: param_name,
                            value,
                            apply_method,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let pg_view = DbParameterGroupView {
            name: name.clone(),
            family,
            description,
            arn: arn.clone(),
            tags,
            parameters,
        };

        let mut state_view = NeptuneStateView::default();
        state_view.db_parameter_groups.insert(name, pg_view);
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
        for pg in view.db_parameter_groups.values() {
            let tags: HashMap<String, String> = pg
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let parameters: Vec<serde_json::Value> = pg
                .parameters
                .iter()
                .map(|p| {
                    let mut obj = serde_json::json!({
                        "name": p.name,
                        "value": p.value,
                    });
                    if let Some(ref method) = p.apply_method {
                        obj["apply_method"] = serde_json::json!(method);
                    }
                    obj
                })
                .collect();
            let attrs = serde_json::json!({
                "id": pg.name,
                "name": pg.name,
                "arn": pg.arn,
                "family": pg.family,
                "description": pg.description,
                "parameter": parameters,
                "tags": tags,
                "tags_all": tags,
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

// ---------------------------------------------------------------------------
// aws_neptune_cluster_endpoint
// ---------------------------------------------------------------------------

/// Converts `aws_neptune_cluster_endpoint` Terraform resources to/from Neptune state.
pub struct AwsNeptuneClusterEndpointConverter {
    service: Arc<NeptuneService>,
}

impl AwsNeptuneClusterEndpointConverter {
    pub fn new(service: Arc<NeptuneService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNeptuneClusterEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_neptune_cluster_endpoint"
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

impl AwsNeptuneClusterEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: neptune_gen::DbClusterEndpointTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_neptune_cluster_endpoint", e))?;

        let attrs = &instance.attributes;
        let identifier = model.cluster_endpoint_identifier.clone();

        let static_members: Vec<String> = attrs
            .get("static_members")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let excluded_members: Vec<String> = attrs
            .get("excluded_members")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:cluster-endpoint:{}",
                region, ctx.default_account_id, identifier
            )
        });
        let endpoint = model.endpoint.unwrap_or_else(|| {
            format!(
                "{}.cluster-custom-xxxxxxxxxx.{}.neptune.amazonaws.com",
                identifier, region
            )
        });

        let endpoint_view = DbClusterEndpointView {
            identifier: identifier.clone(),
            db_cluster_identifier: model.cluster_identifier,
            endpoint_type: model.endpoint_type.clone(),
            custom_endpoint_type: Some(model.endpoint_type),
            endpoint: Some(endpoint),
            arn,
            resource_identifier: format!("cluster-endpoint-{}", uuid::Uuid::new_v4().simple()),
            status: "available".to_string(),
            static_members,
            excluded_members,
        };

        let mut state_view = NeptuneStateView::default();
        state_view
            .db_cluster_endpoints
            .insert(identifier, endpoint_view);
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
        for ep in view.db_cluster_endpoints.values() {
            let attrs = serde_json::json!({
                "id": ep.identifier,
                "cluster_endpoint_identifier": ep.identifier,
                "cluster_identifier": ep.db_cluster_identifier,
                "endpoint_type": ep.endpoint_type,
                "endpoint": ep.endpoint,
                "arn": ep.arn,
                "static_members": ep.static_members,
                "excluded_members": ep.excluded_members,
            });
            results.push(ExtractedResource {
                name: ep.identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_neptune_cluster_parameter_group
// ---------------------------------------------------------------------------

/// Converts `aws_neptune_cluster_parameter_group` Terraform resources to/from Neptune state.
pub struct AwsNeptuneClusterParameterGroupConverter {
    service: Arc<NeptuneService>,
}

impl AwsNeptuneClusterParameterGroupConverter {
    pub fn new(service: Arc<NeptuneService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNeptuneClusterParameterGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_neptune_cluster_parameter_group"
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

impl AwsNeptuneClusterParameterGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: neptune_gen::DbClusterParameterGroupTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_neptune_cluster_parameter_group", e)
            })?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let family = model.family.clone();
        let description = model
            .description
            .unwrap_or_else(|| "Managed by Terraform".to_string());

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:cluster-pg:{}",
                region, ctx.default_account_id, name
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        // Parse parameter blocks from raw attributes.
        let parameters: Vec<ParameterView> = attrs
            .get("parameter")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        let obj = item.as_object()?;
                        let param_name = obj.get("name")?.as_str()?.to_string();
                        let value = obj.get("value")?.as_str()?.to_string();
                        let apply_method = obj
                            .get("apply_method")
                            .and_then(|v| v.as_str())
                            .map(String::from);
                        Some(ParameterView {
                            name: param_name,
                            value,
                            apply_method,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let pg_view = DbClusterParameterGroupView {
            name: name.clone(),
            family,
            description,
            arn,
            tags,
            parameters,
        };

        let mut state_view = NeptuneStateView::default();
        state_view.db_cluster_parameter_groups.insert(name, pg_view);
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
        for pg in view.db_cluster_parameter_groups.values() {
            let tags: HashMap<String, String> = pg
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let parameters: Vec<serde_json::Value> = pg
                .parameters
                .iter()
                .map(|p| {
                    let mut obj = serde_json::json!({
                        "name": p.name,
                        "value": p.value,
                    });
                    if let Some(ref method) = p.apply_method {
                        obj["apply_method"] = serde_json::json!(method);
                    }
                    obj
                })
                .collect();
            let attrs = serde_json::json!({
                "id": pg.name,
                "name": pg.name,
                "arn": pg.arn,
                "family": pg.family,
                "description": pg.description,
                "parameter": parameters,
                "tags": tags,
                "tags_all": tags,
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

// ---------------------------------------------------------------------------
// aws_neptune_cluster_snapshot
// ---------------------------------------------------------------------------

/// Converts `aws_neptune_cluster_snapshot` Terraform resources to/from Neptune state.
pub struct AwsNeptuneClusterSnapshotConverter {
    service: Arc<NeptuneService>,
}

impl AwsNeptuneClusterSnapshotConverter {
    pub fn new(service: Arc<NeptuneService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNeptuneClusterSnapshotConverter {
    fn resource_type(&self) -> &str {
        "aws_neptune_cluster_snapshot"
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

impl AwsNeptuneClusterSnapshotConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: neptune_gen::DbClusterSnapshotTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_neptune_cluster_snapshot", e))?;

        let attrs = &instance.attributes;
        let identifier = model.db_cluster_snapshot_identifier.clone();

        let availability_zones: Vec<String> = attrs
            .get("availability_zones")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let arn = model.db_cluster_snapshot_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:cluster-snapshot:{}",
                region, ctx.default_account_id, identifier
            )
        });

        let engine = model.engine.unwrap_or_else(|| "neptune".to_string());
        let status = model.status.unwrap_or_else(|| "available".to_string());
        let snapshot_type = model.snapshot_type.unwrap_or_else(|| "manual".to_string());
        let port = optional_i64(attrs, "port").map(|v| v as i32);

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let snapshot_view = DbClusterSnapshotView {
            identifier: identifier.clone(),
            db_cluster_identifier: model.db_cluster_identifier,
            engine,
            engine_version: model.engine_version,
            allocated_storage: model.allocated_storage as i32,
            status,
            port,
            vpc_id: model.vpc_id,
            cluster_create_time: None,
            master_username: None,
            snapshot_type,
            percent_progress: 100,
            storage_encrypted: model.storage_encrypted,
            kms_key_id: model.kms_key_id,
            db_cluster_snapshot_arn: arn,
            availability_zones,
            snapshot_create_time: None,
            tags,
        };

        let mut state_view = NeptuneStateView::default();
        state_view
            .db_cluster_snapshots
            .insert(identifier, snapshot_view);
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
        for s in view.db_cluster_snapshots.values() {
            let tags: HashMap<String, String> = s
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": s.identifier,
                "db_cluster_snapshot_identifier": s.identifier,
                "db_cluster_identifier": s.db_cluster_identifier,
                "db_cluster_snapshot_arn": s.db_cluster_snapshot_arn,
                "engine": s.engine,
                "engine_version": s.engine_version,
                "allocated_storage": s.allocated_storage,
                "availability_zones": s.availability_zones,
                "kms_key_id": s.kms_key_id,
                "port": s.port,
                "snapshot_type": s.snapshot_type,
                "status": s.status,
                "storage_encrypted": s.storage_encrypted,
                "vpc_id": s.vpc_id,
                "tags": tags,
                "tags_all": tags,
            });
            results.push(ExtractedResource {
                name: s.identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_neptune_event_subscription
// ---------------------------------------------------------------------------

/// Converts `aws_neptune_event_subscription` Terraform resources to/from Neptune state.
pub struct AwsNeptuneEventSubscriptionConverter {
    service: Arc<NeptuneService>,
}

impl AwsNeptuneEventSubscriptionConverter {
    pub fn new(service: Arc<NeptuneService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNeptuneEventSubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_neptune_event_subscription"
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

impl AwsNeptuneEventSubscriptionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: neptune_gen::EventSubscriptionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_neptune_event_subscription", e))?;

        let attrs = &instance.attributes;
        let name = model.subscription_name.clone();

        let event_categories: Vec<String> = attrs
            .get("event_categories")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let source_ids: Vec<String> = attrs
            .get("source_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:es:{}",
                region, ctx.default_account_id, name
            )
        });

        let subscription_view = EventSubscriptionView {
            subscription_name: name.clone(),
            sns_topic_arn: model.sns_topic_arn,
            source_type: model.source_type,
            enabled: model.enabled,
            event_categories,
            source_ids,
            status: "active".to_string(),
            arn,
            customer_aws_id: ctx.default_account_id.clone(),
            subscription_creation_time: None,
        };

        let mut state_view = NeptuneStateView::default();
        state_view
            .event_subscriptions
            .insert(name, subscription_view);
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
        for es in view.event_subscriptions.values() {
            let attrs = serde_json::json!({
                "id": es.subscription_name,
                "name": es.subscription_name,
                "sns_topic_arn": es.sns_topic_arn,
                "source_type": es.source_type,
                "enabled": es.enabled,
                "event_categories": es.event_categories,
                "source_ids": es.source_ids,
                "status": es.status,
                "arn": es.arn,
                "customer_aws_id": es.customer_aws_id,
            });
            results.push(ExtractedResource {
                name: es.subscription_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_neptune_global_cluster
// ---------------------------------------------------------------------------

/// Converts `aws_neptune_global_cluster` Terraform resources to/from Neptune state.
pub struct AwsNeptuneGlobalClusterConverter {
    service: Arc<NeptuneService>,
}

impl AwsNeptuneGlobalClusterConverter {
    pub fn new(service: Arc<NeptuneService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsNeptuneGlobalClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_neptune_global_cluster"
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

impl AwsNeptuneGlobalClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: neptune_gen::GlobalClusterTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_neptune_global_cluster", e))?;

        let identifier = model.global_cluster_identifier.clone();
        let engine = model.engine.unwrap_or_else(|| "neptune".to_string());

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds::{}:global-cluster:{}",
                ctx.default_account_id, identifier
            )
        });

        let global_cluster_view = GlobalClusterView {
            identifier: identifier.clone(),
            engine,
            engine_version: model.engine_version,
            database_name: model.database_name,
            deletion_protection: model.deletion_protection,
            storage_encrypted: model.storage_encrypted,
            status: "available".to_string(),
            arn,
            source_db_cluster_identifier: model.source_db_cluster_identifier,
        };

        let mut state_view = NeptuneStateView::default();
        state_view
            .global_clusters
            .insert(identifier, global_cluster_view);
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
        for gc in view.global_clusters.values() {
            let attrs = serde_json::json!({
                "id": gc.identifier,
                "global_cluster_identifier": gc.identifier,
                "engine": gc.engine,
                "engine_version": gc.engine_version,
                "database_name": gc.database_name,
                "deletion_protection": gc.deletion_protection,
                "storage_encrypted": gc.storage_encrypted,
                "status": gc.status,
                "arn": gc.arn,
                "source_db_cluster_identifier": gc.source_db_cluster_identifier,
            });
            results.push(ExtractedResource {
                name: gc.identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
