//! Terraform converter for DMS (Database Migration Service) resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_databasemigration::DatabaseMigrationService;
use winterbaume_databasemigration::views::{
    CertificateView, DmsStateView, EndpointView, EventSubscriptionView, ReplicationInstanceView,
    ReplicationSubnetGroupView, ReplicationTaskView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::dms as dms_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(attrs, &ctx.default_region);
        let model: dms_gen::EndpointTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_dms_endpoint", e))?;

        let endpoint_id = model.endpoint_id.clone();
        let endpoint_arn = model.endpoint_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:dms:{}:{}:endpoint:{}",
                region, ctx.default_account_id, endpoint_id
            )
        });
        // port is Option<i32>; not part of the strongly-typed model — read raw.
        let port = attrs.get("port").and_then(|v| v.as_i64()).map(|v| v as i32);
        let status = model.status.unwrap_or_else(|| "active".to_string());

        // Settings nested blobs are not part of the strongly-typed model.
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
            endpoint_identifier: endpoint_id.clone(),
            endpoint_type: model.endpoint_type,
            engine_name: model.engine_name,
            username: model.username,
            server_name: model.server_name,
            port,
            database_name: model.database_name,
            status,
            endpoint_arn,
            extra_connection_attributes: model.extra_connection_attributes,
            tags: model.tags,
            s3_settings,
            kafka_settings,
            kinesis_settings,
            mongodb_settings,
            elasticsearch_settings,
            redis_settings,
        };

        let mut state_view = DmsStateView::default();
        state_view.endpoints.insert(endpoint_id, ep_view);
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
        let region = extract_region(attrs, &ctx.default_region);
        let model: dms_gen::ReplicationInstanceTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_dms_replication_instance", e))?;

        let ri_id = model.replication_instance_id.clone();
        let ri_arn = model.replication_instance_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:dms:{}:{}:rep:{}",
                region, ctx.default_account_id, ri_id
            )
        });
        let allocated_storage = model.allocated_storage as i32;
        let status = model.status.unwrap_or_else(|| "available".to_string());
        // f64 timestamp not part of strongly-typed model — read raw.
        let instance_create_time = attrs
            .get("replication_instance_create_time")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let ri_view = ReplicationInstanceView {
            replication_instance_identifier: ri_id.clone(),
            replication_instance_class: model.replication_instance_class,
            allocated_storage,
            status,
            replication_instance_arn: ri_arn,
            availability_zone: model.availability_zone,
            publicly_accessible: model.publicly_accessible,
            multi_az: model.multi_az,
            engine_version: model.engine_version,
            instance_create_time,
            tags: model.tags,
        };

        let mut state_view = DmsStateView::default();
        state_view.replication_instances.insert(ri_id, ri_view);
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
        let region = extract_region(attrs, &ctx.default_region);
        let model: dms_gen::ReplicationTaskTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_dms_replication_task", e))?;

        let task_id = model.replication_task_id.clone();
        let task_arn = model.replication_task_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:dms:{}:{}:task:{}",
                region, ctx.default_account_id, task_id
            )
        });
        let status = model.status.unwrap_or_else(|| "ready".to_string());
        // f64 timestamps not part of strongly-typed model — read raw.
        let creation_date = attrs
            .get("replication_task_creation_date")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let start_date = attrs
            .get("replication_task_start_date")
            .and_then(|v| v.as_f64());

        let task_view = ReplicationTaskView {
            replication_task_identifier: task_id.clone(),
            source_endpoint_arn: model.source_endpoint_arn,
            target_endpoint_arn: model.target_endpoint_arn,
            replication_instance_arn: model.replication_instance_arn,
            migration_type: model.migration_type,
            table_mappings: model.table_mappings,
            replication_task_settings: model.replication_task_settings,
            status,
            replication_task_arn: task_arn,
            replication_task_creation_date: creation_date,
            replication_task_start_date: start_date,
            tags: model.tags,
        };

        let mut state_view = DmsStateView::default();
        state_view.replication_tasks.insert(task_id, task_view);
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

// ---------------------------------------------------------------------------
// aws_dms_certificate
// ---------------------------------------------------------------------------

pub struct AwsDmsCertificateConverter {
    service: Arc<DatabaseMigrationService>,
}

impl AwsDmsCertificateConverter {
    pub fn new(service: Arc<DatabaseMigrationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDmsCertificateConverter {
    fn resource_type(&self) -> &str {
        "aws_dms_certificate"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let attrs = &instance.attributes;
            let region = extract_region(attrs, &ctx.default_region);
            let model: dms_gen::CertificateTfModel = serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_dms_certificate", e))?;

            let arn = model.certificate_arn.unwrap_or_else(|| {
                format!(
                    "arn:aws:dms:{}:{}:cert:{}",
                    region, ctx.default_account_id, model.certificate_id
                )
            });

            let cert = CertificateView {
                certificate_identifier: model.certificate_id.clone(),
                certificate_arn: arn,
                certificate_pem: model.certificate_pem,
                certificate_wallet: model.certificate_wallet,
                kms_key_id: None,
                certificate_creation_date: 0.0,
            };

            let mut state_view = DmsStateView::default();
            state_view.certificates.insert(model.certificate_id, cert);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for c in view.certificates.values() {
                let attrs = serde_json::json!({
                    "id": c.certificate_identifier,
                    "certificate_id": c.certificate_identifier,
                    "certificate_arn": c.certificate_arn,
                    "certificate_pem": c.certificate_pem,
                    "certificate_wallet": c.certificate_wallet,
                });
                results.push(ExtractedResource {
                    name: c.certificate_identifier.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_dms_event_subscription
// ---------------------------------------------------------------------------

pub struct AwsDmsEventSubscriptionConverter {
    service: Arc<DatabaseMigrationService>,
}

impl AwsDmsEventSubscriptionConverter {
    pub fn new(service: Arc<DatabaseMigrationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDmsEventSubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_dms_event_subscription"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let attrs = &instance.attributes;
            let region = extract_region(attrs, &ctx.default_region);
            let model: dms_gen::EventSubscriptionTfModel = serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_dms_event_subscription", e))?;

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

            let sub = EventSubscriptionView {
                subscription_name: model.name.clone(),
                sns_topic_arn: model.sns_topic_arn,
                source_type: model.source_type,
                event_categories,
                source_ids,
                enabled: model.enabled,
                status: "active".to_string(),
                subscription_creation_time: chrono::Utc::now().to_rfc3339(),
                customer_aws_id: ctx.default_account_id.clone(),
            };

            let mut state_view = DmsStateView::default();
            state_view.event_subscriptions.insert(model.name, sub);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for s in view.event_subscriptions.values() {
                let attrs = serde_json::json!({
                    "id": s.subscription_name,
                    "name": s.subscription_name,
                    "sns_topic_arn": s.sns_topic_arn,
                    "source_type": s.source_type,
                    "event_categories": s.event_categories,
                    "source_ids": s.source_ids,
                    "enabled": s.enabled,
                });
                results.push(ExtractedResource {
                    name: s.subscription_name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_dms_replication_config — no state slot
// ---------------------------------------------------------------------------

pub struct AwsDmsReplicationConfigConverter {
    #[allow(dead_code)]
    service: Arc<DatabaseMigrationService>,
}

impl AwsDmsReplicationConfigConverter {
    pub fn new(service: Arc<DatabaseMigrationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDmsReplicationConfigConverter {
    fn resource_type(&self) -> &str {
        "aws_dms_replication_config"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: dms_gen::ReplicationConfigTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_dms_replication_config", e))?;
            let warn_msg = "no state slot in winterbaume_databasemigration for serverless \
                            replication configs; inject is a no-op"
                .to_string();
            eprintln!("warning: aws_dms_replication_config: {warn_msg}");
            Ok(ConversionResult {
                region,
                warnings: vec![format!("aws_dms_replication_config: {warn_msg}")],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_dms_replication_subnet_group
// ---------------------------------------------------------------------------

pub struct AwsDmsReplicationSubnetGroupConverter {
    service: Arc<DatabaseMigrationService>,
}

impl AwsDmsReplicationSubnetGroupConverter {
    pub fn new(service: Arc<DatabaseMigrationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDmsReplicationSubnetGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_dms_replication_subnet_group"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let attrs = &instance.attributes;
            let region = extract_region(attrs, &ctx.default_region);
            let model: dms_gen::ReplicationSubnetGroupTfModel =
                serde_json::from_value(attrs.clone()).map_err(|e| {
                    classify_deserialize_error("aws_dms_replication_subnet_group", e)
                })?;

            let subnet_ids: Vec<String> = attrs
                .get("subnet_ids")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();

            let g = ReplicationSubnetGroupView {
                replication_subnet_group_identifier: model.replication_subnet_group_id.clone(),
                replication_subnet_group_description: model.replication_subnet_group_description,
                vpc_id: model.vpc_id,
                subnet_ids,
                tags: std::collections::HashMap::new(),
            };

            let mut state_view = DmsStateView::default();
            state_view
                .replication_subnet_groups
                .insert(model.replication_subnet_group_id, g);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for g in view.replication_subnet_groups.values() {
                let attrs = serde_json::json!({
                    "id": g.replication_subnet_group_identifier,
                    "replication_subnet_group_id": g.replication_subnet_group_identifier,
                    "replication_subnet_group_description": g.replication_subnet_group_description,
                    "vpc_id": g.vpc_id,
                    "subnet_ids": g.subnet_ids,
                });
                results.push(ExtractedResource {
                    name: g.replication_subnet_group_identifier.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_dms_s3_endpoint (reuses endpoints state slot)
// ---------------------------------------------------------------------------

pub struct AwsDmsS3EndpointConverter {
    service: Arc<DatabaseMigrationService>,
}

impl AwsDmsS3EndpointConverter {
    pub fn new(service: Arc<DatabaseMigrationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDmsS3EndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_dms_s3_endpoint"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let attrs = &instance.attributes;
            let region = extract_region(attrs, &ctx.default_region);
            let model: dms_gen::S3EndpointTfModel = serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_dms_s3_endpoint", e))?;

            let endpoint_id = model.endpoint_id.clone();
            let endpoint_arn = model.endpoint_arn.unwrap_or_else(|| {
                format!(
                    "arn:aws:dms:{}:{}:endpoint:{}",
                    region, ctx.default_account_id, endpoint_id
                )
            });

            let s3_settings = serde_json::json!({
                "service_access_role_arn": model.service_access_role_arn,
                "bucket_name": model.bucket_name,
                "bucket_folder": model.bucket_folder,
            });

            let ep = EndpointView {
                endpoint_identifier: endpoint_id.clone(),
                endpoint_type: model.endpoint_type,
                engine_name: "s3".to_string(),
                username: None,
                server_name: None,
                port: None,
                database_name: None,
                status: "active".to_string(),
                endpoint_arn,
                extra_connection_attributes: None,
                tags: std::collections::HashMap::new(),
                s3_settings: Some(s3_settings),
                kafka_settings: None,
                kinesis_settings: None,
                mongodb_settings: None,
                elasticsearch_settings: None,
                redis_settings: None,
            };

            let mut state_view = DmsStateView::default();
            state_view.endpoints.insert(endpoint_id, ep);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for ep in view.endpoints.values() {
                if ep.engine_name != "s3" {
                    continue;
                }
                let s3 = ep.s3_settings.clone().unwrap_or(serde_json::json!({}));
                let attrs = serde_json::json!({
                    "id": ep.endpoint_identifier,
                    "endpoint_id": ep.endpoint_identifier,
                    "endpoint_type": ep.endpoint_type,
                    "endpoint_arn": ep.endpoint_arn,
                    "service_access_role_arn": s3.get("service_access_role_arn"),
                    "bucket_name": s3.get("bucket_name"),
                    "bucket_folder": s3.get("bucket_folder"),
                });
                results.push(ExtractedResource {
                    name: ep.endpoint_identifier.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}
