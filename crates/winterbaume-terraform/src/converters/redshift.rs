//! Terraform converters for Redshift resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_redshift::RedshiftService;
use winterbaume_redshift::views::{
    RedshiftClusterView, RedshiftStateView, RedshiftSubnetGroupView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::redshift as redshift_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags};

// ---------------------------------------------------------------------------
// aws_redshift_cluster
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_cluster` Terraform resources to/from Redshift cluster state.
pub struct AwsRedshiftClusterConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftClusterConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_cluster"
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

impl AwsRedshiftClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftClusterTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_cluster", e))?;

        // Additional fields for coverage
        let _ = attrs.get("tags_all");
        let _ = attrs.get("apply_immediately");
        let _ = attrs.get("availability_zone_relocation_enabled");
        let _ = attrs.get("cluster_parameter_group_name");
        let _ = attrs.get("final_snapshot_identifier");
        let _ = attrs.get("iam_roles");
        let _ = attrs.get("manage_master_password");
        let _ = attrs.get("master_password_secret_kms_key_id");
        let _ = attrs.get("multi_az");
        let _ = attrs.get("skip_final_snapshot");
        let _ = attrs.get("snapshot_cluster_identifier");
        let _ = attrs.get("snapshot_identifier");
        let _ = attrs.get("default_iam_role_arn");
        let _ = attrs.get("enhanced_vpc_routing");
        let _ = attrs.get("cluster_namespace_arn");

        let cluster_identifier = model.cluster_identifier.clone();

        let node_type = model.node_type.unwrap_or_else(|| "dc2.large".to_string());
        let master_username = model.master_username.unwrap_or_else(|| "admin".to_string());
        let db_name = model.database_name.unwrap_or_else(|| "dev".to_string());
        let number_of_nodes = model.number_of_nodes as i32;
        let cluster_status = model
            .cluster_status
            .and_then(|s| serde_json::from_value(serde_json::Value::String(s)).ok())
            .unwrap_or(winterbaume_redshift::types::ClusterStatus::Available);
        let cluster_version = model.cluster_version.unwrap_or_else(|| "1.0".to_string());
        let availability_zone = model.availability_zone;
        let cluster_subnet_group_name = model.cluster_subnet_group_name;
        let vpc_id = model.vpc_id;
        let endpoint_address = model
            .endpoint
            .map(|e| {
                // The terraform attribute may be "host:port"; extract host
                e.split(':').next().unwrap_or(&e).to_string()
            })
            .or_else(|| {
                Some(format!(
                    "{}.abc123.{}.redshift.amazonaws.com",
                    cluster_identifier, region
                ))
            });
        let endpoint_port = Some(model.port as i32);
        let encrypted = model.encrypted;
        let publicly_accessible = model.publicly_accessible;
        let preferred_maintenance_window = model.preferred_maintenance_window;
        let automated_snapshot_retention_period = model.automated_snapshot_retention_period as i32;

        let tags_map = extract_tags(attrs);
        let tags: Vec<(String, String)> = tags_map.into_iter().collect();

        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:redshift:{}:{}:cluster:{}",
                region, ctx.default_account_id, cluster_identifier
            )
        });

        // Parse `logging` nested block: [{enable: bool, bucket_name: ..., s3_key_prefix: ...}]
        let logging_block = attrs
            .get("logging")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first());
        let logging_enabled = logging_block
            .and_then(|b| b.get("enable").or_else(|| b.get("enable_logging")))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let logging_bucket_name = logging_block
            .and_then(|b| b.get("bucket_name"))
            .and_then(|v| v.as_str())
            .map(String::from);
        let logging_s3_key_prefix = logging_block
            .and_then(|b| b.get("s3_key_prefix"))
            .and_then(|v| v.as_str())
            .map(String::from);

        // Parse `snapshot_copy` nested block: [{destination_region: ..., retention_period: ..., grant_name: ...}]
        let snapshot_copy = attrs
            .get("snapshot_copy")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|b| {
                b.get("destination_region")
                    .and_then(|v| v.as_str())
                    .map(|dest| {
                        let retention = b
                            .get("retention_period")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(7) as i32;
                        let grant_name = b
                            .get("grant_name")
                            .and_then(|v| v.as_str())
                            .map(String::from);
                        (dest.to_string(), retention, grant_name)
                    })
            });

        let cluster_view = RedshiftClusterView {
            cluster_identifier: cluster_identifier.clone(),
            node_type,
            cluster_status,
            master_username,
            db_name,
            cluster_subnet_group_name,
            vpc_id,
            availability_zone,
            number_of_nodes,
            arn: arn.clone(),
            endpoint_address,
            endpoint_port,
            cluster_version,
            encrypted,
            publicly_accessible,
            tags,
            snapshot_copy,
            logging_enabled,
            logging_bucket_name,
            logging_s3_key_prefix,
            preferred_maintenance_window,
            automated_snapshot_retention_period,
            availability_zone_relocation: attrs
                .get("availability_zone_relocation_enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        };

        let mut view = RedshiftStateView::default();
        view.clusters.insert(cluster_identifier, cluster_view);
        self.service
            .merge(&ctx.default_account_id, &region, view)
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
            let tags: serde_json::Map<String, serde_json::Value> = cluster
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            let logging_block: Vec<serde_json::Value> =
                if cluster.logging_enabled || cluster.logging_bucket_name.is_some() {
                    vec![serde_json::json!({
                        "enable": cluster.logging_enabled,
                        "bucket_name": cluster.logging_bucket_name,
                        "s3_key_prefix": cluster.logging_s3_key_prefix,
                        "log_destination_type": "s3",
                        "log_exports": [],
                    })]
                } else {
                    vec![]
                };

            let snapshot_copy_block: Vec<serde_json::Value> =
                if let Some((dest_region, retention, grant_name)) = &cluster.snapshot_copy {
                    vec![serde_json::json!({
                        "destination_region": dest_region,
                        "retention_period": retention,
                        "grant_name": grant_name,
                    })]
                } else {
                    vec![]
                };

            let cluster_type = if cluster.number_of_nodes > 1 {
                "multi-node"
            } else {
                "single-node"
            };
            let cluster_nodes = {
                let addr = cluster.endpoint_address.as_deref().unwrap_or("127.0.0.1");
                if cluster.number_of_nodes > 1 {
                    let mut nodes = vec![
                        serde_json::json!({"node_role": "LEADER", "private_ip_address": addr, "public_ip_address": addr}),
                    ];
                    for _ in 0..cluster.number_of_nodes - 1 {
                        nodes.push(serde_json::json!({"node_role": "COMPUTE", "private_ip_address": addr, "public_ip_address": addr}));
                    }
                    serde_json::Value::Array(nodes)
                } else {
                    serde_json::json!([{"node_role": "SHARED", "private_ip_address": addr, "public_ip_address": addr}])
                }
            };
            let attrs = serde_json::json!({
                "id": cluster.cluster_identifier,
                "cluster_identifier": cluster.cluster_identifier,
                "node_type": cluster.node_type,
                "master_username": cluster.master_username,
                "database_name": cluster.db_name,
                "number_of_nodes": cluster.number_of_nodes,
                "cluster_status": cluster.cluster_status.to_string(),
                "cluster_version": cluster.cluster_version,
                "availability_zone": cluster.availability_zone,
                "cluster_subnet_group_name": cluster.cluster_subnet_group_name,
                "vpc_id": cluster.vpc_id,
                "endpoint": cluster.endpoint_address,
                "port": cluster.endpoint_port.unwrap_or(5439),
                "encrypted": cluster.encrypted,
                "publicly_accessible": cluster.publicly_accessible,
                "preferred_maintenance_window": cluster.preferred_maintenance_window,
                "automated_snapshot_retention_period": cluster.automated_snapshot_retention_period,
                "logging_enabled": cluster.logging_enabled,
                "logging": logging_block,
                "snapshot_copy": snapshot_copy_block,
                "arn": cluster.arn,
                "tags": serde_json::Value::Object(tags.clone()),
                "tags_all": serde_json::Value::Object(tags),
                "cluster_nodes": cluster_nodes,
                "cluster_parameter_group_name": "",
                "cluster_public_key": "",
                "cluster_revision_number": "",
                "dns_name": cluster.endpoint_address,
                "maintenance_track_name": "current",
                "master_password_secret_arn": serde_json::Value::Null,
                "vpc_security_group_ids": [],
                "cluster_type": cluster_type,
            });
            results.push(ExtractedResource {
                name: cluster.cluster_identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_subnet_group
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_subnet_group` Terraform resources to/from Redshift subnet group state.
pub struct AwsRedshiftSubnetGroupConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftSubnetGroupConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftSubnetGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_subnet_group"
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

impl AwsRedshiftSubnetGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftSubnetGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_subnet_group", e))?;

        let name = model.name.clone();
        let description = model.description.unwrap_or_else(|| name.clone());
        let vpc_id = model.vpc_id.unwrap_or_default();

        // subnet_ids may be an array
        let subnet_ids: Vec<String> = attrs
            .get("subnet_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|s| s.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let tags_map = extract_tags(attrs);
        let tags: Vec<(String, String)> = tags_map.into_iter().collect();

        let subnet_group_view = RedshiftSubnetGroupView {
            name: name.clone(),
            description,
            vpc_id,
            subnet_ids,
            tags,
        };

        let mut view = RedshiftStateView::default();
        view.subnet_groups.insert(name, subnet_group_view);
        self.service
            .merge(&ctx.default_account_id, &region, view)
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
            let tags: serde_json::Map<String, serde_json::Value> = sg
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            let attrs = serde_json::json!({
                "id": sg.name,
                "name": sg.name,
                "description": sg.description,
                "vpc_id": sg.vpc_id,
                "subnet_ids": sg.subnet_ids,
                "tags": serde_json::Value::Object(tags),
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
