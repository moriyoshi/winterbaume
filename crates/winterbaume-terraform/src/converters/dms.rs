//! Terraform converter for DMS (Database Migration Service) resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_databasemigration::DatabaseMigrationService;
use winterbaume_databasemigration::views::{
    DmsStateView, EndpointView, ReplicationInstanceView, ReplicationTaskView,
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
// aws_dms_endpoint
// ---------------------------------------------------------------------------

/// Converts `aws_dms_endpoint` Terraform resources to/from DMS state.
pub struct AwsDmsEndpointConverter {
    service: Arc<DatabaseMigrationService>,
}

impl AwsDmsEndpointConverter {
    pub fn new(service: Arc<DatabaseMigrationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDmsEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_dms_endpoint"
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

impl AwsDmsEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let endpoint_id = require_str(attrs, "endpoint_id", "aws_dms_endpoint")?;
        let endpoint_type = require_str(attrs, "endpoint_type", "aws_dms_endpoint")?;
        let engine_name = require_str(attrs, "engine_name", "aws_dms_endpoint")?;
        let region = extract_region(attrs, &ctx.default_region);
        let endpoint_arn = optional_str(attrs, "endpoint_arn").unwrap_or_else(|| {
            format!(
                "arn:aws:dms:{}:{}:endpoint:{}",
                region, ctx.default_account_id, endpoint_id
            )
        });
        let username = optional_str(attrs, "username");
        let server_name = optional_str(attrs, "server_name");
        let port = optional_i64(attrs, "port").map(|v| v as i32);
        let database_name = optional_str(attrs, "database_name");
        let status = optional_str(attrs, "status").unwrap_or_else(|| "active".to_string());
        let extra_connection_attributes = optional_str(attrs, "extra_connection_attributes");
        let _tags_all = attrs.get("tags_all");
        let _ssl_mode = optional_str(attrs, "ssl_mode");
        let _secrets_manager_access_role_arn =
            optional_str(attrs, "secrets_manager_access_role_arn");
        let _secrets_manager_arn = optional_str(attrs, "secrets_manager_arn");
        let _certificate_arn = optional_str(attrs, "certificate_arn");
        let tags = extract_tags(attrs);

        let s3_settings = attrs
            .get("s3_settings")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let kafka_settings = attrs
            .get("kafka_settings")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let kinesis_settings = attrs
            .get("kinesis_settings")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let mongodb_settings = attrs
            .get("mongodb_settings")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let elasticsearch_settings = attrs
            .get("elasticsearch_settings")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let redis_settings = attrs
            .get("redis_settings")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });

        let ep_view = EndpointView {
            endpoint_identifier: endpoint_id.to_string(),
            endpoint_type: endpoint_type.to_string(),
            engine_name: engine_name.to_string(),
            username,
            server_name,
            port,
            database_name,
            status,
            endpoint_arn,
            extra_connection_attributes,
            tags,
            s3_settings,
            kafka_settings,
            kinesis_settings,
            mongodb_settings,
            elasticsearch_settings,
            redis_settings,
        };

        let mut state_view = DmsStateView::default();
        state_view
            .endpoints
            .insert(endpoint_id.to_string(), ep_view);
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
        for ep in view.endpoints.values() {
            let attrs = serde_json::json!({
                "id": ep.endpoint_identifier,
                "endpoint_id": ep.endpoint_identifier,
                "endpoint_type": ep.endpoint_type,
                "engine_name": ep.engine_name,
                "username": ep.username,
                "server_name": ep.server_name,
                "port": ep.port,
                "database_name": ep.database_name,
                "status": ep.status,
                "endpoint_arn": ep.endpoint_arn,
                "extra_connection_attributes": ep.extra_connection_attributes,
                "tags": ep.tags,
                "tags_all": ep.tags,
                "ssl_mode": "none",
                "s3_settings": ep.s3_settings,
                "kafka_settings": ep.kafka_settings,
                "kinesis_settings": ep.kinesis_settings,
                "mongodb_settings": ep.mongodb_settings,
                "elasticsearch_settings": ep.elasticsearch_settings,
                "redis_settings": ep.redis_settings,
            });
            results.push(ExtractedResource {
                name: ep.endpoint_identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_dms_replication_instance
// ---------------------------------------------------------------------------

/// Converts `aws_dms_replication_instance` Terraform resources to/from DMS state.
pub struct AwsDmsReplicationInstanceConverter {
    service: Arc<DatabaseMigrationService>,
}

impl AwsDmsReplicationInstanceConverter {
    pub fn new(service: Arc<DatabaseMigrationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDmsReplicationInstanceConverter {
    fn resource_type(&self) -> &str {
        "aws_dms_replication_instance"
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

impl AwsDmsReplicationInstanceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let ri_id = require_str(
            attrs,
            "replication_instance_id",
            "aws_dms_replication_instance",
        )?;
        let ri_class = require_str(
            attrs,
            "replication_instance_class",
            "aws_dms_replication_instance",
        )?;
        let region = extract_region(attrs, &ctx.default_region);
        let _tags_all = attrs.get("tags_all");
        let _apply_immediately = optional_bool(attrs, "apply_immediately");
        let _auto_minor_version_upgrade = optional_bool(attrs, "auto_minor_version_upgrade");
        let _vpc_security_group_ids = attrs.get("vpc_security_group_ids");
        let ri_arn = optional_str(attrs, "replication_instance_arn").unwrap_or_else(|| {
            format!(
                "arn:aws:dms:{}:{}:rep:{}",
                region, ctx.default_account_id, ri_id
            )
        });
        let allocated_storage = optional_i64(attrs, "allocated_storage").unwrap_or(50) as i32;
        let status = optional_str(attrs, "status").unwrap_or_else(|| "available".to_string());
        let availability_zone = optional_str(attrs, "availability_zone");
        let publicly_accessible = optional_bool(attrs, "publicly_accessible").unwrap_or(false);
        let multi_az = optional_bool(attrs, "multi_az").unwrap_or(false);
        let engine_version = optional_str(attrs, "engine_version");
        let instance_create_time = attrs
            .get("replication_instance_create_time")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let tags = extract_tags(attrs);

        let ri_view = ReplicationInstanceView {
            replication_instance_identifier: ri_id.to_string(),
            replication_instance_class: ri_class.to_string(),
            allocated_storage,
            status,
            replication_instance_arn: ri_arn,
            availability_zone,
            publicly_accessible,
            multi_az,
            engine_version,
            instance_create_time,
            tags,
        };

        let mut state_view = DmsStateView::default();
        state_view
            .replication_instances
            .insert(ri_id.to_string(), ri_view);
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
        for ri in view.replication_instances.values() {
            let attrs = serde_json::json!({
                "id": ri.replication_instance_identifier,
                "replication_instance_id": ri.replication_instance_identifier,
                "replication_instance_class": ri.replication_instance_class,
                "allocated_storage": ri.allocated_storage,
                "status": ri.status,
                "replication_instance_arn": ri.replication_instance_arn,
                "availability_zone": ri.availability_zone,
                "publicly_accessible": ri.publicly_accessible,
                "multi_az": ri.multi_az,
                "engine_version": ri.engine_version,
                "replication_instance_create_time": ri.instance_create_time,
                "tags": ri.tags,
                "tags_all": ri.tags,
                "replication_instance_private_ips": [],
            });
            results.push(ExtractedResource {
                name: ri.replication_instance_identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_dms_replication_task
// ---------------------------------------------------------------------------

/// Converts `aws_dms_replication_task` Terraform resources to/from DMS state.
pub struct AwsDmsReplicationTaskConverter {
    service: Arc<DatabaseMigrationService>,
}

impl AwsDmsReplicationTaskConverter {
    pub fn new(service: Arc<DatabaseMigrationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDmsReplicationTaskConverter {
    fn resource_type(&self) -> &str {
        "aws_dms_replication_task"
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

impl AwsDmsReplicationTaskConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let task_id = require_str(attrs, "replication_task_id", "aws_dms_replication_task")?;
        let source_endpoint_arn =
            require_str(attrs, "source_endpoint_arn", "aws_dms_replication_task")?;
        let target_endpoint_arn =
            require_str(attrs, "target_endpoint_arn", "aws_dms_replication_task")?;
        let replication_instance_arn = require_str(
            attrs,
            "replication_instance_arn",
            "aws_dms_replication_task",
        )?;
        let migration_type = require_str(attrs, "migration_type", "aws_dms_replication_task")?;
        let table_mappings = require_str(attrs, "table_mappings", "aws_dms_replication_task")?;
        let region = extract_region(attrs, &ctx.default_region);
        let task_arn = optional_str(attrs, "replication_task_arn").unwrap_or_else(|| {
            format!(
                "arn:aws:dms:{}:{}:task:{}",
                region, ctx.default_account_id, task_id
            )
        });
        let replication_task_settings = optional_str(attrs, "replication_task_settings");
        let status = optional_str(attrs, "status").unwrap_or_else(|| "ready".to_string());
        let creation_date = attrs
            .get("replication_task_creation_date")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let start_date = attrs
            .get("replication_task_start_date")
            .and_then(|v| v.as_f64());
        let tags = extract_tags(attrs);

        let task_view = ReplicationTaskView {
            replication_task_identifier: task_id.to_string(),
            source_endpoint_arn: source_endpoint_arn.to_string(),
            target_endpoint_arn: target_endpoint_arn.to_string(),
            replication_instance_arn: replication_instance_arn.to_string(),
            migration_type: migration_type.to_string(),
            table_mappings: table_mappings.to_string(),
            replication_task_settings,
            status,
            replication_task_arn: task_arn,
            replication_task_creation_date: creation_date,
            replication_task_start_date: start_date,
            tags,
        };

        let mut state_view = DmsStateView::default();
        state_view
            .replication_tasks
            .insert(task_id.to_string(), task_view);
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
        for task in view.replication_tasks.values() {
            let attrs = serde_json::json!({
                "id": task.replication_task_identifier,
                "replication_task_id": task.replication_task_identifier,
                "source_endpoint_arn": task.source_endpoint_arn,
                "target_endpoint_arn": task.target_endpoint_arn,
                "replication_instance_arn": task.replication_instance_arn,
                "migration_type": task.migration_type,
                "table_mappings": task.table_mappings,
                "replication_task_settings": task.replication_task_settings,
                "status": task.status,
                "replication_task_arn": task.replication_task_arn,
                "replication_task_creation_date": task.replication_task_creation_date,
                "replication_task_start_date": task.replication_task_start_date,
                "tags": task.tags,
            });
            results.push(ExtractedResource {
                name: task.replication_task_identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
