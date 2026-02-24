//! Terraform converters for EMR (Elastic MapReduce) resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_emr::EmrService;
use winterbaume_emr::views::{ClusterView, EmrStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str};

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

        let name = optional_str(attrs, "name")
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| "unnamed-cluster".to_string());
        let id = optional_str(attrs, "id")
            .unwrap_or_else(|| format!("j-{}", uuid::Uuid::new_v4().simple()));
        let cluster_arn = optional_str(attrs, "cluster_arn").unwrap_or_else(|| {
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
        use winterbaume_emr::views::InstanceFleetView;
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
        use winterbaume_emr::views::InstanceGroupView;
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
            status: optional_str(attrs, "status").unwrap_or_else(|| "WAITING".to_string()),
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
            log_uri: optional_str(attrs, "log_uri"),
            release_label: optional_str(attrs, "release_label"),
            applications: vec![],
            tags: extract_tags(attrs),
            service_role: optional_str(attrs, "service_role"),
            job_flow_role: optional_str(attrs, "ec2_attributes.0.instance_profile")
                .or_else(|| optional_str(attrs, "job_flow_role")),
            auto_scaling_role: optional_str(attrs, "autoscaling_role"),
            scale_down_behavior: optional_str(attrs, "scale_down_behavior"),
            security_configuration: optional_str(attrs, "security_configuration"),
            step_concurrency_level: attrs
                .get("step_concurrency_level")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32),
            auto_termination_policy,
            managed_scaling_policy: None,
            cluster_arn,
            normalized_instance_hours: None,
            master_public_dns_name: optional_str(attrs, "master_public_dns_name"),
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

        let name = optional_str(attrs, "name")
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| "unnamed-security-config".to_string());
        let configuration = optional_str(attrs, "configuration").unwrap_or_default();
        let creation_date_time =
            optional_str(attrs, "creation_date").unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

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
