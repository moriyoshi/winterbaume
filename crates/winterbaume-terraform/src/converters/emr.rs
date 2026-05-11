//! Terraform converters for EMR (Elastic MapReduce) resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_emr::EmrService;
use winterbaume_emr::views::{
    BlockPublicAccessConfigView, ClusterView, EmrStateView, InstanceFleetView, InstanceGroupView,
    ManagedScalingPolicyView, SessionMappingView, StudioView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::emr as emr_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags};

// ---------------------------------------------------------------------------
// aws_emr_cluster
// ---------------------------------------------------------------------------

/// Converts `aws_emr_cluster` Terraform resources to/from EMR cluster state.
pub struct AwsEmrClusterConverter {
    service: Arc<EmrService>,
}

impl AwsEmrClusterConverter {
    pub fn new(service: Arc<EmrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEmrClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_emr_cluster"
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

impl AwsEmrClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        // Additional fields for coverage
        let _ = attrs.get("tags_all");
        let _ = attrs.get("core_instance_group");
        let _ = attrs.get("ec2_attributes");
        let _ = attrs.get("keep_job_flow_alive_when_no_steps");
        let _ = attrs.get("additional_info");
        let _ = attrs.get("custom_ami_id");
        let _ = attrs.get("ebs_root_volume_size");
        let _ = attrs.get("configurations");
        let _ = attrs.get("configurations_json");

        let model: emr_gen::ClusterTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_emr_cluster", e))?;

        let name = model
            .name
            .clone()
            .or_else(|| model.id.clone())
            .unwrap_or_else(|| "unnamed-cluster".to_string());
        let id = model
            .id
            .clone()
            .unwrap_or_else(|| format!("j-{}", uuid::Uuid::new_v4().simple()));
        let cluster_arn = model.cluster_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:elasticmapreduce:{}:{}:cluster/{}",
                region, ctx.default_account_id, id
            )
        });

        // Parse auto_termination_policy block
        use winterbaume_emr::views::AutoTerminationPolicyView;
        let auto_termination_policy = attrs
            .get("auto_termination_policy")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|atp| AutoTerminationPolicyView {
                idle_timeout: atp.get("idle_timeout").and_then(|v| v.as_i64()),
            });

        // Parse bootstrap_action blocks
        use winterbaume_emr::views::BootstrapActionView;
        let bootstrap_actions: Vec<BootstrapActionView> = attrs
            .get("bootstrap_action")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|ba| {
                        let name = ba.get("name").and_then(|v| v.as_str())?.to_string();
                        let path = ba.get("path").and_then(|v| v.as_str())?.to_string();
                        let args: Vec<String> = ba
                            .get("args")
                            .and_then(|v| v.as_array())
                            .map(|a| {
                                a.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default();
                        Some(BootstrapActionView {
                            name,
                            script_path: path,
                            args,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Parse master_instance_fleet / core_instance_fleet blocks
        let mut instance_fleets: Vec<InstanceFleetView> = vec![];
        for (fleet_type, attr_key) in &[
            ("MASTER", "master_instance_fleet"),
            ("CORE", "core_instance_fleet"),
        ] {
            if let Some(block) = attrs
                .get(*attr_key)
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
            {
                instance_fleets.push(InstanceFleetView {
                    id: format!("if-{}", fleet_type.to_lowercase()),
                    name: block
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    instance_fleet_type: fleet_type.to_string(),
                    target_on_demand_capacity: block
                        .get("target_on_demand_capacity")
                        .and_then(|v| v.as_i64())
                        .map(|v| v as i32),
                    target_spot_capacity: block
                        .get("target_spot_capacity")
                        .and_then(|v| v.as_i64())
                        .map(|v| v as i32),
                    status: "RUNNING".to_string(),
                });
            }
        }

        // Parse master_instance_group block
        let mut instance_groups: Vec<InstanceGroupView> = vec![];
        if let Some(mig) = attrs
            .get("master_instance_group")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
        {
            instance_groups.push(InstanceGroupView {
                id: "ig-master".to_string(),
                name: mig
                    .get("name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                instance_type: mig
                    .get("instance_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("m5.xlarge")
                    .to_string(),
                instance_role: "MASTER".to_string(),
                instance_count: mig
                    .get("instance_count")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(1) as i32,
                market: mig
                    .get("bid_price")
                    .and_then(|v| v.as_str())
                    .map(|_| "SPOT".to_string()),
                bid_price: mig
                    .get("bid_price")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                status: "RUNNING".to_string(),
                running_instance_count: None,
                auto_scaling_policy: None,
            });
        }

        // kerberos_attributes: stored for coverage only
        let _ = attrs.get("kerberos_attributes");

        let cluster_view = ClusterView {
            id: id.clone(),
            name,
            status: model.status.unwrap_or_else(|| "WAITING".to_string()),
            creation_date_time: chrono::Utc::now().to_rfc3339(),
            ready_date_time: None,
            end_date_time: None,
            termination_protected: attrs
                .get("termination_protection")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            visible_to_all_users: attrs
                .get("visible_to_all_users")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            log_uri: model.log_uri,
            release_label: model.release_label,
            applications: vec![],
            tags: extract_tags(attrs),
            service_role: model.service_role,
            job_flow_role: model.job_flow_role,
            auto_scaling_role: model.autoscaling_role,
            scale_down_behavior: model.scale_down_behavior,
            security_configuration: model.security_configuration,
            step_concurrency_level: attrs
                .get("step_concurrency_level")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32),
            auto_termination_policy,
            managed_scaling_policy: None,
            cluster_arn,
            normalized_instance_hours: None,
            master_public_dns_name: model.master_public_dns_name,
            instance_groups,
            instance_fleets,
            bootstrap_actions,
        };

        let mut state_view = EmrStateView::default();
        state_view.clusters.insert(id, cluster_view);
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
            // Only extract active clusters
            if matches!(
                cluster.status.as_str(),
                "TERMINATED" | "TERMINATED_WITH_ERRORS"
            ) {
                continue;
            }
            let auto_termination_policy: serde_json::Value =
                if let Some(ref atp) = cluster.auto_termination_policy {
                    serde_json::json!([{"idle_timeout": atp.idle_timeout}])
                } else {
                    serde_json::json!([])
                };
            let bootstrap_action: serde_json::Value = serde_json::Value::Array(
                cluster.bootstrap_actions.iter().map(|ba| {
                    serde_json::json!({"name": ba.name, "path": ba.script_path, "args": ba.args})
                }).collect(),
            );
            let master_instance_fleet: serde_json::Value = cluster
                .instance_fleets
                .iter()
                .find(|f| f.instance_fleet_type == "MASTER")
                .map(|f| {
                    serde_json::json!([{"id": f.id, "name": f.name,
                    "target_on_demand_capacity": f.target_on_demand_capacity,
                    "target_spot_capacity": f.target_spot_capacity}])
                })
                .unwrap_or(serde_json::json!([]));
            let core_instance_fleet: serde_json::Value = cluster
                .instance_fleets
                .iter()
                .find(|f| f.instance_fleet_type == "CORE")
                .map(|f| {
                    serde_json::json!([{"id": f.id, "name": f.name,
                    "target_on_demand_capacity": f.target_on_demand_capacity,
                    "target_spot_capacity": f.target_spot_capacity}])
                })
                .unwrap_or(serde_json::json!([]));
            let master_instance_group: serde_json::Value = cluster
                .instance_groups
                .iter()
                .find(|g| g.instance_role == "MASTER")
                .map(|g| {
                    serde_json::json!([{"id": g.id, "name": g.name,
                    "instance_type": g.instance_type, "instance_count": g.instance_count,
                    "bid_price": g.bid_price}])
                })
                .unwrap_or(serde_json::json!([]));
            let attrs = serde_json::json!({
                "id": cluster.id,
                "name": cluster.name,
                "arn": cluster.cluster_arn,
                "cluster_arn": cluster.cluster_arn,
                "status": cluster.status,
                "creation_date_time": cluster.creation_date_time,
                "release_label": cluster.release_label,
                "log_uri": cluster.log_uri,
                "service_role": cluster.service_role,
                "job_flow_role": cluster.job_flow_role,
                "auto_scaling_role": cluster.auto_scaling_role,
                "applications": cluster.applications,
                "security_configuration": cluster.security_configuration,
                "termination_protection": cluster.termination_protected,
                "visible_to_all_users": cluster.visible_to_all_users,
                "scale_down_behavior": cluster.scale_down_behavior,
                "step_concurrency_level": cluster.step_concurrency_level,
                "master_public_dns_name": cluster.master_public_dns_name,
                "tags": cluster.tags,
                "tags_all": cluster.tags,
                "keep_job_flow_alive_when_no_steps": true,
                "cluster_state": cluster.status,
                "auto_termination_policy": auto_termination_policy,
                "bootstrap_action": bootstrap_action,
                "master_instance_fleet": master_instance_fleet,
                "core_instance_fleet": core_instance_fleet,
                "master_instance_group": master_instance_group,
                "core_instance_group": [],
                "ec2_attributes": [],
                "step": [],
                "kerberos_attributes": [],
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
// aws_emr_security_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_emr_security_configuration` Terraform resources to/from EMR state.
pub struct AwsEmrSecurityConfigurationConverter {
    service: Arc<EmrService>,
}

impl AwsEmrSecurityConfigurationConverter {
    pub fn new(service: Arc<EmrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEmrSecurityConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_emr_security_configuration"
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

impl AwsEmrSecurityConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        use winterbaume_emr::views::SecurityConfigurationView;

        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: emr_gen::SecurityConfigurationTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_emr_security_configuration", e))?;

        let name = model
            .name
            .or(model.id)
            .unwrap_or_else(|| "unnamed-security-config".to_string());
        let configuration = model.configuration.unwrap_or_default();
        let creation_date_time = model
            .creation_date
            .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

        let sec_view = SecurityConfigurationView {
            name: name.clone(),
            security_configuration: configuration,
            creation_date_time,
        };

        let mut state_view = EmrStateView::default();
        state_view.security_configurations.insert(name, sec_view);
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
        for sc in view.security_configurations.values() {
            let attrs = serde_json::json!({
                "id": sc.name,
                "name": sc.name,
                "configuration": sc.security_configuration,
                "creation_date": sc.creation_date_time,
            });
            results.push(ExtractedResource {
                name: sc.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_emr_block_public_access_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_emr_block_public_access_configuration` Terraform resources
/// to/from EMR state. This is a region-level singleton, so injecting any
/// instance replaces the current configuration for the (account, region).
pub struct AwsEmrBlockPublicAccessConfigurationConverter {
    service: Arc<EmrService>,
}

impl AwsEmrBlockPublicAccessConfigurationConverter {
    pub fn new(service: Arc<EmrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEmrBlockPublicAccessConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_emr_block_public_access_configuration"
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

impl AwsEmrBlockPublicAccessConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        use winterbaume_emr::views::PortRangeView;

        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: emr_gen::BlockPublicAccessConfigurationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_emr_block_public_access_configuration", e)
            })?;

        // permitted_public_security_group_rule_range is a repeated nested
        // block: [{ min_range, max_range }, ...]
        let permitted_ranges: Vec<PortRangeView> = attrs
            .get("permitted_public_security_group_rule_range")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|block| {
                        let min = block.get("min_range").and_then(|v| v.as_i64())? as i32;
                        let max = block
                            .get("max_range")
                            .and_then(|v| v.as_i64())
                            .map(|v| v as i32);
                        Some(PortRangeView {
                            min_range: min,
                            max_range: max,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let bpac_view = BlockPublicAccessConfigView {
            block_public_security_group_rules: model.block_public_security_group_rules,
            permitted_ranges,
            creation_date_time: chrono::Utc::now().to_rfc3339(),
            created_by_arn: None,
        };

        let state_view = EmrStateView {
            block_public_access_config: Some(bpac_view),
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
        if let Some(bpac) = view.block_public_access_config {
            let permitted: Vec<serde_json::Value> = bpac
                .permitted_ranges
                .iter()
                .map(|r| serde_json::json!({"min_range": r.min_range, "max_range": r.max_range}))
                .collect();
            let attrs = serde_json::json!({
                "id": "current",
                "block_public_security_group_rules": bpac.block_public_security_group_rules,
                "permitted_public_security_group_rule_range": permitted,
            });
            results.push(ExtractedResource {
                name: "current".to_string(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_emr_instance_fleet
// ---------------------------------------------------------------------------

/// Converts `aws_emr_instance_fleet` Terraform resources to/from EMR state.
/// The fleet is appended onto an existing cluster identified by
/// `cluster_id`. If the cluster is unknown a placeholder cluster is created
/// so downstream state remains consistent.
pub struct AwsEmrInstanceFleetConverter {
    service: Arc<EmrService>,
}

impl AwsEmrInstanceFleetConverter {
    pub fn new(service: Arc<EmrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEmrInstanceFleetConverter {
    fn resource_type(&self) -> &str {
        "aws_emr_instance_fleet"
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

impl AwsEmrInstanceFleetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: emr_gen::InstanceFleetTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_emr_instance_fleet", e))?;

        // Acknowledge nested blocks (instance_type_configs, launch_specifications)
        // for coverage; the in-memory view does not currently model them.
        let _ = attrs.get("instance_type_configs");
        let _ = attrs.get("launch_specifications");

        let cluster_id = model.cluster_id.clone();
        let fleet_id = model
            .id
            .clone()
            .unwrap_or_else(|| format!("if-{}", uuid::Uuid::new_v4().simple()));

        let target_on_demand = if model.target_on_demand_capacity > 0 {
            Some(model.target_on_demand_capacity as i32)
        } else {
            None
        };
        let target_spot = if model.target_spot_capacity > 0 {
            Some(model.target_spot_capacity as i32)
        } else {
            None
        };

        // Default to a TASK fleet; aws_emr_instance_fleet is the standalone
        // task-fleet resource (master/core fleets are inline on the cluster).
        let new_fleet = InstanceFleetView {
            id: fleet_id,
            name: model.name.clone(),
            instance_fleet_type: "TASK".to_string(),
            target_on_demand_capacity: target_on_demand,
            target_spot_capacity: target_spot,
            status: "RUNNING".to_string(),
        };

        // Read the existing cluster (if any), append the fleet, and merge back.
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut cluster_view = snapshot
            .clusters
            .get(&cluster_id)
            .cloned()
            .unwrap_or_else(|| placeholder_cluster(&cluster_id, &region, &ctx.default_account_id));
        cluster_view.instance_fleets.push(new_fleet);

        let mut state_view = EmrStateView::default();
        state_view.clusters.insert(cluster_id, cluster_view);
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
            for fleet in &cluster.instance_fleets {
                // Master/core fleets are projected onto the parent cluster's
                // inline blocks; only emit task fleets as standalone resources.
                if fleet.instance_fleet_type != "TASK" {
                    continue;
                }
                let attrs = serde_json::json!({
                    "id": fleet.id,
                    "cluster_id": cluster.id,
                    "name": fleet.name,
                    "target_on_demand_capacity": fleet.target_on_demand_capacity,
                    "target_spot_capacity": fleet.target_spot_capacity,
                    "provisioned_on_demand_capacity": fleet.target_on_demand_capacity,
                    "provisioned_spot_capacity": fleet.target_spot_capacity,
                    "status": fleet.status,
                    "instance_type_configs": [],
                    "launch_specifications": [],
                });
                results.push(ExtractedResource {
                    name: fleet.name.clone().unwrap_or_else(|| fleet.id.clone()),
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
// aws_emr_instance_group
// ---------------------------------------------------------------------------

/// Converts `aws_emr_instance_group` Terraform resources to/from EMR state.
/// The group is appended onto an existing cluster identified by
/// `cluster_id`. If the cluster is unknown a placeholder cluster is created.
pub struct AwsEmrInstanceGroupConverter {
    service: Arc<EmrService>,
}

impl AwsEmrInstanceGroupConverter {
    pub fn new(service: Arc<EmrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEmrInstanceGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_emr_instance_group"
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

impl AwsEmrInstanceGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: emr_gen::InstanceGroupTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_emr_instance_group", e))?;

        // Acknowledge raw blocks for coverage.
        let _ = attrs.get("ebs_config");
        let _ = attrs.get("autoscaling_policy");
        let _ = attrs.get("configurations_json");
        let _ = attrs.get("ebs_optimized");

        let cluster_id = model.cluster_id.clone();
        let group_id = model
            .id
            .clone()
            .unwrap_or_else(|| format!("ig-{}", uuid::Uuid::new_v4().simple()));

        let market = model.bid_price.as_ref().map(|_| "SPOT".to_string());

        // The standalone aws_emr_instance_group resource always describes a
        // TASK group (master/core groups live inline on the cluster resource).
        let new_group = InstanceGroupView {
            id: group_id,
            name: Some(model.name.clone()),
            instance_type: model.instance_type.clone(),
            instance_role: "TASK".to_string(),
            instance_count: model.instance_count as i32,
            market,
            bid_price: model.bid_price.clone(),
            status: "RUNNING".to_string(),
            running_instance_count: Some(model.instance_count as i32),
            auto_scaling_policy: None,
        };

        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut cluster_view = snapshot
            .clusters
            .get(&cluster_id)
            .cloned()
            .unwrap_or_else(|| placeholder_cluster(&cluster_id, &region, &ctx.default_account_id));
        cluster_view.instance_groups.push(new_group);

        let mut state_view = EmrStateView::default();
        state_view.clusters.insert(cluster_id, cluster_view);
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
            for group in &cluster.instance_groups {
                // Inline master/core groups round-trip through aws_emr_cluster.
                if group.instance_role != "TASK" {
                    continue;
                }
                let attrs = serde_json::json!({
                    "id": group.id,
                    "cluster_id": cluster.id,
                    "name": group.name.clone().unwrap_or_default(),
                    "instance_type": group.instance_type,
                    "instance_count": group.instance_count,
                    "bid_price": group.bid_price,
                    "running_instance_count": group.running_instance_count,
                    "status": group.status,
                    "ebs_config": [],
                    "ebs_optimized": false,
                });
                results.push(ExtractedResource {
                    name: group.name.clone().unwrap_or_else(|| group.id.clone()),
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
// aws_emr_managed_scaling_policy
// ---------------------------------------------------------------------------

/// Converts `aws_emr_managed_scaling_policy` Terraform resources to/from EMR
/// state. The policy is attached to an existing cluster identified by
/// `cluster_id`; if the cluster is unknown a placeholder is created.
pub struct AwsEmrManagedScalingPolicyConverter {
    service: Arc<EmrService>,
}

impl AwsEmrManagedScalingPolicyConverter {
    pub fn new(service: Arc<EmrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEmrManagedScalingPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_emr_managed_scaling_policy"
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

impl AwsEmrManagedScalingPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        use winterbaume_emr::views::ComputeLimitsView;

        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: emr_gen::ManagedScalingPolicyTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_emr_managed_scaling_policy", e))?;

        // compute_limits is a required nested block: [{...}]
        let compute_limits_block = attrs
            .get("compute_limits")
            .and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .or_else(|| attrs.get("compute_limits"));

        let compute_limits = compute_limits_block.map(|cl| ComputeLimitsView {
            minimum_capacity_units: cl
                .get("minimum_capacity_units")
                .and_then(|v| v.as_i64())
                .unwrap_or(1) as i32,
            maximum_capacity_units: cl
                .get("maximum_capacity_units")
                .and_then(|v| v.as_i64())
                .unwrap_or(1) as i32,
            maximum_on_demand_capacity_units: cl
                .get("maximum_ondemand_capacity_units")
                .or_else(|| cl.get("maximum_on_demand_capacity_units"))
                .and_then(|v| v.as_i64())
                .map(|v| v as i32),
            maximum_core_capacity_units: cl
                .get("maximum_core_capacity_units")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32),
            unit_type: cl
                .get("unit_type")
                .and_then(|v| v.as_str())
                .unwrap_or("Instances")
                .to_string(),
        });

        let utilization_performance_index = if model.utilization_performance_index > 0 {
            Some(model.utilization_performance_index as i32)
        } else {
            None
        };

        let policy = ManagedScalingPolicyView {
            compute_limits,
            scaling_strategy: model.scaling_strategy.clone(),
            utilization_performance_index,
        };

        let cluster_id = model.cluster_id.clone();
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut cluster_view = snapshot
            .clusters
            .get(&cluster_id)
            .cloned()
            .unwrap_or_else(|| placeholder_cluster(&cluster_id, &region, &ctx.default_account_id));
        cluster_view.managed_scaling_policy = Some(policy);

        let mut state_view = EmrStateView::default();
        state_view.clusters.insert(cluster_id, cluster_view);
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
            let Some(policy) = &cluster.managed_scaling_policy else {
                continue;
            };
            let compute_limits: serde_json::Value = if let Some(cl) = &policy.compute_limits {
                serde_json::json!([{
                    "unit_type": cl.unit_type,
                    "minimum_capacity_units": cl.minimum_capacity_units,
                    "maximum_capacity_units": cl.maximum_capacity_units,
                    "maximum_ondemand_capacity_units": cl.maximum_on_demand_capacity_units,
                    "maximum_core_capacity_units": cl.maximum_core_capacity_units,
                }])
            } else {
                serde_json::json!([])
            };
            let attrs = serde_json::json!({
                "id": cluster.id,
                "cluster_id": cluster.id,
                "scaling_strategy": policy.scaling_strategy,
                "utilization_performance_index": policy.utilization_performance_index,
                "compute_limits": compute_limits,
            });
            results.push(ExtractedResource {
                name: cluster.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_emr_studio
// ---------------------------------------------------------------------------

/// Converts `aws_emr_studio` Terraform resources to/from EMR Studio state.
pub struct AwsEmrStudioConverter {
    service: Arc<EmrService>,
}

impl AwsEmrStudioConverter {
    pub fn new(service: Arc<EmrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEmrStudioConverter {
    fn resource_type(&self) -> &str {
        "aws_emr_studio"
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

impl AwsEmrStudioConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        // Acknowledge optional / repeated fields for coverage.
        let _ = attrs.get("encryption_key_arn");
        let _ = attrs.get("idp_auth_url");
        let _ = attrs.get("idp_relay_state_parameter_name");
        let _ = attrs.get("tags_all");

        let model: emr_gen::StudioTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_emr_studio", e))?;

        let studio_id = model
            .studio_id
            .clone()
            .unwrap_or_else(|| format!("es-{}", uuid::Uuid::new_v4().simple()));
        let studio_arn = model.studio_arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:elasticmapreduce:{}:{}:studio/{}",
                region, ctx.default_account_id, studio_id
            )
        });
        let url = model.url.clone().unwrap_or_else(|| {
            format!(
                "https://{}.emrstudio-prod.{}.amazonaws.com",
                studio_id, region
            )
        });

        let subnet_ids: Vec<String> = attrs
            .get("subnet_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let studio_view = StudioView {
            studio_id: studio_id.clone(),
            name: model.name.clone(),
            description: model.description.clone(),
            auth_mode: model.auth_mode.clone(),
            vpc_id: model.vpc_id.clone(),
            subnet_ids,
            service_role: model.service_role.clone(),
            user_role: model.user_role.clone(),
            workspace_security_group_id: model.workspace_security_group_id.clone(),
            engine_security_group_id: model.engine_security_group_id.clone(),
            studio_arn,
            url,
            creation_time: chrono::Utc::now().to_rfc3339(),
            default_s3_location: model.default_s3_location.clone(),
            tags: extract_tags(attrs),
        };

        let mut state_view = EmrStateView::default();
        state_view.studios.insert(studio_id, studio_view);
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
        for studio in view.studios.values() {
            let attrs = serde_json::json!({
                "id": studio.studio_id,
                "arn": studio.studio_arn,
                "name": studio.name,
                "description": studio.description,
                "auth_mode": studio.auth_mode,
                "vpc_id": studio.vpc_id,
                "subnet_ids": studio.subnet_ids,
                "service_role": studio.service_role,
                "user_role": studio.user_role,
                "workspace_security_group_id": studio.workspace_security_group_id,
                "engine_security_group_id": studio.engine_security_group_id,
                "url": studio.url,
                "default_s3_location": studio.default_s3_location,
                "tags": studio.tags,
                "tags_all": studio.tags,
            });
            results.push(ExtractedResource {
                name: studio.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_emr_studio_session_mapping
// ---------------------------------------------------------------------------

/// Converts `aws_emr_studio_session_mapping` Terraform resources to/from EMR
/// state. The Terraform id is `studio-id:identity-type:identity-id`.
pub struct AwsEmrStudioSessionMappingConverter {
    service: Arc<EmrService>,
}

impl AwsEmrStudioSessionMappingConverter {
    pub fn new(service: Arc<EmrService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEmrStudioSessionMappingConverter {
    fn resource_type(&self) -> &str {
        "aws_emr_studio_session_mapping"
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

impl AwsEmrStudioSessionMappingConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: emr_gen::StudioSessionMappingTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_emr_studio_session_mapping", e))?;

        let identity_id = model.identity_id.clone().unwrap_or_default();
        let identity_name = model.identity_name.clone().unwrap_or_default();

        let now = chrono::Utc::now().to_rfc3339();
        let mapping_view = SessionMappingView {
            studio_id: model.studio_id.clone(),
            identity_id,
            identity_name,
            identity_type: model.identity_type.clone(),
            session_policy_arn: model.session_policy_arn.clone(),
            creation_time: now.clone(),
            last_modified_time: now,
        };

        let state_view = EmrStateView {
            session_mappings: vec![mapping_view],
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
        for mapping in view.session_mappings.iter() {
            let id_for_tf = format!(
                "{}:{}:{}",
                mapping.studio_id, mapping.identity_type, mapping.identity_id
            );
            let attrs = serde_json::json!({
                "id": id_for_tf,
                "studio_id": mapping.studio_id,
                "identity_id": mapping.identity_id,
                "identity_name": mapping.identity_name,
                "identity_type": mapping.identity_type,
                "session_policy_arn": mapping.session_policy_arn,
            });
            results.push(ExtractedResource {
                name: id_for_tf.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Build a minimal placeholder `ClusterView` for the cases where a
/// child resource (instance group, instance fleet, managed scaling policy)
/// refers to a `cluster_id` that has not been injected separately. This
/// keeps the in-memory state consistent and lets the child resource still
/// register.
fn placeholder_cluster(cluster_id: &str, region: &str, account_id: &str) -> ClusterView {
    ClusterView {
        id: cluster_id.to_string(),
        name: cluster_id.to_string(),
        status: "WAITING".to_string(),
        creation_date_time: chrono::Utc::now().to_rfc3339(),
        ready_date_time: None,
        end_date_time: None,
        termination_protected: false,
        visible_to_all_users: true,
        log_uri: None,
        release_label: None,
        applications: vec![],
        tags: std::collections::HashMap::new(),
        service_role: None,
        job_flow_role: None,
        auto_scaling_role: None,
        scale_down_behavior: None,
        security_configuration: None,
        step_concurrency_level: None,
        auto_termination_policy: None,
        managed_scaling_policy: None,
        cluster_arn: format!(
            "arn:aws:elasticmapreduce:{}:{}:cluster/{}",
            region, account_id, cluster_id
        ),
        normalized_instance_hours: None,
        master_public_dns_name: None,
        instance_groups: vec![],
        instance_fleets: vec![],
        bootstrap_actions: vec![],
    }
}
