//! Terraform converters for RDS resources (DB instances, clusters, subnet groups, etc.).
//!
//! The TfModel structs are generated from `specs/rds.toml`. ARN
//! templates, the `endpoint` synthesis (with random UUIDs), the
//! "Managed by Terraform" description default, the discarded
//! Terraform-only flags, the `Vec<String>` array fields, and the
//! `parameter`/`option` nested-block JSON capture are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_rds::RdsService;
use winterbaume_rds::views::{
    DbClusterEndpointView, DbClusterParameterGroupView, DbClusterSnapshotView, DbClusterView,
    DbInstanceView, DbParameterGroupView, DbProxyEndpointView, DbProxyTargetGroupView,
    DbProxyTargetView, DbProxyView, DbShardGroupView, DbSnapshotView, DbSubnetGroupView,
    EventSubscriptionView, ExportTaskView, GlobalClusterView, OptionGroupView, RdsStateView,
    TagView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::rds as rds_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_db_subnet_group
// ---------------------------------------------------------------------------

pub struct AwsDbSubnetGroupConverter {
    service: Arc<RdsService>,
}

impl AwsDbSubnetGroupConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDbSubnetGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_db_subnet_group"
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

impl AwsDbSubnetGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::DbSubnetGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_db_subnet_group", e))?;

        let _tags_all = attrs.get("tags_all");
        let _name_prefix = attrs.get("name_prefix");
        let subnet_ids = extract_string_array(attrs, "subnet_ids");
        let tags = extract_rds_tags(attrs);
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:subgrp:{}",
                region, ctx.default_account_id, model.name
            )
        });
        let description = model.description.unwrap_or_default();

        let view = DbSubnetGroupView {
            name: model.name.clone(),
            description,
            vpc_id: model.vpc_id,
            subnet_ids,
            status: "Complete".to_string(),
            arn,
            tags,
        };

        let mut state_view = minimal_rds_state_view();
        state_view.db_subnet_groups.insert(model.name, view);
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
            let attrs = serde_json::json!({
                "id": sg.name,
                "name": sg.name,
                "description": sg.description,
                "vpc_id": sg.vpc_id,
                "subnet_ids": sg.subnet_ids,
                "arn": sg.arn,
                "tags": tags_to_map(&sg.tags),
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
// aws_db_parameter_group
// ---------------------------------------------------------------------------

pub struct AwsDbParameterGroupConverter {
    service: Arc<RdsService>,
}

impl AwsDbParameterGroupConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDbParameterGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_db_parameter_group"
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

impl AwsDbParameterGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::DbParameterGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_db_parameter_group", e))?;

        let _tags_all = attrs.get("tags_all");
        let _name_prefix = attrs.get("name_prefix");
        let tags = extract_rds_tags(attrs);
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:pg:{}",
                region, ctx.default_account_id, model.name
            )
        });
        let description = model
            .description
            .unwrap_or_else(|| "Managed by Terraform".to_string());
        let parameter = extract_json_array(attrs, "parameter");

        let view = DbParameterGroupView {
            name: model.name.clone(),
            family: model.family,
            description,
            arn,
            tags,
            parameter,
        };

        let mut state_view = minimal_rds_state_view();
        state_view.db_parameter_groups.insert(model.name, view);
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
            let attrs = serde_json::json!({
                "id": pg.name,
                "name": pg.name,
                "family": pg.family,
                "description": pg.description,
                "arn": pg.arn,
                "tags": tags_to_map(&pg.tags),
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

// ---------------------------------------------------------------------------
// aws_rds_cluster_parameter_group
// ---------------------------------------------------------------------------

pub struct AwsRdsClusterParameterGroupConverter {
    service: Arc<RdsService>,
}

impl AwsRdsClusterParameterGroupConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRdsClusterParameterGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_rds_cluster_parameter_group"
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

impl AwsRdsClusterParameterGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::RdsClusterParameterGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_rds_cluster_parameter_group", e))?;

        let _name_prefix = attrs.get("name_prefix");
        let tags = extract_rds_tags(attrs);
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:cluster-pg:{}",
                region, ctx.default_account_id, model.name
            )
        });
        let description = model
            .description
            .unwrap_or_else(|| "Managed by Terraform".to_string());
        let parameter = extract_json_array(attrs, "parameter");

        let view = DbClusterParameterGroupView {
            name: model.name.clone(),
            family: model.family,
            description,
            arn,
            tags,
            parameter,
        };

        let mut state_view = minimal_rds_state_view();
        state_view
            .db_cluster_parameter_groups
            .insert(model.name, view);
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
            let attrs = serde_json::json!({
                "id": pg.name,
                "name": pg.name,
                "family": pg.family,
                "description": pg.description,
                "arn": pg.arn,
                "tags": tags_to_map(&pg.tags),
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

// ---------------------------------------------------------------------------
// aws_db_option_group
// ---------------------------------------------------------------------------

pub struct AwsDbOptionGroupConverter {
    service: Arc<RdsService>,
}

impl AwsDbOptionGroupConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDbOptionGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_db_option_group"
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

impl AwsDbOptionGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::DbOptionGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_db_option_group", e))?;

        let _name_prefix = attrs.get("name_prefix");
        let mut tags = extract_rds_tags(attrs);
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(val) = v.as_str() {
                    let key = k.clone();
                    let tv = TagView {
                        key: key.clone(),
                        value: val.to_string(),
                    };
                    if !tags.iter().any(|t| t.key == key) {
                        tags.push(tv);
                    }
                }
            }
        }
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:og:{}",
                region, ctx.default_account_id, model.name
            )
        });
        let description = model
            .option_group_description
            .unwrap_or_else(|| "Managed by Terraform".to_string());
        let option = extract_json_array(attrs, "option");

        let view = OptionGroupView {
            name: model.name.clone(),
            engine_name: model.engine_name,
            major_engine_version: model.major_engine_version,
            description,
            allows_vpc_and_non_vpc_instance_memberships: true,
            vpc_id: model.vpc_id,
            arn,
            tags,
            option,
        };

        let mut state_view = minimal_rds_state_view();
        state_view.option_groups.insert(model.name, view);
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
        for og in view.option_groups.values() {
            let tag_map = tags_to_map(&og.tags);
            let attrs = serde_json::json!({
                "id": og.name,
                "name": og.name,
                "engine_name": og.engine_name,
                "major_engine_version": og.major_engine_version,
                "option_group_description": og.description,
                "arn": og.arn,
                "tags": tag_map,
                "tags_all": tag_map,
                "option": og.option,
            });
            results.push(ExtractedResource {
                name: og.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_db_instance
// ---------------------------------------------------------------------------

pub struct AwsDbInstanceConverter {
    service: Arc<RdsService>,
}

impl AwsDbInstanceConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDbInstanceConverter {
    fn resource_type(&self) -> &str {
        "aws_db_instance"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_db_subnet_group", "aws_db_parameter_group"]
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

impl AwsDbInstanceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::DbInstanceTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_db_instance", e))?;

        // Additional fields for coverage (Terraform-only or untyped pass-throughs)
        let _ = attrs.get("tags_all");
        let _ = attrs.get("allow_major_version_upgrade");
        let _ = attrs.get("apply_immediately");
        let _ = attrs.get("backup_window");
        let _ = attrs.get("character_set_name");
        let _ = attrs.get("custom_iam_instance_profile");
        let _ = attrs.get("dedicated_log_volume");
        let _ = attrs.get("domain");
        let _ = attrs.get("domain_iam_role_name");
        let _ = attrs.get("enabled_cloudwatch_logs_exports");
        let _ = attrs.get("final_snapshot_identifier");
        let _ = attrs.get("maintenance_window");
        let _ = attrs.get("manage_master_user_password");
        let _ = attrs.get("master_user_secret_kms_key_id");
        let _ = attrs.get("max_allocated_storage");
        let _ = attrs.get("nchar_character_set_name");
        let _ = attrs.get("network_type");
        let _ = attrs.get("option_group_name");
        let _ = attrs.get("performance_insights_kms_key_id");
        let _ = attrs.get("performance_insights_retention_period");
        let _ = attrs.get("replica_mode");
        let _ = attrs.get("skip_final_snapshot");
        let _ = attrs.get("timezone");

        let blue_green_update = attrs.get("blue_green_update").cloned();
        let restore_to_point_in_time = attrs.get("restore_to_point_in_time").cloned();
        let s3_import = attrs.get("s3_import").cloned();

        let tags = extract_rds_tags(attrs);

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:db:{}",
                region, ctx.default_account_id, model.identifier
            )
        });

        let engine_version = model.engine_version.unwrap_or_default();
        let allocated_storage = model.allocated_storage as i32;

        let vpc_sg_ids = extract_string_array(attrs, "vpc_security_group_ids");
        let param_group_name = attrs
            .get("parameter_group_name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let param_groups = param_group_name.into_iter().collect::<Vec<_>>();

        let endpoint_address = model.address.or_else(|| {
            Some(format!(
                "{}.{}.{}.rds.amazonaws.com",
                model.identifier,
                &uuid::Uuid::new_v4().to_string()[..8],
                region
            ))
        });

        // Option<i32> view fields read raw (Option<i64> in spec rules → drop).
        let port = attrs.get("port").and_then(|v| v.as_i64()).map(|p| p as i32);
        let iops = attrs.get("iops").and_then(|v| v.as_i64()).map(|v| v as i32);
        let monitoring_interval = attrs
            .get("monitoring_interval")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let view = DbInstanceView {
            identifier: model.identifier.clone(),
            db_instance_class: model.instance_class,
            engine: model.engine,
            engine_version,
            status: "available".to_string(),
            master_username: model.username,
            db_name: model.db_name,
            endpoint_address,
            port,
            multi_az: model.multi_az,
            storage_type: model.storage_type,
            allocated_storage,
            db_subnet_group_name: model.db_subnet_group_name,
            vpc_security_group_ids: vpc_sg_ids,
            db_parameter_group_names: param_groups,
            availability_zone: model.availability_zone,
            publicly_accessible: model.publicly_accessible,
            auto_minor_version_upgrade: model.auto_minor_version_upgrade,
            backup_retention_period: model.backup_retention_period as i32,
            db_cluster_identifier: model.cluster_identifier,
            arn: arn.clone(),
            tags,
            instance_create_time: None,
            license_model: model.license_model,
            iops,
            deletion_protection: model.deletion_protection,
            copy_tags_to_snapshot: model.copy_tags_to_snapshot,
            monitoring_interval,
            performance_insights_enabled: model.performance_insights_enabled,
            storage_encrypted: model.storage_encrypted,
            kms_key_id: model.kms_key_id,
            ca_certificate_identifier: model.ca_cert_identifier,
            secondary_availability_zone: None,
            blue_green_update,
            restore_to_point_in_time,
            s3_import,
        };

        let mut state_view = minimal_rds_state_view();
        state_view.db_instances.insert(model.identifier, view);
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
        for dbi in view.db_instances.values() {
            let attrs = serde_json::json!({
                "id": dbi.identifier,
                "identifier": dbi.identifier,
                "engine": dbi.engine,
                "engine_version": dbi.engine_version,
                "instance_class": dbi.db_instance_class,
                "allocated_storage": dbi.allocated_storage,
                "storage_type": dbi.storage_type,
                "username": dbi.master_username,
                "db_name": dbi.db_name,
                "port": dbi.port,
                "multi_az": dbi.multi_az,
                "publicly_accessible": dbi.publicly_accessible,
                "availability_zone": dbi.availability_zone,
                "db_subnet_group_name": dbi.db_subnet_group_name,
                "vpc_security_group_ids": dbi.vpc_security_group_ids,
                "parameter_group_name": dbi.db_parameter_group_names.first(),
                "backup_retention_period": dbi.backup_retention_period,
                "deletion_protection": dbi.deletion_protection,
                "storage_encrypted": dbi.storage_encrypted,
                "kms_key_id": dbi.kms_key_id,
                "address": dbi.endpoint_address,
                "arn": dbi.arn,
                "instance_create_time": dbi.instance_create_time,
                "license_model": dbi.license_model,
                "ca_cert_identifier": dbi.ca_certificate_identifier,
                "iops": dbi.iops,
                "monitoring_interval": dbi.monitoring_interval,
                "performance_insights_enabled": dbi.performance_insights_enabled,
                "copy_tags_to_snapshot": dbi.copy_tags_to_snapshot,
                "tags": tags_to_map(&dbi.tags),
                "tags_all": tags_to_map(&dbi.tags),
                "endpoint": dbi.endpoint_address,
                "hosted_zone_id": "",
                "latest_restorable_time": "",
                "maintenance_window": "",
                "master_user_secret": [],
                "option_group_name": "",
                "performance_insights_kms_key_id": "",
                "performance_insights_retention_period": 0,
                "replicas": [],
                "resource_id": dbi.identifier,
                "status": dbi.status,
                "character_set_name": serde_json::Value::Null,
                "network_type": "IPV4",
                "replica_mode": serde_json::Value::Null,
                "blue_green_update": dbi.blue_green_update,
                "restore_to_point_in_time": dbi.restore_to_point_in_time,
                "s3_import": dbi.s3_import,
            });
            results.push(ExtractedResource {
                name: dbi.identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_rds_cluster
// ---------------------------------------------------------------------------

pub struct AwsRdsClusterConverter {
    service: Arc<RdsService>,
}

impl AwsRdsClusterConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRdsClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_rds_cluster"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_db_subnet_group", "aws_rds_cluster_parameter_group"]
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

impl AwsRdsClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::RdsClusterTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_rds_cluster", e))?;

        // Additional fields for coverage (Terraform-only or untyped pass-throughs)
        let _ = attrs.get("tags_all");
        let _ = attrs.get("allow_major_version_upgrade");
        let _ = attrs.get("apply_immediately");
        let _ = attrs.get("backtrack_window");
        let _ = attrs.get("ca_certificate_identifier");
        let _ = attrs.get("cluster_members");
        let _ = attrs.get("db_cluster_instance_class");
        let _ = attrs.get("db_system_id");
        let _ = attrs.get("enable_global_write_forwarding");
        let _ = attrs.get("enable_http_endpoint");
        let _ = attrs.get("enable_local_write_forwarding");
        let _ = attrs.get("enabled_cloudwatch_logs_exports");
        let _ = attrs.get("final_snapshot_identifier");
        let _ = attrs.get("global_cluster_identifier");
        let _ = attrs.get("iam_database_authentication_enabled");
        let _ = attrs.get("iam_roles");
        let _ = attrs.get("manage_master_user_password");
        let _ = attrs.get("master_user_secret_kms_key_id");
        let _ = attrs.get("network_type");
        let _ = attrs.get("performance_insights_enabled");
        let _ = attrs.get("performance_insights_kms_key_id");
        let _ = attrs.get("performance_insights_retention_period");
        let _ = attrs.get("preferred_backup_window");
        let _ = attrs.get("preferred_maintenance_window");
        let _ = attrs.get("skip_final_snapshot");
        let _ = attrs.get("cluster_identifier_prefix");
        let _ = attrs.get("delete_automated_backups");

        let restore_to_point_in_time = attrs.get("restore_to_point_in_time").cloned();
        let s3_import = attrs.get("s3_import").cloned();
        let scaling_configuration = attrs.get("scaling_configuration").cloned();
        let serverlessv2_scaling_configuration =
            attrs.get("serverlessv2_scaling_configuration").cloned();

        let tags = extract_rds_tags(attrs);

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:cluster:{}",
                region, ctx.default_account_id, model.cluster_identifier
            )
        });

        let vpc_sg_ids = extract_string_array(attrs, "vpc_security_group_ids");
        let azs = extract_string_array(attrs, "availability_zones");

        let endpoint = model.endpoint.or_else(|| {
            Some(format!(
                "{}.cluster-{}.{}.rds.amazonaws.com",
                model.cluster_identifier,
                &uuid::Uuid::new_v4().to_string()[..12],
                region
            ))
        });
        let reader_endpoint = model.reader_endpoint.or_else(|| {
            Some(format!(
                "{}.cluster-ro-{}.{}.rds.amazonaws.com",
                model.cluster_identifier,
                &uuid::Uuid::new_v4().to_string()[..12],
                region
            ))
        });

        // Option<i32> view fields read raw (Option<i64> in spec rules → drop).
        let port = attrs.get("port").and_then(|v| v.as_i64()).map(|p| p as i32);
        let allocated_storage = attrs
            .get("allocated_storage")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let view = DbClusterView {
            identifier: model.cluster_identifier.clone(),
            engine: model.engine,
            engine_version: model.engine_version,
            status: "available".to_string(),
            endpoint,
            reader_endpoint,
            port,
            master_username: model.master_username,
            database_name: model.database_name,
            db_subnet_group_name: model.db_subnet_group_name,
            vpc_security_group_ids: vpc_sg_ids,
            availability_zones: azs,
            arn: arn.clone(),
            tags,
            cluster_create_time: None,
            multi_az: model.multi_az,
            storage_type: model.storage_type,
            allocated_storage,
            backup_retention_period: model.backup_retention_period as i32,
            deletion_protection: model.deletion_protection,
            storage_encrypted: model.storage_encrypted,
            kms_key_id: model.kms_key_id,
            db_cluster_parameter_group: model.db_cluster_parameter_group_name,
            engine_mode: model.engine_mode,
            copy_tags_to_snapshot: model.copy_tags_to_snapshot,
            members: vec![],
            restore_to_point_in_time,
            s3_import,
            scaling_configuration,
            serverlessv2_scaling_configuration,
        };

        let mut state_view = minimal_rds_state_view();
        state_view
            .db_clusters
            .insert(model.cluster_identifier, view);
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
            let cluster_members: Vec<serde_json::Value> = cluster
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
            let attrs = serde_json::json!({
                "id": cluster.identifier,
                "cluster_identifier": cluster.identifier,
                "engine": cluster.engine,
                "engine_version": cluster.engine_version,
                "status": cluster.status,
                "endpoint": cluster.endpoint,
                "reader_endpoint": cluster.reader_endpoint,
                "port": cluster.port,
                "master_username": cluster.master_username,
                "database_name": cluster.database_name,
                "db_subnet_group_name": cluster.db_subnet_group_name,
                "vpc_security_group_ids": cluster.vpc_security_group_ids,
                "availability_zones": cluster.availability_zones,
                "backup_retention_period": cluster.backup_retention_period,
                "deletion_protection": cluster.deletion_protection,
                "storage_encrypted": cluster.storage_encrypted,
                "kms_key_id": cluster.kms_key_id,
                "db_cluster_parameter_group_name": cluster.db_cluster_parameter_group,
                "engine_mode": cluster.engine_mode,
                "cluster_create_time": cluster.cluster_create_time,
                "copy_tags_to_snapshot": cluster.copy_tags_to_snapshot,
                "multi_az": cluster.multi_az,
                "storage_type": cluster.storage_type,
                "allocated_storage": cluster.allocated_storage,
                "cluster_members": cluster_members,
                "arn": cluster.arn,
                "tags": tags_to_map(&cluster.tags),
                "tags_all": tags_to_map(&cluster.tags),
                "cluster_resource_id": cluster.identifier,
                "hosted_zone_id": "",
                "master_user_secret": [],
                "network_type": "IPV4",
                "performance_insights_enabled": false,
                "preferred_backup_window": "",
                "preferred_maintenance_window": "",
                "ca_certificate_identifier": serde_json::Value::Null,
                "db_cluster_instance_class": serde_json::Value::Null,
                "enabled_cloudwatch_logs_exports": [],
                "iam_database_authentication_enabled": false,
                "iam_roles": [],
                "availability_zones": cluster.availability_zones,
                "reader_endpoint": cluster.reader_endpoint,
                "port": cluster.port,
                "engine_version_actual": cluster.engine_version,
                "restore_to_point_in_time": cluster.restore_to_point_in_time,
                "s3_import": cluster.s3_import,
                "scaling_configuration": cluster.scaling_configuration,
                "serverlessv2_scaling_configuration": cluster.serverlessv2_scaling_configuration,
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
// aws_db_event_subscription
// ---------------------------------------------------------------------------

pub struct AwsDbEventSubscriptionConverter {
    service: Arc<RdsService>,
}

impl AwsDbEventSubscriptionConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDbEventSubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_db_event_subscription"
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

impl AwsDbEventSubscriptionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::DbEventSubscriptionTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_db_event_subscription", e))?;

        let _tags_all = attrs.get("tags_all");
        let _name_prefix = attrs.get("name_prefix");
        let _timeouts = attrs.get("timeouts");
        let source_ids = extract_string_array(attrs, "source_ids");
        let event_categories = extract_string_array(attrs, "event_categories");

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:es:{}",
                region, ctx.default_account_id, model.name
            )
        });

        let view = EventSubscriptionView {
            subscription_name: model.name.clone(),
            sns_topic_arn: model.sns_topic,
            source_type: model.source_type,
            source_ids,
            event_categories,
            enabled: model.enabled,
            status: "active".to_string(),
            arn,
            customer_aws_id: ctx.default_account_id.clone(),
            subscription_creation_time: String::new(),
        };

        let mut state_view = minimal_rds_state_view();
        state_view.event_subscriptions.insert(model.name, view);
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
                "sns_topic": es.sns_topic_arn,
                "source_type": es.source_type,
                "source_ids": es.source_ids,
                "event_categories": es.event_categories,
                "enabled": es.enabled,
                "arn": es.arn,
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
// aws_db_proxy
// ---------------------------------------------------------------------------

pub struct AwsDbProxyConverter {
    service: Arc<RdsService>,
}

impl AwsDbProxyConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDbProxyConverter {
    fn resource_type(&self) -> &str {
        "aws_db_proxy"
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

impl AwsDbProxyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::DbProxyTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_db_proxy", e))?;

        let _ = attrs.get("tags_all");
        let _ = attrs.get("auth");
        let _ = attrs.get("default_auth_scheme");
        let _ = attrs.get("endpoint_network_type");
        let _ = attrs.get("target_connection_network_type");

        let tags = extract_rds_tags(attrs);
        let vpc_security_group_ids = extract_string_array(attrs, "vpc_security_group_ids");
        let vpc_subnet_ids = extract_string_array(attrs, "vpc_subnet_ids");

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:db-proxy:prx-{}",
                region,
                ctx.default_account_id,
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..17]
            )
        });
        let endpoint = model.endpoint.unwrap_or_else(|| {
            format!(
                "{}.proxy-{}.{}.rds.amazonaws.com",
                model.name,
                &uuid::Uuid::new_v4().to_string()[..12],
                region
            )
        });

        let view = DbProxyView {
            db_proxy_name: model.name.clone(),
            db_proxy_arn: arn,
            status: "available".to_string(),
            engine_family: model.engine_family,
            vpc_id: model.vpc_id,
            vpc_security_group_ids,
            vpc_subnet_ids,
            endpoint,
            require_tls: model.require_tls,
            idle_client_timeout: model.idle_client_timeout as i32,
            debug_logging: model.debug_logging,
            role_arn: model.role_arn,
            created_date: None,
            updated_date: None,
            tags,
            targets: Vec::new(),
        };

        let mut state_view = minimal_rds_state_view();
        state_view.db_proxies.insert(model.name, view);
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
        for p in view.db_proxies.values() {
            let attrs = serde_json::json!({
                "id": p.db_proxy_name,
                "name": p.db_proxy_name,
                "arn": p.db_proxy_arn,
                "engine_family": p.engine_family,
                "vpc_id": p.vpc_id,
                "vpc_security_group_ids": p.vpc_security_group_ids,
                "vpc_subnet_ids": p.vpc_subnet_ids,
                "endpoint": p.endpoint,
                "require_tls": p.require_tls,
                "idle_client_timeout": p.idle_client_timeout,
                "debug_logging": p.debug_logging,
                "role_arn": p.role_arn,
                "tags": tags_to_map(&p.tags),
                "tags_all": tags_to_map(&p.tags),
            });
            results.push(ExtractedResource {
                name: p.db_proxy_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_db_proxy_default_target_group
// ---------------------------------------------------------------------------

pub struct AwsDbProxyDefaultTargetGroupConverter {
    service: Arc<RdsService>,
}

impl AwsDbProxyDefaultTargetGroupConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDbProxyDefaultTargetGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_db_proxy_default_target_group"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_db_proxy"]
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

impl AwsDbProxyDefaultTargetGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::DbProxyDefaultTargetGroupTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_db_proxy_default_target_group", e))?;

        let connection_pool_config = attrs.get("connection_pool_config").map(|v| v.to_string());

        let target_group_name = model.name.unwrap_or_else(|| "default".to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:target-group:prx-tg-{}",
                region,
                ctx.default_account_id,
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..17]
            )
        });
        let key = format!("{}/{}", model.db_proxy_name, target_group_name);

        let view = DbProxyTargetGroupView {
            target_group_name: target_group_name.clone(),
            db_proxy_name: model.db_proxy_name.clone(),
            target_group_arn: arn,
            is_default: true,
            status: "available".to_string(),
            connection_pool_config,
            created_date: None,
            updated_date: None,
        };

        let mut state_view = minimal_rds_state_view();
        state_view.db_proxy_target_groups.insert(key, view);
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
        for tg in view.db_proxy_target_groups.values() {
            if !tg.is_default {
                continue;
            }
            let cpc = tg
                .connection_pool_config
                .as_ref()
                .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
                .unwrap_or(serde_json::Value::Array(vec![]));
            let attrs = serde_json::json!({
                "id": tg.db_proxy_name,
                "db_proxy_name": tg.db_proxy_name,
                "name": tg.target_group_name,
                "arn": tg.target_group_arn,
                "connection_pool_config": cpc,
            });
            results.push(ExtractedResource {
                name: tg.db_proxy_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_db_proxy_endpoint
// ---------------------------------------------------------------------------

pub struct AwsDbProxyEndpointConverter {
    service: Arc<RdsService>,
}

impl AwsDbProxyEndpointConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDbProxyEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_db_proxy_endpoint"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_db_proxy"]
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

impl AwsDbProxyEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::DbProxyEndpointTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_db_proxy_endpoint", e))?;

        let _ = attrs.get("tags_all");
        let vpc_security_group_ids = extract_string_array(attrs, "vpc_security_group_ids");
        let vpc_subnet_ids = extract_string_array(attrs, "vpc_subnet_ids");

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:db-proxy-endpoint:prx-endpoint-{}",
                region,
                ctx.default_account_id,
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..17]
            )
        });
        let endpoint = model.endpoint.unwrap_or_else(|| {
            format!(
                "{}.endpoint.proxy-{}.{}.rds.amazonaws.com",
                model.db_proxy_endpoint_name,
                &uuid::Uuid::new_v4().to_string()[..12],
                region
            )
        });
        let target_role = model
            .target_role
            .unwrap_or_else(|| "READ_WRITE".to_string());

        let view = DbProxyEndpointView {
            db_proxy_endpoint_name: model.db_proxy_endpoint_name.clone(),
            db_proxy_endpoint_arn: arn,
            db_proxy_name: model.db_proxy_name.clone(),
            status: "available".to_string(),
            vpc_id: model.vpc_id,
            vpc_security_group_ids,
            vpc_subnet_ids,
            endpoint,
            is_default: model.is_default,
            target_role,
            created_date: None,
        };

        let key = format!("{}/{}", model.db_proxy_name, model.db_proxy_endpoint_name);
        let mut state_view = minimal_rds_state_view();
        state_view.db_proxy_endpoints.insert(key, view);
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
        for ep in view.db_proxy_endpoints.values() {
            let attrs = serde_json::json!({
                "id": format!("{}/{}", ep.db_proxy_name, ep.db_proxy_endpoint_name),
                "db_proxy_name": ep.db_proxy_name,
                "db_proxy_endpoint_name": ep.db_proxy_endpoint_name,
                "arn": ep.db_proxy_endpoint_arn,
                "endpoint": ep.endpoint,
                "vpc_id": ep.vpc_id,
                "vpc_security_group_ids": ep.vpc_security_group_ids,
                "vpc_subnet_ids": ep.vpc_subnet_ids,
                "is_default": ep.is_default,
                "target_role": ep.target_role,
            });
            results.push(ExtractedResource {
                name: ep.db_proxy_endpoint_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_db_proxy_target
// ---------------------------------------------------------------------------
//
// Sub-resource of aws_db_proxy. We snapshot+mutate+restore the parent
// `DbProxyView.targets` list, deliberately bypassing the generated model.
pub struct AwsDbProxyTargetConverter {
    service: Arc<RdsService>,
}

impl AwsDbProxyTargetConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDbProxyTargetConverter {
    fn resource_type(&self) -> &str {
        "aws_db_proxy_target"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_db_proxy", "aws_db_instance", "aws_rds_cluster"]
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

impl AwsDbProxyTargetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let db_proxy_name = attrs
            .get("db_proxy_name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ConversionError::MissingAttribute {
                resource_type: "aws_db_proxy_target".into(),
                attribute: "db_proxy_name".into(),
            })?
            .to_string();
        let _target_group_name = attrs
            .get("target_group_name")
            .and_then(|v| v.as_str())
            .unwrap_or("default")
            .to_string();
        let db_instance_identifier = attrs
            .get("db_instance_identifier")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let db_cluster_identifier = attrs
            .get("db_cluster_identifier")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let target_arn = attrs
            .get("target_arn")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                if let Some(id) = &db_cluster_identifier {
                    format!(
                        "arn:aws:rds:{}:{}:cluster:{}",
                        region, ctx.default_account_id, id
                    )
                } else if let Some(id) = &db_instance_identifier {
                    format!(
                        "arn:aws:rds:{}:{}:db:{}",
                        region, ctx.default_account_id, id
                    )
                } else {
                    String::new()
                }
            });
        let endpoint = attrs
            .get("endpoint")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let port = attrs.get("port").and_then(|v| v.as_i64()).map(|p| p as i32);
        let rds_resource_id = attrs
            .get("rds_resource_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let tracked_cluster_id = attrs
            .get("tracked_cluster_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| db_cluster_identifier.clone());
        let type_ = attrs
            .get("type")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| {
                if db_cluster_identifier.is_some() {
                    Some("TRACKED_CLUSTER".to_string())
                } else {
                    Some("RDS_INSTANCE".to_string())
                }
            });

        let new_target = DbProxyTargetView {
            target_arn,
            endpoint,
            tracked_cluster_id,
            rds_resource_id,
            port,
            type_,
            role: None,
            target_health_status: Some("AVAILABLE".to_string()),
        };

        // Snapshot+mutate+restore to add target to parent proxy.
        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(proxy) = view.db_proxies.get_mut(&db_proxy_name) {
            if !proxy
                .targets
                .iter()
                .any(|t| t.target_arn == new_target.target_arn)
            {
                proxy.targets.push(new_target);
            }
        } else {
            warnings.push(format!(
                "db_proxy '{db_proxy_name}' not found; target attachment skipped"
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
        for proxy in view.db_proxies.values() {
            for tgt in &proxy.targets {
                let attrs = serde_json::json!({
                    "id": format!("{}/default/{}", proxy.db_proxy_name, tgt.target_arn),
                    "db_proxy_name": proxy.db_proxy_name,
                    "target_group_name": "default",
                    "target_arn": tgt.target_arn,
                    "endpoint": tgt.endpoint,
                    "port": tgt.port,
                    "rds_resource_id": tgt.rds_resource_id,
                    "tracked_cluster_id": tgt.tracked_cluster_id,
                    "type": tgt.type_,
                });
                results.push(ExtractedResource {
                    name: format!("{}/{}", proxy.db_proxy_name, tgt.target_arn),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_db_snapshot
// ---------------------------------------------------------------------------

pub struct AwsDbSnapshotConverter {
    service: Arc<RdsService>,
}

impl AwsDbSnapshotConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDbSnapshotConverter {
    fn resource_type(&self) -> &str {
        "aws_db_snapshot"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_db_instance"]
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

impl AwsDbSnapshotConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::DbSnapshotTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_db_snapshot", e))?;

        let _ = attrs.get("tags_all");
        let _ = attrs.get("shared_accounts");
        let tags = extract_rds_tags(attrs);

        let arn = model.db_snapshot_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:snapshot:{}",
                region, ctx.default_account_id, model.db_snapshot_identifier
            )
        });
        let status = model.status.unwrap_or_else(|| "available".to_string());
        let snapshot_type = model.snapshot_type.unwrap_or_else(|| "manual".to_string());
        let engine = model.engine.unwrap_or_else(|| "mysql".to_string());

        let port = if model.port == 0 {
            None
        } else {
            Some(model.port as i32)
        };
        let iops = if model.iops == 0 {
            None
        } else {
            Some(model.iops as i32)
        };

        let view = DbSnapshotView {
            identifier: model.db_snapshot_identifier.clone(),
            db_instance_identifier: model.db_instance_identifier,
            engine,
            engine_version: model.engine_version,
            allocated_storage: model.allocated_storage as i32,
            status,
            port,
            availability_zone: model.availability_zone,
            vpc_id: model.vpc_id,
            instance_create_time: None,
            master_username: None,
            snapshot_type,
            iops,
            option_group_name: model.option_group_name,
            percent_progress: 100,
            source_region: model.source_region,
            source_db_snapshot_identifier: model.source_db_snapshot_identifier,
            storage_type: model.storage_type,
            tde_credential_arn: None,
            encrypted: model.encrypted,
            kms_key_id: model.kms_key_id,
            db_snapshot_arn: arn,
            timezone: None,
            db_instance_automated_backups_arn: None,
            snapshot_create_time: None,
            tags,
        };

        let mut state_view = minimal_rds_state_view();
        state_view
            .db_snapshots
            .insert(model.db_snapshot_identifier, view);
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
        for s in view.db_snapshots.values() {
            // Manual snapshots map to aws_db_snapshot; automated / copy live in their own converters.
            if s.snapshot_type == "automated" {
                continue;
            }
            let attrs = serde_json::json!({
                "id": s.identifier,
                "db_snapshot_identifier": s.identifier,
                "db_instance_identifier": s.db_instance_identifier,
                "db_snapshot_arn": s.db_snapshot_arn,
                "engine": s.engine,
                "engine_version": s.engine_version,
                "allocated_storage": s.allocated_storage,
                "availability_zone": s.availability_zone,
                "encrypted": s.encrypted,
                "iops": s.iops,
                "kms_key_id": s.kms_key_id,
                "option_group_name": s.option_group_name,
                "port": s.port,
                "snapshot_type": s.snapshot_type,
                "source_db_snapshot_identifier": s.source_db_snapshot_identifier,
                "source_region": s.source_region,
                "status": s.status,
                "storage_type": s.storage_type,
                "vpc_id": s.vpc_id,
                "tags": tags_to_map(&s.tags),
                "tags_all": tags_to_map(&s.tags),
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
// aws_db_snapshot_copy
// ---------------------------------------------------------------------------

pub struct AwsDbSnapshotCopyConverter {
    service: Arc<RdsService>,
}

impl AwsDbSnapshotCopyConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDbSnapshotCopyConverter {
    fn resource_type(&self) -> &str {
        "aws_db_snapshot_copy"
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

impl AwsDbSnapshotCopyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::DbSnapshotCopyTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_db_snapshot_copy", e))?;

        let _ = attrs.get("tags_all");
        let _ = attrs.get("shared_accounts");
        let _ = attrs.get("presigned_url");
        let tags = extract_rds_tags(attrs);

        let target_region = model
            .destination_region
            .clone()
            .unwrap_or_else(|| region.clone());

        let arn = model.db_snapshot_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:snapshot:{}",
                target_region, ctx.default_account_id, model.target_db_snapshot_identifier
            )
        });
        let snapshot_type = model.snapshot_type.unwrap_or_else(|| "manual".to_string());
        let engine = model.engine.unwrap_or_else(|| "mysql".to_string());

        let port = if model.port == 0 {
            None
        } else {
            Some(model.port as i32)
        };
        let iops = if model.iops == 0 {
            None
        } else {
            Some(model.iops as i32)
        };

        let view = DbSnapshotView {
            identifier: model.target_db_snapshot_identifier.clone(),
            db_instance_identifier: String::new(),
            engine,
            engine_version: model.engine_version,
            allocated_storage: model.allocated_storage as i32,
            status: "available".to_string(),
            port,
            availability_zone: None,
            vpc_id: model.vpc_id,
            instance_create_time: None,
            master_username: None,
            snapshot_type,
            iops,
            option_group_name: model.option_group_name,
            percent_progress: 100,
            source_region: model.source_region,
            source_db_snapshot_identifier: Some(model.source_db_snapshot_identifier),
            storage_type: model.storage_type,
            tde_credential_arn: None,
            encrypted: model.encrypted,
            kms_key_id: model.kms_key_id,
            db_snapshot_arn: arn,
            timezone: None,
            db_instance_automated_backups_arn: None,
            snapshot_create_time: None,
            tags,
        };

        let mut state_view = minimal_rds_state_view();
        state_view
            .db_snapshots
            .insert(model.target_db_snapshot_identifier, view);
        self.service
            .merge(&ctx.default_account_id, &target_region, state_view)
            .await?;

        Ok(ConversionResult {
            region: target_region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Snapshot copies are stored alongside primary snapshots; we cannot
        // reliably distinguish them on extract, so emit nothing here to avoid
        // doubling output. aws_db_snapshot picks them up via the regular path.
        let _ = ctx;
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_db_cluster_snapshot
// ---------------------------------------------------------------------------

pub struct AwsDbClusterSnapshotConverter {
    service: Arc<RdsService>,
}

impl AwsDbClusterSnapshotConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDbClusterSnapshotConverter {
    fn resource_type(&self) -> &str {
        "aws_db_cluster_snapshot"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_rds_cluster"]
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

impl AwsDbClusterSnapshotConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::DbClusterSnapshotTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_db_cluster_snapshot", e))?;

        let _ = attrs.get("tags_all");
        let _ = attrs.get("shared_accounts");
        let tags = extract_rds_tags(attrs);
        let availability_zones = extract_string_array(attrs, "availability_zones");

        let arn = model.db_cluster_snapshot_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:cluster-snapshot:{}",
                region, ctx.default_account_id, model.db_cluster_snapshot_identifier
            )
        });
        let status = model.status.unwrap_or_else(|| "available".to_string());
        let snapshot_type = model.snapshot_type.unwrap_or_else(|| "manual".to_string());
        let engine = model.engine.unwrap_or_else(|| "aurora-mysql".to_string());

        let port = if model.port == 0 {
            None
        } else {
            Some(model.port as i32)
        };

        let view = DbClusterSnapshotView {
            identifier: model.db_cluster_snapshot_identifier.clone(),
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
            source_db_cluster_snapshot_arn: model.source_db_cluster_snapshot_arn,
            availability_zones,
            snapshot_create_time: None,
            tags,
            storage_type: model.storage_type,
        };

        let mut state_view = minimal_rds_state_view();
        state_view
            .db_cluster_snapshots
            .insert(model.db_cluster_snapshot_identifier, view);
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
                "source_db_cluster_snapshot_arn": s.source_db_cluster_snapshot_arn,
                "status": s.status,
                "storage_encrypted": s.storage_encrypted,
                "storage_type": s.storage_type,
                "vpc_id": s.vpc_id,
                "tags": tags_to_map(&s.tags),
                "tags_all": tags_to_map(&s.tags),
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
// aws_rds_cluster_snapshot_copy
// ---------------------------------------------------------------------------

pub struct AwsRdsClusterSnapshotCopyConverter {
    service: Arc<RdsService>,
}

impl AwsRdsClusterSnapshotCopyConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRdsClusterSnapshotCopyConverter {
    fn resource_type(&self) -> &str {
        "aws_rds_cluster_snapshot_copy"
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

impl AwsRdsClusterSnapshotCopyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::RdsClusterSnapshotCopyTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_rds_cluster_snapshot_copy", e))?;

        let _ = attrs.get("tags_all");
        let _ = attrs.get("shared_accounts");
        let _ = attrs.get("presigned_url");
        let tags = extract_rds_tags(attrs);

        let target_region = model
            .destination_region
            .clone()
            .unwrap_or_else(|| region.clone());

        let arn = model.db_cluster_snapshot_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:cluster-snapshot:{}",
                target_region, ctx.default_account_id, model.target_db_cluster_snapshot_identifier
            )
        });
        let snapshot_type = model.snapshot_type.unwrap_or_else(|| "manual".to_string());
        let engine = model.engine.unwrap_or_else(|| "aurora-mysql".to_string());

        let view = DbClusterSnapshotView {
            identifier: model.target_db_cluster_snapshot_identifier.clone(),
            db_cluster_identifier: String::new(),
            engine,
            engine_version: model.engine_version,
            allocated_storage: model.allocated_storage as i32,
            status: "available".to_string(),
            port: None,
            vpc_id: model.vpc_id,
            cluster_create_time: None,
            master_username: None,
            snapshot_type,
            percent_progress: 100,
            storage_encrypted: model.storage_encrypted,
            kms_key_id: model.kms_key_id,
            db_cluster_snapshot_arn: arn,
            source_db_cluster_snapshot_arn: Some(format!(
                "arn:aws:rds:{}:{}:cluster-snapshot:{}",
                region, ctx.default_account_id, model.source_db_cluster_snapshot_identifier
            )),
            availability_zones: Vec::new(),
            snapshot_create_time: None,
            tags,
            storage_type: model.storage_type,
        };

        let mut state_view = minimal_rds_state_view();
        state_view
            .db_cluster_snapshots
            .insert(model.target_db_cluster_snapshot_identifier, view);
        self.service
            .merge(&ctx.default_account_id, &target_region, state_view)
            .await?;

        Ok(ConversionResult {
            region: target_region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Copies are written into db_cluster_snapshots and emitted by the
        // aws_db_cluster_snapshot converter.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_rds_cluster_endpoint
// ---------------------------------------------------------------------------

pub struct AwsRdsClusterEndpointConverter {
    service: Arc<RdsService>,
}

impl AwsRdsClusterEndpointConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRdsClusterEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_rds_cluster_endpoint"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_rds_cluster"]
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

impl AwsRdsClusterEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::RdsClusterEndpointTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_rds_cluster_endpoint", e))?;

        let _ = attrs.get("tags_all");
        let static_members = extract_string_array(attrs, "static_members");
        let excluded_members = extract_string_array(attrs, "excluded_members");

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:cluster-endpoint:{}",
                region, ctx.default_account_id, model.cluster_endpoint_identifier
            )
        });
        let endpoint = model.endpoint.unwrap_or_else(|| {
            format!(
                "{}.cluster-custom-{}.{}.rds.amazonaws.com",
                model.cluster_endpoint_identifier,
                &uuid::Uuid::new_v4().to_string()[..12],
                region
            )
        });

        let view = DbClusterEndpointView {
            db_cluster_endpoint_identifier: model.cluster_endpoint_identifier.clone(),
            db_cluster_identifier: model.cluster_identifier,
            db_cluster_endpoint_arn: arn,
            endpoint,
            status: "available".to_string(),
            endpoint_type: "CUSTOM".to_string(),
            custom_endpoint_type: Some(model.custom_endpoint_type),
            static_members,
            excluded_members,
        };

        let mut state_view = minimal_rds_state_view();
        state_view
            .db_cluster_endpoints
            .insert(model.cluster_endpoint_identifier, view);
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
        for e in view.db_cluster_endpoints.values() {
            let attrs = serde_json::json!({
                "id": e.db_cluster_endpoint_identifier,
                "cluster_endpoint_identifier": e.db_cluster_endpoint_identifier,
                "cluster_identifier": e.db_cluster_identifier,
                "arn": e.db_cluster_endpoint_arn,
                "endpoint": e.endpoint,
                "custom_endpoint_type": e.custom_endpoint_type,
                "static_members": e.static_members,
                "excluded_members": e.excluded_members,
            });
            results.push(ExtractedResource {
                name: e.db_cluster_endpoint_identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_rds_cluster_instance
// ---------------------------------------------------------------------------

pub struct AwsRdsClusterInstanceConverter {
    service: Arc<RdsService>,
}

impl AwsRdsClusterInstanceConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRdsClusterInstanceConverter {
    fn resource_type(&self) -> &str {
        "aws_rds_cluster_instance"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_rds_cluster", "aws_db_subnet_group"]
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

impl AwsRdsClusterInstanceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::RdsClusterInstanceTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_rds_cluster_instance", e))?;

        let _ = attrs.get("tags_all");
        let _ = attrs.get("apply_immediately");
        let _ = attrs.get("monitoring_role_arn");
        let _ = attrs.get("performance_insights_kms_key_id");
        let _ = attrs.get("preferred_maintenance_window");
        let _ = attrs.get("preferred_backup_window");
        let _ = attrs.get("identifier_prefix");
        let tags = extract_rds_tags(attrs);

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:db:{}",
                region, ctx.default_account_id, model.identifier
            )
        });
        let engine_version = model.engine_version.unwrap_or_default();
        let endpoint_address = Some(format!(
            "{}.{}.{}.rds.amazonaws.com",
            model.identifier,
            &uuid::Uuid::new_v4().to_string()[..8],
            region
        ));

        let port = attrs.get("port").and_then(|v| v.as_i64()).map(|p| p as i32);
        let monitoring_interval = attrs
            .get("monitoring_interval")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let _ = attrs.get("promotion_tier");

        let param_groups = model
            .db_parameter_group_name
            .clone()
            .into_iter()
            .collect::<Vec<_>>();

        let view = DbInstanceView {
            identifier: model.identifier.clone(),
            db_instance_class: model.instance_class,
            engine: model.engine,
            engine_version,
            status: "available".to_string(),
            master_username: None,
            db_name: None,
            endpoint_address,
            port,
            multi_az: false,
            storage_type: None,
            allocated_storage: 0,
            db_subnet_group_name: model.db_subnet_group_name,
            vpc_security_group_ids: Vec::new(),
            db_parameter_group_names: param_groups,
            availability_zone: model.availability_zone,
            publicly_accessible: model.publicly_accessible,
            auto_minor_version_upgrade: model.auto_minor_version_upgrade,
            backup_retention_period: 0,
            db_cluster_identifier: Some(model.cluster_identifier),
            arn,
            tags,
            instance_create_time: None,
            license_model: None,
            iops: None,
            deletion_protection: false,
            copy_tags_to_snapshot: model.copy_tags_to_snapshot,
            monitoring_interval,
            performance_insights_enabled: model.performance_insights_enabled,
            storage_encrypted: false,
            kms_key_id: model.kms_key_id,
            ca_certificate_identifier: model.ca_cert_identifier,
            secondary_availability_zone: None,
            blue_green_update: None,
            restore_to_point_in_time: None,
            s3_import: None,
        };

        let mut state_view = minimal_rds_state_view();
        state_view.db_instances.insert(model.identifier, view);
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
        for dbi in view.db_instances.values() {
            if dbi.db_cluster_identifier.is_none() {
                continue;
            }
            let attrs = serde_json::json!({
                "id": dbi.identifier,
                "identifier": dbi.identifier,
                "cluster_identifier": dbi.db_cluster_identifier,
                "engine": dbi.engine,
                "engine_version": dbi.engine_version,
                "instance_class": dbi.db_instance_class,
                "arn": dbi.arn,
                "availability_zone": dbi.availability_zone,
                "db_parameter_group_name": dbi.db_parameter_group_names.first(),
                "db_subnet_group_name": dbi.db_subnet_group_name,
                "endpoint": dbi.endpoint_address,
                "port": dbi.port,
                "publicly_accessible": dbi.publicly_accessible,
                "auto_minor_version_upgrade": dbi.auto_minor_version_upgrade,
                "performance_insights_enabled": dbi.performance_insights_enabled,
                "copy_tags_to_snapshot": dbi.copy_tags_to_snapshot,
                "monitoring_interval": dbi.monitoring_interval,
                "ca_cert_identifier": dbi.ca_certificate_identifier,
                "kms_key_id": dbi.kms_key_id,
                "storage_encrypted": dbi.storage_encrypted,
                "writer": false,
                "dbi_resource_id": dbi.identifier,
                "tags": tags_to_map(&dbi.tags),
                "tags_all": tags_to_map(&dbi.tags),
            });
            results.push(ExtractedResource {
                name: dbi.identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_rds_export_task
// ---------------------------------------------------------------------------

pub struct AwsRdsExportTaskConverter {
    service: Arc<RdsService>,
}

impl AwsRdsExportTaskConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRdsExportTaskConverter {
    fn resource_type(&self) -> &str {
        "aws_rds_export_task"
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

impl AwsRdsExportTaskConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::RdsExportTaskTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_rds_export_task", e))?;

        let export_only = extract_string_array(attrs, "export_only");
        let status = model.status.unwrap_or_else(|| "complete".to_string());

        let view = ExportTaskView {
            export_task_identifier: model.export_task_identifier.clone(),
            source_arn: model.source_arn,
            export_only,
            source_type: model.source_type,
            snapshot_time: model.snapshot_time,
            task_start_time: model.task_start_time,
            task_end_time: model.task_end_time,
            s3_bucket: model.s3_bucket_name,
            s3_prefix: model.s3_prefix,
            iam_role_arn: model.iam_role_arn,
            kms_key_id: model.kms_key_id,
            status,
            percent_progress: model.percent_progress as i32,
            total_extracted_data_in_gb: None,
            failure_cause: model.failure_cause,
            warning_message: model.warning_message,
        };

        let mut state_view = minimal_rds_state_view();
        state_view
            .export_tasks
            .insert(model.export_task_identifier, view);
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
        for t in view.export_tasks.values() {
            let attrs = serde_json::json!({
                "id": t.export_task_identifier,
                "export_task_identifier": t.export_task_identifier,
                "source_arn": t.source_arn,
                "source_type": t.source_type,
                "export_only": t.export_only,
                "s3_bucket_name": t.s3_bucket,
                "s3_prefix": t.s3_prefix,
                "iam_role_arn": t.iam_role_arn,
                "kms_key_id": t.kms_key_id,
                "snapshot_time": t.snapshot_time,
                "task_start_time": t.task_start_time,
                "task_end_time": t.task_end_time,
                "status": t.status,
                "percent_progress": t.percent_progress,
                "failure_cause": t.failure_cause,
                "warning_message": t.warning_message,
            });
            results.push(ExtractedResource {
                name: t.export_task_identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_rds_global_cluster
// ---------------------------------------------------------------------------

pub struct AwsRdsGlobalClusterConverter {
    service: Arc<RdsService>,
}

impl AwsRdsGlobalClusterConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRdsGlobalClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_rds_global_cluster"
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

impl AwsRdsGlobalClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::RdsGlobalClusterTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_rds_global_cluster", e))?;

        let _ = attrs.get("tags_all");
        let _ = attrs.get("force_destroy");
        let _ = attrs.get("engine_lifecycle_support");

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds::{}:global-cluster:{}",
                ctx.default_account_id, model.global_cluster_identifier
            )
        });
        let resource_id = model
            .global_cluster_resource_id
            .unwrap_or_else(|| format!("cluster-{}", uuid::Uuid::new_v4().simple()));

        let view = GlobalClusterView {
            global_cluster_identifier: model.global_cluster_identifier.clone(),
            global_cluster_resource_id: resource_id,
            global_cluster_arn: arn,
            status: "available".to_string(),
            engine: model.engine,
            engine_version: model.engine_version,
            database_name: model.database_name,
            storage_encrypted: model.storage_encrypted,
            deletion_protection: model.deletion_protection,
        };

        let mut state_view = minimal_rds_state_view();
        state_view
            .global_clusters
            .insert(model.global_cluster_identifier, view);
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
                "id": gc.global_cluster_identifier,
                "global_cluster_identifier": gc.global_cluster_identifier,
                "global_cluster_resource_id": gc.global_cluster_resource_id,
                "arn": gc.global_cluster_arn,
                "engine": gc.engine,
                "engine_version": gc.engine_version,
                "database_name": gc.database_name,
                "storage_encrypted": gc.storage_encrypted,
                "deletion_protection": gc.deletion_protection,
            });
            results.push(ExtractedResource {
                name: gc.global_cluster_identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_rds_shard_group
// ---------------------------------------------------------------------------

pub struct AwsRdsShardGroupConverter {
    service: Arc<RdsService>,
}

impl AwsRdsShardGroupConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRdsShardGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_rds_shard_group"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_rds_cluster"]
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

impl AwsRdsShardGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: rds_gen::RdsShardGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_rds_shard_group", e))?;

        let _ = attrs.get("tags_all");
        let _ = attrs.get("compute_redundancy");
        let tag_list = extract_rds_tags(attrs);

        // f64 fields are unsupported in spec; read raw from attrs.
        let max_acu = attrs.get("max_acu").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let min_acu = attrs.get("min_acu").and_then(|v| v.as_f64());

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rds:{}:{}:shard-group:{}",
                region, ctx.default_account_id, model.db_shard_group_identifier
            )
        });
        let endpoint = model.endpoint.unwrap_or_else(|| {
            format!(
                "{}.shardgrp-{}.{}.rds.amazonaws.com",
                model.db_shard_group_identifier,
                &uuid::Uuid::new_v4().to_string()[..12],
                region
            )
        });

        let view = DbShardGroupView {
            db_shard_group_identifier: model.db_shard_group_identifier.clone(),
            db_shard_group_resource_id: model.db_shard_group_resource_id,
            db_cluster_identifier: model.db_cluster_identifier,
            max_acu,
            min_acu,
            publicly_accessible: model.publicly_accessible,
            status: "available".to_string(),
            endpoint: Some(endpoint),
            db_shard_group_arn: Some(arn),
            tag_list,
        };

        let mut state_view = minimal_rds_state_view();
        state_view
            .db_shard_groups
            .insert(model.db_shard_group_identifier, view);
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
        for g in view.db_shard_groups.values() {
            let attrs = serde_json::json!({
                "id": g.db_shard_group_identifier,
                "db_shard_group_identifier": g.db_shard_group_identifier,
                "db_cluster_identifier": g.db_cluster_identifier,
                "db_shard_group_resource_id": g.db_shard_group_resource_id,
                "arn": g.db_shard_group_arn,
                "endpoint": g.endpoint,
                "max_acu": g.max_acu,
                "min_acu": g.min_acu,
                "publicly_accessible": g.publicly_accessible,
                "tags": tags_to_map(&g.tag_list),
                "tags_all": tags_to_map(&g.tag_list),
            });
            results.push(ExtractedResource {
                name: g.db_shard_group_identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_rds_instance_state
// ---------------------------------------------------------------------------
//
// Sub-resource: snapshots+mutates+restores the parent DbInstance.status.
// No generated model.
pub struct AwsRdsInstanceStateConverter {
    service: Arc<RdsService>,
}

impl AwsRdsInstanceStateConverter {
    pub fn new(service: Arc<RdsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRdsInstanceStateConverter {
    fn resource_type(&self) -> &str {
        "aws_rds_instance_state"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_db_instance"]
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

impl AwsRdsInstanceStateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let identifier = attrs
            .get("identifier")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ConversionError::MissingAttribute {
                resource_type: "aws_rds_instance_state".into(),
                attribute: "identifier".into(),
            })?
            .to_string();
        let state = attrs
            .get("state")
            .and_then(|v| v.as_str())
            .unwrap_or("available")
            .to_string();

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(dbi) = view.db_instances.get_mut(&identifier) {
            dbi.status = state;
        } else {
            warnings.push(format!(
                "db_instance '{identifier}' not found; instance_state ignored"
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Sub-resource that overlays DbInstance.status; do not emit on
        // extract to avoid pseudo-resources next to every aws_db_instance.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn minimal_rds_state_view() -> RdsStateView {
    RdsStateView {
        db_instances: HashMap::new(),
        db_clusters: HashMap::new(),
        db_subnet_groups: HashMap::new(),
        db_parameter_groups: HashMap::new(),
        db_cluster_parameter_groups: HashMap::new(),
        db_snapshots: HashMap::new(),
        db_cluster_snapshots: HashMap::new(),
        db_security_groups: HashMap::new(),
        event_subscriptions: HashMap::new(),
        option_groups: HashMap::new(),
        global_clusters: HashMap::new(),
        export_tasks: HashMap::new(),
        resource_tags: HashMap::new(),
        ..Default::default()
    }
}

/// Extract Terraform tags into RDS TagView format.
fn extract_rds_tags(attrs: &serde_json::Value) -> Vec<TagView> {
    let mut tags = Vec::new();
    // tags_all first (lower precedence)
    let mut tag_map: HashMap<String, String> = HashMap::new();
    if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
        for (k, v) in obj {
            if let Some(s) = v.as_str() {
                tag_map.insert(k.clone(), s.to_string());
            }
        }
    }
    if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
        for (k, v) in obj {
            if let Some(s) = v.as_str() {
                tag_map.insert(k.clone(), s.to_string());
            }
        }
    }
    for (k, v) in tag_map {
        tags.push(TagView { key: k, value: v });
    }
    tags
}

/// Convert TagView list to a JSON-friendly map.
fn tags_to_map(tags: &[TagView]) -> HashMap<String, String> {
    tags.iter()
        .map(|t| (t.key.clone(), t.value.clone()))
        .collect()
}

/// Extract a JSON value array from JSON attributes (preserves full objects).
fn extract_json_array(attrs: &serde_json::Value, key: &str) -> Vec<serde_json::Value> {
    attrs
        .get(key)
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
}

/// Extract a string array from JSON attributes.
fn extract_string_array(attrs: &serde_json::Value, key: &str) -> Vec<String> {
    attrs
        .get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Warning-only converters
// ---------------------------------------------------------------------------
//
// The following RDS resources have no addressable slot on `RdsStateView`:
// they either configure account-level metadata (certificates, reserved
// instances, custom engine versions), attach IAM roles to DbInstance /
// DbCluster (the `associated_roles` Vec is on the state types but not
// exposed on the views), enable feature streams without a corresponding
// view field, or describe cross-account/cross-region replication that
// the service crate doesn't model. The converters validate the input
// (so HCL typos still fail) and emit a single warning describing the
// state-layer gap. Extract is a no-op to avoid pseudo-resources next to
// every parent record.

macro_rules! rds_warning_only_converter {
    (
        struct_name = $struct_name:ident,
        resource_type = $resource_type:expr,
        model_type = $model_type:ident,
        warn_msg = $warn_msg:expr $(,)?
    ) => {
        pub struct $struct_name {
            #[allow(dead_code)]
            service: Arc<RdsService>,
        }

        impl $struct_name {
            pub fn new(service: Arc<RdsService>) -> Self {
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
                let _model: rds_gen::$model_type = serde_json::from_value(attrs.clone())
                    .map_err(|e| classify_deserialize_error($resource_type, e))?;
                Ok(ConversionResult {
                    region,
                    warnings: vec![format!("{}: {}", $resource_type, $warn_msg)],
                })
            }
        }
    };
}

rds_warning_only_converter! {
    struct_name = AwsDbInstanceAutomatedBackupsReplicationConverter,
    resource_type = "aws_db_instance_automated_backups_replication",
    model_type = DbInstanceAutomatedBackupsReplicationTfModel,
    warn_msg = "cross-region automated-backup replication is not modelled in winterbaume_rds; inject is a no-op",
}

rds_warning_only_converter! {
    struct_name = AwsDbInstanceRoleAssociationConverter,
    resource_type = "aws_db_instance_role_association",
    model_type = DbInstanceRoleAssociationTfModel,
    warn_msg = "DbInstanceView does not expose associated_roles; IAM role attachment is a no-op",
}

rds_warning_only_converter! {
    struct_name = AwsRdsCertificateConverter,
    resource_type = "aws_rds_certificate",
    model_type = RdsCertificateTfModel,
    warn_msg = "account-level default CA certificate is not modelled in winterbaume_rds; inject is a no-op",
}

rds_warning_only_converter! {
    struct_name = AwsRdsClusterActivityStreamConverter,
    resource_type = "aws_rds_cluster_activity_stream",
    model_type = RdsClusterActivityStreamTfModel,
    warn_msg = "DbClusterView does not expose activity-stream state; inject is a no-op",
}

rds_warning_only_converter! {
    struct_name = AwsRdsClusterRoleAssociationConverter,
    resource_type = "aws_rds_cluster_role_association",
    model_type = RdsClusterRoleAssociationTfModel,
    warn_msg = "DbClusterView does not expose associated_roles; IAM role attachment is a no-op",
}

rds_warning_only_converter! {
    struct_name = AwsRdsCustomDbEngineVersionConverter,
    resource_type = "aws_rds_custom_db_engine_version",
    model_type = RdsCustomDbEngineVersionTfModel,
    warn_msg = "custom DB engine versions are not modelled in winterbaume_rds; inject is a no-op",
}

rds_warning_only_converter! {
    struct_name = AwsRdsIntegrationConverter,
    resource_type = "aws_rds_integration",
    model_type = RdsIntegrationTfModel,
    warn_msg = "zero-ETL integrations are not modelled in winterbaume_rds; inject is a no-op",
}

rds_warning_only_converter! {
    struct_name = AwsRdsReservedInstanceConverter,
    resource_type = "aws_rds_reserved_instance",
    model_type = RdsReservedInstanceTfModel,
    warn_msg = "reserved instance purchases are not modelled in winterbaume_rds; inject is a no-op",
}
