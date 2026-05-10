//! Terraform converter for Timestream InfluxDB resources.
//!
//! `DbInstanceTfModel` is generated from `specs/timestreaminfluxdb.toml`. The
//! ARN template, the `status = "AVAILABLE"` constant, the
//! `Option<i32>`/`Option<bool>` numeric/bool fields, and the
//! `Vec<String>` VPC arrays are wired up here from the raw
//! `instance.attributes`.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;
use winterbaume_timestreaminfluxdb::TimestreamInfluxDbService;
use winterbaume_timestreaminfluxdb::views::{DbInstanceView, TimestreamInfluxDbStateView};

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::timestreaminfluxdb as timestreaminfluxdb_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_timestreaminfluxdb_db_instance
// ---------------------------------------------------------------------------

/// Converts `aws_timestreaminfluxdb_db_instance` Terraform resources to/from Timestream InfluxDB state.
pub struct AwsTimestreaminfluxdbDbInstanceConverter {
    service: Arc<TimestreamInfluxDbService>,
}

impl AwsTimestreaminfluxdbDbInstanceConverter {
    pub fn new(service: Arc<TimestreamInfluxDbService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsTimestreaminfluxdbDbInstanceConverter {
    fn resource_type(&self) -> &str {
        "aws_timestreaminfluxdb_db_instance"
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

impl AwsTimestreaminfluxdbDbInstanceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: timestreaminfluxdb_gen::DbInstanceTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_timestreaminfluxdb_db_instance", e))?;

        let id = model
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:timestream-influxdb:{}:{}:db-instance/{}",
                region, ctx.default_account_id, id
            )
        });

        let db_instance_type = model
            .db_instance_type
            .unwrap_or_else(|| "db.influx.medium".to_string());

        // Numeric/bool fields with Option semantics on the view side stay as
        // raw reads so we preserve the "missing" distinction.
        let attrs = &instance.attributes;
        let allocated_storage = attrs
            .get("allocated_storage")
            .and_then(|v| v.as_i64())
            .unwrap_or(20) as i32;
        let publicly_accessible = attrs.get("publicly_accessible").and_then(|v| v.as_bool());
        let port = attrs.get("port").and_then(|v| v.as_i64()).map(|v| v as i32);

        let vpc_subnet_ids = extract_string_array(attrs, "vpc_subnet_ids");
        let vpc_security_group_ids = extract_string_array(attrs, "vpc_security_group_ids");

        let instance_view = DbInstanceView {
            id: id.clone(),
            name: model.name.clone(),
            arn,
            status: "AVAILABLE".to_string(),
            endpoint: model.endpoint,
            port,
            db_instance_type,
            db_storage_type: model.db_storage_type,
            allocated_storage,
            deployment_type: model.deployment_type,
            vpc_subnet_ids,
            vpc_security_group_ids,
            publicly_accessible,
            db_parameter_group_identifier: model.db_parameter_group_identifier,
            availability_zone: model.availability_zone,
            tags: model.tags,
        };

        let mut state_view = TimestreamInfluxDbStateView {
            db_instances: HashMap::new(),
            db_clusters: HashMap::new(),
            db_parameter_groups: HashMap::new(),
        };
        state_view.db_instances.insert(id, instance_view);
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
            let attrs = serde_json::json!({
                "id": inst.id,
                "name": inst.name,
                "arn": inst.arn,
                "endpoint": inst.endpoint,
                "port": inst.port,
                "db_instance_type": inst.db_instance_type,
                "db_storage_type": inst.db_storage_type,
                "allocated_storage": inst.allocated_storage,
                "deployment_type": inst.deployment_type,
                "vpc_subnet_ids": inst.vpc_subnet_ids,
                "vpc_security_group_ids": inst.vpc_security_group_ids,
                "publicly_accessible": inst.publicly_accessible,
                "db_parameter_group_identifier": inst.db_parameter_group_identifier,
                "availability_zone": inst.availability_zone,
                "tags": inst.tags,
                "tags_all": inst.tags,
                "username": null,
            });
            results.push(ExtractedResource {
                name: inst.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
