//! Terraform converters for Neptune resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_neptune::NeptuneService;
use winterbaume_neptune::views::{
    DbClusterView, DbInstanceView, DbParameterGroupView, DbSubnetGroupView, NeptuneStateView,
    ParameterView, ServerlessV2ScalingConfigurationView, TagView,
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
        let attrs = &instance.attributes;
        let identifier = require_str(attrs, "cluster_identifier", "aws_neptune_cluster")?;
        let region = extract_region(attrs, &ctx.default_region);

        let engine = optional_str(attrs, "engine").unwrap_or_else(|| "neptune".to_string());
        let engine_version = optional_str(attrs, "engine_version");
        let port = optional_i64(attrs, "port").map(|v| v as i32);
        let master_username = optional_str(attrs, "master_username");
        let database_name = optional_str(attrs, "database_name");
        let db_subnet_group_name = optional_str(attrs, "neptune_subnet_group_name");
        let db_cluster_parameter_group =
            optional_str(attrs, "neptune_cluster_parameter_group_name");
        let kms_key_id = optional_str(attrs, "kms_key_arn");
        let engine_mode = optional_str(attrs, "engine_mode");
        let storage_encrypted = optional_bool(attrs, "storage_encrypted").unwrap_or(false);
        let copy_tags_to_snapshot = optional_bool(attrs, "copy_tags_to_snapshot").unwrap_or(false);
        let deletion_protection = optional_bool(attrs, "deletion_protection").unwrap_or(false);
        let backup_retention_period =
            optional_i64(attrs, "backup_retention_period").unwrap_or(1) as i32;

        let _skip_final_snapshot = optional_bool(attrs, "skip_final_snapshot").unwrap_or(false);
        let _apply_immediately = optional_bool(attrs, "apply_immediately").unwrap_or(false);
        let _preferred_backup_window = optional_str(attrs, "preferred_backup_window");
        let _iam_database_authentication_enabled =
            optional_bool(attrs, "iam_database_authentication_enabled").unwrap_or(false);

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

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:cluster:{}",
                region, ctx.default_account_id, identifier
            )
        });

        let endpoint = optional_str(attrs, "endpoint").unwrap_or_else(|| {
            format!(
                "{}.cluster-xxxxxxxxxx.{}.neptune.amazonaws.com",
                identifier, region
            )
        });
        let reader_endpoint = optional_str(attrs, "reader_endpoint").unwrap_or_else(|| {
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
            identifier: identifier.to_string(),
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
        state_view
            .db_clusters
            .insert(identifier.to_string(), cluster_view);
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
        let attrs = &instance.attributes;
        let identifier = require_str(attrs, "identifier", "aws_neptune_cluster_instance")?;
        let region = extract_region(attrs, &ctx.default_region);

        let db_instance_class =
            optional_str(attrs, "instance_class").unwrap_or_else(|| "db.r5.large".to_string());
        let engine = optional_str(attrs, "engine").unwrap_or_else(|| "neptune".to_string());
        let engine_version =
            optional_str(attrs, "engine_version").unwrap_or_else(|| "1.2.0.2".to_string());
        let db_subnet_group_name = optional_str(attrs, "neptune_subnet_group_name");
        let db_cluster_identifier = optional_str(attrs, "cluster_identifier");
        let availability_zone = optional_str(attrs, "availability_zone");
        let auto_minor_version_upgrade =
            optional_bool(attrs, "auto_minor_version_upgrade").unwrap_or(true);
        let publicly_accessible = optional_bool(attrs, "publicly_accessible").unwrap_or(false);

        let vpc_security_group_ids: Vec<String> = attrs
            .get("vpc_security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let db_parameter_group_names: Vec<String> =
            optional_str(attrs, "neptune_parameter_group_name")
                .map(|n| vec![n])
                .unwrap_or_default();

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:db:{}",
                region, ctx.default_account_id, identifier
            )
        });

        let endpoint_address = optional_str(attrs, "endpoint").unwrap_or_else(|| {
            format!("{}.xxxxxxxxxx.{}.neptune.amazonaws.com", identifier, region)
        });

        let port = optional_i64(attrs, "port").map(|v| v as i32);
        let storage_encrypted = optional_bool(attrs, "storage_encrypted").unwrap_or(false);
        let kms_key_id = optional_str(attrs, "kms_key_arn");

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let instance_view = DbInstanceView {
            identifier: identifier.to_string(),
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
        state_view
            .db_instances
            .insert(identifier.to_string(), instance_view);
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
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_neptune_subnet_group")?;
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
                "arn:aws:rds:{}:{}:subgrp:{}",
                region, ctx.default_account_id, name
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let subnet_group_view = DbSubnetGroupView {
            name: name.to_string(),
            description,
            vpc_id: optional_str(attrs, "vpc_id"),
            subnet_ids,
            status: "Complete".to_string(),
            arn: arn.clone(),
            tags,
        };

        let mut state_view = NeptuneStateView::default();
        state_view
            .db_subnet_groups
            .insert(name.to_string(), subnet_group_view);
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
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_neptune_parameter_group")?;
        let family = require_str(attrs, "family", "aws_neptune_parameter_group")?;
        let description = optional_str(attrs, "description").unwrap_or_else(|| name.to_string());
        let region = extract_region(attrs, &ctx.default_region);

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
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
            name: name.to_string(),
            family: family.to_string(),
            description,
            arn: arn.clone(),
            tags,
            parameters,
        };

        let mut state_view = NeptuneStateView::default();
        state_view
            .db_parameter_groups
            .insert(name.to_string(), pg_view);
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
