//! Terraform converter for CloudHSM V2 resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_cloudhsmv2::CloudHsmV2Service;
use winterbaume_cloudhsmv2::views::{CloudHsmV2StateView, ClusterView, TagView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_cloudhsm_v2_cluster
// ---------------------------------------------------------------------------

/// Converts `aws_cloudhsm_v2_cluster` Terraform resources to/from CloudHSM V2 state.
pub struct AwsCloudHsmV2ClusterConverter {
    service: Arc<CloudHsmV2Service>,
}

impl AwsCloudHsmV2ClusterConverter {
    pub fn new(service: Arc<CloudHsmV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudHsmV2ClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudhsm_v2_cluster"
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

impl AwsCloudHsmV2ClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let hsm_type = require_str(attrs, "hsm_type", "aws_cloudhsm_v2_cluster")?;
        let region = extract_region(attrs, &ctx.default_region);
        let cluster_id = optional_str(attrs, "cluster_id").unwrap_or_else(|| {
            format!(
                "cluster-{}",
                uuid::Uuid::new_v4()
                    .to_string()
                    .replace('-', "")
                    .get(..17)
                    .unwrap_or("00000000000000000")
            )
        });
        let vpc_id = optional_str(attrs, "vpc_id").unwrap_or_default();
        let state =
            optional_str(attrs, "cluster_state").unwrap_or_else(|| "UNINITIALIZED".to_string());
        let security_group = optional_str(attrs, "security_group_id").unwrap_or_default();
        let source_backup_id = optional_str(attrs, "source_backup_id");
        let backup_policy = "DEFAULT".to_string();

        // Parse subnet_ids into subnet_mapping
        let subnet_mapping: HashMap<String, String> =
            if let Some(arr) = attrs.get("subnet_ids").and_then(|v| v.as_array()) {
                arr.iter()
                    .enumerate()
                    .filter_map(|(i, v)| {
                        let subnet = v.as_str()?.to_string();
                        Some((format!("az-{i}"), subnet))
                    })
                    .collect()
            } else {
                HashMap::new()
            };

        // Parse tags (tags_all first, then tags overrides)
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
        let tag_list: Vec<TagView> = tag_map
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let cluster_view = ClusterView {
            cluster_id: cluster_id.clone(),
            hsm_type: hsm_type.to_string(),
            subnet_mapping,
            vpc_id,
            state,
            security_group,
            source_backup_id,
            backup_policy,
            backup_retention_policy: None,
            create_timestamp: 0.0,
            tag_list,
            region: region.clone(),
            account_id: ctx.default_account_id.clone(),
            hsms: vec![],
        };

        let mut state_view = CloudHsmV2StateView {
            clusters: HashMap::new(),
            backups: HashMap::new(),
            resource_policies: HashMap::new(),
        };
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
            let tags: HashMap<&str, &str> = cluster
                .tag_list
                .iter()
                .map(|t| (t.key.as_str(), t.value.as_str()))
                .collect();
            let subnet_ids: Vec<&str> = cluster
                .subnet_mapping
                .values()
                .map(|s| s.as_str())
                .collect();
            let hsms_json: Vec<serde_json::Value> = cluster
                .hsms
                .iter()
                .map(|h| {
                    serde_json::json!({
                        "hsm_id": h.hsm_id,
                        "cluster_id": h.cluster_id,
                        "availability_zone": h.availability_zone,
                        "subnet_id": h.subnet_id,
                        "eni_id": h.eni_id,
                        "eni_ip": h.eni_ip,
                        "state": h.state,
                    })
                })
                .collect();
            let attrs = serde_json::json!({
                "id": cluster.cluster_id,
                "cluster_id": cluster.cluster_id,
                "hsm_type": cluster.hsm_type,
                "subnet_ids": subnet_ids,
                "subnet_mapping": cluster.subnet_mapping,
                "vpc_id": cluster.vpc_id,
                "cluster_state": cluster.state,
                "security_group_id": cluster.security_group,
                "source_backup_id": cluster.source_backup_id,
                "backup_policy": cluster.backup_policy,
                "backup_retention_policy": cluster.backup_retention_policy,
                "create_timestamp": cluster.create_timestamp,
                "tags": tags,
                "hsms": hsms_json,
            });
            results.push(ExtractedResource {
                name: cluster.cluster_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
