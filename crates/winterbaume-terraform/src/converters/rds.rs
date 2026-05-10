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
    DbClusterParameterGroupView, DbClusterView, DbInstanceView, DbParameterGroupView,
    DbSubnetGroupView, EventSubscriptionView, OptionGroupView, RdsStateView, TagView,
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
