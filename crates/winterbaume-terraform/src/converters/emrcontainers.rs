//! Terraform converter for EMR Containers resources.
//!
//! `VirtualClusterTfModel` is generated from `specs/emrcontainers.toml`. The
//! ARN template, the `state = "RUNNING"` constant, the `created_at`
//! timestamp, and the nested `container_provider` block (with its `info`
//! / `eks_info` sub-blocks) are wired up here from raw attributes.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_emrcontainers::EmrContainersService;
use winterbaume_emrcontainers::views::{
    ContainerInfoView, ContainerProviderView, EksInfoView, EmrContainersStateView,
    VirtualClusterView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::emrcontainers as emrcontainers_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_emrcontainers_virtual_cluster
// ---------------------------------------------------------------------------

/// Converts `aws_emrcontainers_virtual_cluster` Terraform resources to/from EMR Containers state.
pub struct AwsEmrcontainersVirtualClusterConverter {
    service: Arc<EmrContainersService>,
}

impl AwsEmrcontainersVirtualClusterConverter {
    pub fn new(service: Arc<EmrContainersService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEmrcontainersVirtualClusterConverter {
    fn resource_type(&self) -> &str {
        "aws_emrcontainers_virtual_cluster"
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

impl AwsEmrcontainersVirtualClusterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: emrcontainers_gen::VirtualClusterTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_emrcontainers_virtual_cluster", e))?;

        let id = model
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:emr-containers:{}:{}:/virtualclusters/{}",
                region, ctx.default_account_id, id
            )
        });

        // Parse container_provider block from raw attributes (nested
        // object-or-array shape isn't expressible in the spec).
        let attrs = &instance.attributes;
        let cp = attrs.get("container_provider");
        let container_provider = if let Some(cp_val) = cp {
            let cp_obj = cp_val.as_array().and_then(|a| a.first()).unwrap_or(cp_val);
            let provider_type = cp_obj
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("EKS")
                .to_string();
            let provider_id = cp_obj
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let info = cp_obj.get("info").map(|info_val| {
                let info_obj = info_val
                    .as_array()
                    .and_then(|a| a.first())
                    .unwrap_or(info_val);
                let eks_info = info_obj.get("eks_info").map(|eks_val| {
                    let eks_obj = eks_val
                        .as_array()
                        .and_then(|a| a.first())
                        .unwrap_or(eks_val);
                    EksInfoView {
                        namespace: eks_obj
                            .get("namespace")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                    }
                });
                ContainerInfoView { eks_info }
            });
            ContainerProviderView {
                provider_type,
                id: provider_id,
                info,
            }
        } else {
            ContainerProviderView {
                provider_type: "EKS".to_string(),
                id: String::new(),
                info: None,
            }
        };

        let vc_view = VirtualClusterView {
            id: id.clone(),
            name: model.name.clone(),
            arn,
            state: "RUNNING".to_string(),
            container_provider,
            created_at: chrono::Utc::now(),
            tags: model.tags,
        };

        let mut state_view = EmrContainersStateView {
            virtual_clusters: HashMap::new(),
            job_runs: HashMap::new(),
            managed_endpoints: HashMap::new(),
            job_templates: HashMap::new(),
            security_configurations: HashMap::new(),
        };
        state_view.virtual_clusters.insert(id, vc_view);
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
        for vc in view.virtual_clusters.values() {
            let container_provider = {
                let mut cp = serde_json::json!({
                    "type": vc.container_provider.provider_type,
                    "id": vc.container_provider.id,
                });
                if let Some(info) = &vc.container_provider.info {
                    if let Some(eks) = &info.eks_info {
                        cp["info"] = serde_json::json!([{
                            "eks_info": [{
                                "namespace": eks.namespace,
                            }],
                        }]);
                    }
                }
                cp
            };
            let attrs = serde_json::json!({
                "id": vc.id,
                "name": vc.name,
                "arn": vc.arn,
                "container_provider": [container_provider],
                "tags": vc.tags,
            });
            results.push(ExtractedResource {
                name: vc.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
