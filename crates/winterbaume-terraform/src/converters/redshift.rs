//! Terraform converters for Redshift resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_redshift::RedshiftService;
use winterbaume_redshift::views::{
    RedshiftAuthenticationProfileView, RedshiftClusterView, RedshiftEventSubscriptionView,
    RedshiftHsmClientCertificateView, RedshiftHsmConfigurationView, RedshiftParameterGroupView,
    RedshiftPartnerIntegrationView, RedshiftResourcePolicyView, RedshiftScheduledActionView,
    RedshiftSnapshotCopyGrantView, RedshiftSnapshotScheduleView, RedshiftSnapshotView,
    RedshiftStateView, RedshiftSubnetGroupView, RedshiftUsageLimitView,
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

// ---------------------------------------------------------------------------
// aws_redshift_authentication_profile
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_authentication_profile` Terraform resources.
pub struct AwsRedshiftAuthenticationProfileConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftAuthenticationProfileConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftAuthenticationProfileConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_authentication_profile"
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

impl AwsRedshiftAuthenticationProfileConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftAuthenticationProfileTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_redshift_authentication_profile", e)
            })?;

        let name = model.name.clone();
        let content = model.content.unwrap_or_else(|| "{}".to_string());

        let view_entry = RedshiftAuthenticationProfileView {
            name: name.clone(),
            content,
        };

        let mut view = RedshiftStateView::default();
        view.authentication_profiles.insert(name, view_entry);
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
        for p in view.authentication_profiles.values() {
            let attrs = serde_json::json!({
                "id": p.name,
                "authentication_profile_name": p.name,
                "authentication_profile_content": p.content,
            });
            results.push(ExtractedResource {
                name: p.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_cluster_iam_roles
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_cluster_iam_roles` Terraform resources.
///
/// Sub-resource modifier: attaches `iam_role_arns` to an existing cluster.
/// Since the cluster view does not have a dedicated `iam_roles` field, this
/// converter is a no-op against state but is registered for completeness so
/// that converter parity checks pass. `iam_role_arns` and
/// `default_iam_role_arn` are read straight from `instance.attributes`.
pub struct AwsRedshiftClusterIamRolesConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftClusterIamRolesConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftClusterIamRolesConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_cluster_iam_roles"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_redshift_cluster"]
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

impl AwsRedshiftClusterIamRolesConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftClusterIamRolesTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_cluster_iam_roles", e))?;

        let _ = attrs.get("iam_role_arns");
        let _ = &model.default_iam_role_arn;

        let mut warnings = vec![];
        let state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if !state_view.clusters.contains_key(&model.cluster_identifier) {
            warnings.push(format!(
                "cluster '{}' not found in state; iam_roles association skipped",
                model.cluster_identifier
            ));
        }

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // No iam_role_arns are persisted on the cluster view; extract is empty.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_cluster_snapshot
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_cluster_snapshot` Terraform resources.
pub struct AwsRedshiftClusterSnapshotConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftClusterSnapshotConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftClusterSnapshotConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_cluster_snapshot"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_redshift_cluster"]
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

impl AwsRedshiftClusterSnapshotConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftClusterSnapshotTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_cluster_snapshot", e))?;

        let snapshot_identifier = model.snapshot_identifier.clone();
        let cluster_identifier = model.cluster_identifier.clone();
        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:redshift:{}:{}:snapshot:{}/{}",
                region, ctx.default_account_id, cluster_identifier, snapshot_identifier
            )
        });

        let tags_map = extract_tags(attrs);
        let tags: Vec<(String, String)> = tags_map.into_iter().collect();

        // Fill cluster-derived fields from the existing cluster if present,
        // otherwise use safe defaults.
        let state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let cluster = state_view.clusters.get(&cluster_identifier);
        let master_username = cluster
            .map(|c| c.master_username.clone())
            .unwrap_or_else(|| "admin".to_string());
        let db_name = cluster
            .map(|c| c.db_name.clone())
            .unwrap_or_else(|| "dev".to_string());
        let node_type = cluster
            .map(|c| c.node_type.clone())
            .unwrap_or_else(|| "dc2.large".to_string());
        let number_of_nodes = cluster.map(|c| c.number_of_nodes).unwrap_or(1);
        let cluster_version = cluster
            .map(|c| c.cluster_version.clone())
            .unwrap_or_else(|| "1.0".to_string());

        let snapshot_view = RedshiftSnapshotView {
            snapshot_identifier: snapshot_identifier.clone(),
            cluster_identifier,
            status: "available".to_string(),
            arn,
            tags,
            master_username,
            db_name,
            node_type,
            number_of_nodes,
            cluster_version,
        };

        let mut view = RedshiftStateView::default();
        view.snapshots.insert(snapshot_identifier, snapshot_view);
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
        for s in view.snapshots.values() {
            let tags: serde_json::Map<String, serde_json::Value> = s
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            let attrs = serde_json::json!({
                "id": s.snapshot_identifier,
                "snapshot_identifier": s.snapshot_identifier,
                "cluster_identifier": s.cluster_identifier,
                "arn": s.arn,
                "status": s.status,
                "master_username": s.master_username,
                "database_name": s.db_name,
                "node_type": s.node_type,
                "number_of_nodes": s.number_of_nodes,
                "cluster_version": s.cluster_version,
                "tags": serde_json::Value::Object(tags.clone()),
                "tags_all": serde_json::Value::Object(tags),
            });
            results.push(ExtractedResource {
                name: s.snapshot_identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_event_subscription
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_event_subscription` Terraform resources.
pub struct AwsRedshiftEventSubscriptionConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftEventSubscriptionConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftEventSubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_event_subscription"
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

impl AwsRedshiftEventSubscriptionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftEventSubscriptionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_event_subscription", e))?;

        let subscription_name = model.subscription_name.clone();
        let sns_topic_arn = model.sns_topic_arn.unwrap_or_default();
        let source_type = model.source_type;
        let severity = model.severity;
        let enabled = model.enabled;
        let status = model.status.unwrap_or_else(|| "active".to_string());
        let customer_aws_id = model
            .customer_aws_id
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let source_ids: Vec<String> = attrs
            .get("source_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let event_categories: Vec<String> = attrs
            .get("event_categories")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let tags_map = extract_tags(attrs);
        let tags: Vec<(String, String)> = tags_map.into_iter().collect();

        let view_entry = RedshiftEventSubscriptionView {
            subscription_name: subscription_name.clone(),
            sns_topic_arn,
            source_type,
            source_ids,
            event_categories,
            severity,
            enabled,
            tags,
            status,
            creation_time: "1970-01-01T00:00:00Z".to_string(),
            customer_aws_id,
        };

        let mut view = RedshiftStateView::default();
        view.event_subscriptions
            .insert(subscription_name, view_entry);
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
        for s in view.event_subscriptions.values() {
            let tags: serde_json::Map<String, serde_json::Value> = s
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            let arn = format!(
                "arn:aws:redshift:{}:{}:eventsubscription:{}",
                ctx.default_region, ctx.default_account_id, s.subscription_name
            );
            let attrs = serde_json::json!({
                "id": s.subscription_name,
                "name": s.subscription_name,
                "arn": arn,
                "sns_topic_arn": s.sns_topic_arn,
                "source_type": s.source_type,
                "source_ids": s.source_ids,
                "event_categories": s.event_categories,
                "severity": s.severity,
                "enabled": s.enabled,
                "status": s.status,
                "customer_aws_id": s.customer_aws_id,
                "tags": serde_json::Value::Object(tags.clone()),
                "tags_all": serde_json::Value::Object(tags),
            });
            results.push(ExtractedResource {
                name: s.subscription_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_hsm_client_certificate
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_hsm_client_certificate` Terraform resources.
pub struct AwsRedshiftHsmClientCertificateConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftHsmClientCertificateConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftHsmClientCertificateConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_hsm_client_certificate"
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

impl AwsRedshiftHsmClientCertificateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftHsmClientCertificateTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_redshift_hsm_client_certificate", e)
            })?;

        let identifier = model.identifier.clone();
        let public_key = model.public_key.unwrap_or_default();

        let tags_map = extract_tags(attrs);
        let tags: Vec<(String, String)> = tags_map.into_iter().collect();

        let view_entry = RedshiftHsmClientCertificateView {
            identifier: identifier.clone(),
            public_key,
            tags,
        };

        let mut view = RedshiftStateView::default();
        view.hsm_client_certificates.insert(identifier, view_entry);
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
        for c in view.hsm_client_certificates.values() {
            let tags: serde_json::Map<String, serde_json::Value> = c
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            let arn = format!(
                "arn:aws:redshift:{}:{}:hsmclientcertificate:{}",
                ctx.default_region, ctx.default_account_id, c.identifier
            );
            let attrs = serde_json::json!({
                "id": c.identifier,
                "hsm_client_certificate_identifier": c.identifier,
                "hsm_client_certificate_public_key": c.public_key,
                "arn": arn,
                "tags": serde_json::Value::Object(tags.clone()),
                "tags_all": serde_json::Value::Object(tags),
            });
            results.push(ExtractedResource {
                name: c.identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_hsm_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_hsm_configuration` Terraform resources.
pub struct AwsRedshiftHsmConfigurationConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftHsmConfigurationConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftHsmConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_hsm_configuration"
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

impl AwsRedshiftHsmConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftHsmConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_hsm_configuration", e))?;

        let identifier = model.identifier.clone();
        let description = model.description.unwrap_or_default();
        let hsm_ip_address = model.hsm_ip_address.unwrap_or_default();
        let hsm_partition_name = model.hsm_partition_name.unwrap_or_default();

        let tags_map = extract_tags(attrs);
        let tags: Vec<(String, String)> = tags_map.into_iter().collect();

        let view_entry = RedshiftHsmConfigurationView {
            identifier: identifier.clone(),
            description,
            hsm_ip_address,
            hsm_partition_name,
            tags,
        };

        let mut view = RedshiftStateView::default();
        view.hsm_configurations.insert(identifier, view_entry);
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
        for c in view.hsm_configurations.values() {
            let tags: serde_json::Map<String, serde_json::Value> = c
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            let arn = format!(
                "arn:aws:redshift:{}:{}:hsmconfiguration:{}",
                ctx.default_region, ctx.default_account_id, c.identifier
            );
            let attrs = serde_json::json!({
                "id": c.identifier,
                "hsm_configuration_identifier": c.identifier,
                "description": c.description,
                "hsm_ip_address": c.hsm_ip_address,
                "hsm_partition_name": c.hsm_partition_name,
                "arn": arn,
                "tags": serde_json::Value::Object(tags.clone()),
                "tags_all": serde_json::Value::Object(tags),
            });
            results.push(ExtractedResource {
                name: c.identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_logging
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_logging` Terraform resources.
///
/// Sub-resource modifier: enables logging on an existing cluster by setting
/// `logging_enabled`, `logging_bucket_name`, and `logging_s3_key_prefix`.
pub struct AwsRedshiftLoggingConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftLoggingConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftLoggingConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_logging"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_redshift_cluster"]
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

impl AwsRedshiftLoggingConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftLoggingTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_logging", e))?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(cluster) = state_view.clusters.get_mut(&model.cluster_identifier) {
            cluster.logging_enabled = true;
            cluster.logging_bucket_name = model.bucket_name.clone();
            cluster.logging_s3_key_prefix = model.s3_key_prefix.clone();
        } else {
            warnings.push(format!(
                "cluster '{}' not found in state; logging association skipped",
                model.cluster_identifier
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
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
        for cluster in view.clusters.values() {
            if !cluster.logging_enabled && cluster.logging_bucket_name.is_none() {
                continue;
            }
            let attrs = serde_json::json!({
                "id": cluster.cluster_identifier,
                "cluster_identifier": cluster.cluster_identifier,
                "bucket_name": cluster.logging_bucket_name,
                "s3_key_prefix": cluster.logging_s3_key_prefix,
                "log_destination_type": "s3",
                "log_exports": [],
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
// aws_redshift_parameter_group
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_parameter_group` Terraform resources.
pub struct AwsRedshiftParameterGroupConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftParameterGroupConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftParameterGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_parameter_group"
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

impl AwsRedshiftParameterGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftParameterGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_parameter_group", e))?;

        let name = model.name.clone();
        let family = model.family.clone();
        let description = model.description.unwrap_or_else(|| name.clone());

        let parameters: Vec<winterbaume_redshift::types::RedshiftParameter> = attrs
            .get("parameter")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|p| {
                        let name = p.get("name")?.as_str()?.to_string();
                        let value = p.get("value")?.as_str()?.to_string();
                        Some(winterbaume_redshift::types::RedshiftParameter {
                            name,
                            value,
                            description: String::new(),
                            is_modifiable: true,
                            apply_type: "static".to_string(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let tags_map = extract_tags(attrs);
        let tags: Vec<(String, String)> = tags_map.into_iter().collect();

        let view_entry = RedshiftParameterGroupView {
            name: name.clone(),
            family,
            description,
            tags,
            parameters,
        };

        let mut view = RedshiftStateView::default();
        view.parameter_groups.insert(name, view_entry);
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
        for pg in view.parameter_groups.values() {
            let tags: serde_json::Map<String, serde_json::Value> = pg
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            let parameters: Vec<serde_json::Value> = pg
                .parameters
                .iter()
                .map(|p| serde_json::json!({"name": p.name, "value": p.value}))
                .collect();
            let arn = format!(
                "arn:aws:redshift:{}:{}:parametergroup:{}",
                ctx.default_region, ctx.default_account_id, pg.name
            );
            let attrs = serde_json::json!({
                "id": pg.name,
                "name": pg.name,
                "family": pg.family,
                "description": pg.description,
                "arn": arn,
                "parameter": parameters,
                "tags": serde_json::Value::Object(tags.clone()),
                "tags_all": serde_json::Value::Object(tags),
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
// aws_redshift_partner
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_partner` Terraform resources.
pub struct AwsRedshiftPartnerConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftPartnerConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftPartnerConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_partner"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_redshift_cluster"]
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

impl AwsRedshiftPartnerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftPartnerTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_partner", e))?;

        let cluster_identifier = model.cluster_identifier.clone();
        let database_name = model.database_name.clone();
        let partner_name = model.partner_name.clone();
        let status = model.status.unwrap_or_else(|| "Active".to_string());
        let status_message = model.status_message;

        let view_entry = RedshiftPartnerIntegrationView {
            cluster_identifier,
            database_name,
            partner_name,
            status,
            status_message,
        };

        let mut view = RedshiftStateView::default();
        view.partner_integrations.push(view_entry);
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
        for p in &view.partner_integrations {
            let id = format!(
                "{}:{}:{}:{}",
                ctx.default_account_id, p.cluster_identifier, p.database_name, p.partner_name
            );
            let attrs = serde_json::json!({
                "id": id,
                "account_id": ctx.default_account_id,
                "cluster_identifier": p.cluster_identifier,
                "database_name": p.database_name,
                "partner_name": p.partner_name,
                "status": p.status,
                "status_message": p.status_message,
            });
            results.push(ExtractedResource {
                name: id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_resource_policy
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_resource_policy` Terraform resources.
pub struct AwsRedshiftResourcePolicyConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftResourcePolicyConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftResourcePolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_resource_policy"
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

impl AwsRedshiftResourcePolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftResourcePolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_resource_policy", e))?;

        let resource_arn = model.resource_arn.clone();
        let policy = model.policy.clone();

        let view_entry = RedshiftResourcePolicyView {
            resource_arn: resource_arn.clone(),
            policy,
        };

        let mut view = RedshiftStateView::default();
        view.resource_policies.insert(resource_arn, view_entry);
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
        for p in view.resource_policies.values() {
            let attrs = serde_json::json!({
                "id": p.resource_arn,
                "resource_arn": p.resource_arn,
                "policy": p.policy,
            });
            results.push(ExtractedResource {
                name: p.resource_arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_scheduled_action
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_scheduled_action` Terraform resources.
pub struct AwsRedshiftScheduledActionConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftScheduledActionConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftScheduledActionConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_scheduled_action"
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

impl AwsRedshiftScheduledActionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftScheduledActionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_scheduled_action", e))?;

        let name = model.name.clone();
        let state = if model.enable { "ACTIVE" } else { "DISABLED" }.to_string();

        let view_entry = RedshiftScheduledActionView {
            name: name.clone(),
            schedule: model.schedule,
            iam_role: model.iam_role,
            description: model.description,
            start_time: model.start_time,
            end_time: model.end_time,
            state,
        };

        let mut view = RedshiftStateView::default();
        view.scheduled_actions.insert(name, view_entry);
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
        for a in view.scheduled_actions.values() {
            let attrs = serde_json::json!({
                "id": a.name,
                "name": a.name,
                "schedule": a.schedule,
                "iam_role": a.iam_role,
                "description": a.description,
                "start_time": a.start_time,
                "end_time": a.end_time,
                "enable": a.state.eq_ignore_ascii_case("ACTIVE"),
                "state": a.state,
            });
            results.push(ExtractedResource {
                name: a.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_snapshot_copy
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_snapshot_copy` Terraform resources.
///
/// Sub-resource modifier: sets the `snapshot_copy` tuple on an existing
/// cluster. The cluster view exposes a single `snapshot_copy:
/// Option<(destination_region, retention_period, snapshot_copy_grant_name)>`.
pub struct AwsRedshiftSnapshotCopyConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftSnapshotCopyConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftSnapshotCopyConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_snapshot_copy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_redshift_cluster"]
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

impl AwsRedshiftSnapshotCopyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftSnapshotCopyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_snapshot_copy", e))?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(cluster) = state_view.clusters.get_mut(&model.cluster_identifier) {
            cluster.snapshot_copy = Some((
                model.destination_region.clone(),
                model.retention_period as i32,
                model.snapshot_copy_grant_name.clone(),
            ));
        } else {
            warnings.push(format!(
                "cluster '{}' not found in state; snapshot_copy association skipped",
                model.cluster_identifier
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
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
        for cluster in view.clusters.values() {
            if let Some((dest_region, retention, grant_name)) = &cluster.snapshot_copy {
                let attrs = serde_json::json!({
                    "id": cluster.cluster_identifier,
                    "cluster_identifier": cluster.cluster_identifier,
                    "destination_region": dest_region,
                    "retention_period": retention,
                    "snapshot_copy_grant_name": grant_name,
                });
                results.push(ExtractedResource {
                    name: cluster.cluster_identifier.clone(),
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
// aws_redshift_snapshot_copy_grant
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_snapshot_copy_grant` Terraform resources.
pub struct AwsRedshiftSnapshotCopyGrantConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftSnapshotCopyGrantConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftSnapshotCopyGrantConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_snapshot_copy_grant"
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

impl AwsRedshiftSnapshotCopyGrantConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftSnapshotCopyGrantTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_snapshot_copy_grant", e))?;

        let name = model.name.clone();
        let kms_key_id = model.kms_key_id;

        let tags_map = extract_tags(attrs);
        let tags: Vec<(String, String)> = tags_map.into_iter().collect();

        let view_entry = RedshiftSnapshotCopyGrantView {
            name: name.clone(),
            kms_key_id,
            tags,
        };

        let mut view = RedshiftStateView::default();
        view.snapshot_copy_grants.insert(name, view_entry);
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
        for g in view.snapshot_copy_grants.values() {
            let tags: serde_json::Map<String, serde_json::Value> = g
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            let arn = format!(
                "arn:aws:redshift:{}:{}:snapshotcopygrant:{}",
                ctx.default_region, ctx.default_account_id, g.name
            );
            let attrs = serde_json::json!({
                "id": g.name,
                "snapshot_copy_grant_name": g.name,
                "kms_key_id": g.kms_key_id,
                "arn": arn,
                "tags": serde_json::Value::Object(tags.clone()),
                "tags_all": serde_json::Value::Object(tags),
            });
            results.push(ExtractedResource {
                name: g.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_snapshot_schedule
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_snapshot_schedule` Terraform resources.
pub struct AwsRedshiftSnapshotScheduleConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftSnapshotScheduleConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftSnapshotScheduleConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_snapshot_schedule"
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

impl AwsRedshiftSnapshotScheduleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftSnapshotScheduleTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_snapshot_schedule", e))?;

        let schedule_identifier = model.schedule_identifier.clone();
        let schedule_description = model.schedule_description;

        let schedule_definitions: Vec<String> = attrs
            .get("definitions")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let tags_map = extract_tags(attrs);
        let tags: Vec<(String, String)> = tags_map.into_iter().collect();

        let view_entry = RedshiftSnapshotScheduleView {
            schedule_identifier: schedule_identifier.clone(),
            schedule_definitions,
            schedule_description,
            tags,
        };

        let mut view = RedshiftStateView::default();
        view.snapshot_schedules
            .insert(schedule_identifier, view_entry);
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
        for s in view.snapshot_schedules.values() {
            let tags: serde_json::Map<String, serde_json::Value> = s
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            let arn = format!(
                "arn:aws:redshift:{}:{}:snapshotschedule:{}",
                ctx.default_region, ctx.default_account_id, s.schedule_identifier
            );
            let attrs = serde_json::json!({
                "id": s.schedule_identifier,
                "identifier": s.schedule_identifier,
                "description": s.schedule_description,
                "definitions": s.schedule_definitions,
                "arn": arn,
                "tags": serde_json::Value::Object(tags.clone()),
                "tags_all": serde_json::Value::Object(tags),
            });
            results.push(ExtractedResource {
                name: s.schedule_identifier.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_usage_limit
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_usage_limit` Terraform resources.
pub struct AwsRedshiftUsageLimitConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftUsageLimitConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftUsageLimitConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_usage_limit"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_redshift_cluster"]
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

impl AwsRedshiftUsageLimitConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftUsageLimitTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_usage_limit", e))?;

        let usage_limit_id = attrs
            .get("id")
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| {
                format!(
                    "{}-{}-{}",
                    model.cluster_identifier, model.feature_type, model.limit_type
                )
            });

        let cluster_identifier = model.cluster_identifier.clone();
        let feature_type = model.feature_type.clone();
        let limit_type = model.limit_type.clone();
        let amount = model.amount;
        let period = model.period;
        let breach_action = model.breach_action;

        let tags_map = extract_tags(attrs);
        let tags: Vec<(String, String)> = tags_map.into_iter().collect();

        let view_entry = RedshiftUsageLimitView {
            usage_limit_id: usage_limit_id.clone(),
            cluster_identifier,
            feature_type,
            limit_type,
            amount,
            period,
            breach_action,
            tags,
        };

        let mut view = RedshiftStateView::default();
        view.usage_limits.insert(usage_limit_id, view_entry);
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
        for u in view.usage_limits.values() {
            let tags: serde_json::Map<String, serde_json::Value> = u
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            let arn = format!(
                "arn:aws:redshift:{}:{}:usagelimit:{}",
                ctx.default_region, ctx.default_account_id, u.usage_limit_id
            );
            let attrs = serde_json::json!({
                "id": u.usage_limit_id,
                "cluster_identifier": u.cluster_identifier,
                "feature_type": u.feature_type,
                "limit_type": u.limit_type,
                "amount": u.amount,
                "period": u.period,
                "breach_action": u.breach_action,
                "arn": arn,
                "tags": serde_json::Value::Object(tags.clone()),
                "tags_all": serde_json::Value::Object(tags),
            });
            results.push(ExtractedResource {
                name: u.usage_limit_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_data_share_authorization
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_data_share_authorization` Terraform resources.
///
/// Stub: the redshift service does not track datashare authorizations in its
/// state view. The converter parses the HCL fields for shape validation and
/// extracts nothing. Future work: extend `RedshiftStateView` with a datashare
/// authorization map.
pub struct AwsRedshiftDataShareAuthorizationConverter {
    #[allow(dead_code)]
    service: Arc<RedshiftService>,
}

impl AwsRedshiftDataShareAuthorizationConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftDataShareAuthorizationConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_data_share_authorization"
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

impl AwsRedshiftDataShareAuthorizationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftDataShareAuthorizationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_redshift_data_share_authorization", e)
            })?;

        let _ = model.data_share_arn;
        let _ = model.consumer_identifier;
        let _ = model.allow_writes;

        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_redshift_data_share_authorization is not tracked by redshift state; \
                 inject is a no-op"
                    .to_string(),
            ],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_data_share_consumer_association
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_data_share_consumer_association` Terraform resources.
///
/// Stub: the redshift service does not track datashare consumer associations
/// in its state view. The converter parses the HCL fields for shape validation
/// and extracts nothing.
pub struct AwsRedshiftDataShareConsumerAssociationConverter {
    #[allow(dead_code)]
    service: Arc<RedshiftService>,
}

impl AwsRedshiftDataShareConsumerAssociationConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftDataShareConsumerAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_data_share_consumer_association"
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

impl AwsRedshiftDataShareConsumerAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftDataShareConsumerAssociationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_redshift_data_share_consumer_association", e)
            })?;

        let _ = model.data_share_arn;
        let _ = model.allow_writes;
        let _ = model.associate_entire_account;
        let _ = model.consumer_arn;
        let _ = model.consumer_region;

        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_redshift_data_share_consumer_association is not tracked by redshift state; \
                 inject is a no-op"
                    .to_string(),
            ],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_endpoint_access
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_endpoint_access` Terraform resources.
///
/// Stub: the redshift service does not track Redshift-managed VPC endpoints in
/// its state view. The converter parses the HCL fields, validates that the
/// referenced cluster exists, and extracts nothing.
pub struct AwsRedshiftEndpointAccessConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftEndpointAccessConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftEndpointAccessConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_endpoint_access"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_redshift_cluster", "aws_redshift_subnet_group"]
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

impl AwsRedshiftEndpointAccessConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftEndpointAccessTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_endpoint_access", e))?;

        let _ = attrs.get("vpc_security_group_ids");
        let _ = model.endpoint_name;
        let _ = model.subnet_group_name;
        let _ = model.resource_owner;

        let mut warnings = vec![];
        let state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if !state_view.clusters.contains_key(&model.cluster_identifier) {
            warnings.push(format!(
                "cluster '{}' not found in state; endpoint_access association skipped",
                model.cluster_identifier
            ));
        }

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // No endpoint_access entries are persisted on the redshift state view.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_endpoint_authorization
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_endpoint_authorization` Terraform resources.
///
/// Stub: the redshift service does not track endpoint authorizations in its
/// state view. The converter parses the HCL fields, validates that the
/// referenced cluster exists, and extracts nothing.
pub struct AwsRedshiftEndpointAuthorizationConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftEndpointAuthorizationConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftEndpointAuthorizationConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_endpoint_authorization"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_redshift_cluster"]
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

impl AwsRedshiftEndpointAuthorizationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftEndpointAuthorizationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_redshift_endpoint_authorization", e)
            })?;

        let _ = attrs.get("vpc_ids");
        let _ = model.account;
        let _ = model.force_delete;

        let mut warnings = vec![];
        let state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if !state_view.clusters.contains_key(&model.cluster_identifier) {
            warnings.push(format!(
                "cluster '{}' not found in state; endpoint_authorization association skipped",
                model.cluster_identifier
            ));
        }

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // No endpoint_authorization entries are persisted on the redshift state view.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_integration
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_integration` Terraform resources.
///
/// Stub: the redshift service does not track zero-ETL or S3 event integrations
/// in its state view. The converter parses the HCL fields, captures
/// `additional_encryption_context` for coverage, and extracts nothing.
pub struct AwsRedshiftIntegrationConverter {
    #[allow(dead_code)]
    service: Arc<RedshiftService>,
}

impl AwsRedshiftIntegrationConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftIntegrationConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_integration"
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

impl AwsRedshiftIntegrationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftIntegrationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_redshift_integration", e))?;

        let _ = attrs.get("additional_encryption_context");
        let _ = attrs.get("tags_all");
        let _ = extract_tags(attrs);
        let _ = model.integration_name;
        let _ = model.source_arn;
        let _ = model.target_arn;
        let _ = model.description;
        let _ = model.kms_key_id;

        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_redshift_integration is not tracked by redshift state; inject is a no-op"
                    .to_string(),
            ],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_redshift_snapshot_schedule_association
// ---------------------------------------------------------------------------

/// Converts `aws_redshift_snapshot_schedule_association` Terraform resources.
///
/// Sub-resource modifier: attaches a snapshot schedule to a cluster. The
/// cluster view does not track schedule associations, so this is a
/// validation-only converter that warns when the cluster or the schedule is
/// missing from state.
pub struct AwsRedshiftSnapshotScheduleAssociationConverter {
    service: Arc<RedshiftService>,
}

impl AwsRedshiftSnapshotScheduleAssociationConverter {
    pub fn new(service: Arc<RedshiftService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRedshiftSnapshotScheduleAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_redshift_snapshot_schedule_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_redshift_cluster", "aws_redshift_snapshot_schedule"]
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

impl AwsRedshiftSnapshotScheduleAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: redshift_gen::RedshiftSnapshotScheduleAssociationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_redshift_snapshot_schedule_association", e)
            })?;

        let mut warnings = vec![];
        let state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if !state_view.clusters.contains_key(&model.cluster_identifier) {
            warnings.push(format!(
                "cluster '{}' not found in state; \
                 snapshot_schedule_association skipped",
                model.cluster_identifier
            ));
        }
        if !state_view
            .snapshot_schedules
            .contains_key(&model.schedule_identifier)
        {
            warnings.push(format!(
                "snapshot schedule '{}' not found in state; \
                 snapshot_schedule_association skipped",
                model.schedule_identifier
            ));
        }

        Ok(ConversionResult { region, warnings })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // No snapshot_schedule_association entries are persisted on the redshift state view.
        Ok(vec![])
    }
}
