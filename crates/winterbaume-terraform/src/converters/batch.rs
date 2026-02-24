//! Terraform converters for AWS Batch resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_batch::BatchService;
use winterbaume_batch::views::{
    BatchStateView, ComputeEnvironmentOrderView, ComputeEnvironmentView, ContainerPropertiesView,
    FairsharePolicyView, JobDefinitionView, JobQueueView, ResourceRequirementView,
    SchedulingPolicyView, ShareAttributesView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{
    extract_region, extract_tags, optional_bool, optional_i64, optional_str, require_str,
};

// ---------------------------------------------------------------------------
// aws_batch_compute_environment
// ---------------------------------------------------------------------------

pub struct AwsBatchComputeEnvironmentConverter {
    service: Arc<BatchService>,
}

impl AwsBatchComputeEnvironmentConverter {
    pub fn new(service: Arc<BatchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBatchComputeEnvironmentConverter {
    fn resource_type(&self) -> &str {
        "aws_batch_compute_environment"
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

impl AwsBatchComputeEnvironmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_batch_compute_environment")?;
        let ce_type = require_str(attrs, "type", "aws_batch_compute_environment")?;
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:batch:{}:{}:compute-environment/{}",
                region, ctx.default_account_id, name
            )
        });
        let state = optional_str(attrs, "state").unwrap_or_else(|| "ENABLED".to_string());
        let service_role = optional_str(attrs, "service_role");
        let compute_resources: Vec<serde_json::Value> = attrs
            .get("compute_resources")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let eks_configuration: Vec<serde_json::Value> = attrs
            .get("eks_configuration")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let mut tags = extract_tags(attrs);
        // Merge tags_all into tags (provider-level default tags)
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }
        // update_policy is accepted but not stored in internal state (Terraform-only lifecycle field)
        let _update_policy = attrs.get("update_policy");

        let view = ComputeEnvironmentView {
            compute_environment_name: name.to_string(),
            compute_environment_arn: arn,
            ce_type: ce_type.to_string(),
            state,
            status: "VALID".to_string(),
            status_reason: String::new(),
            service_role,
            tags,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            compute_resources,
            eks_configuration,
        };

        let mut state_view = minimal_batch_state_view();
        state_view
            .compute_environments
            .insert(name.to_string(), view);
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
        for ce in view.compute_environments.values() {
            let attrs = serde_json::json!({
                "id": ce.compute_environment_name,
                "name": ce.compute_environment_name,
                "arn": ce.compute_environment_arn,
                "type": ce.ce_type,
                "state": ce.state,
                "status": ce.status,
                "service_role": ce.service_role,
                "compute_resources": ce.compute_resources,
                "eks_configuration": ce.eks_configuration,
                "tags": ce.tags,
                "tags_all": ce.tags,
                "update_policy": [],
            });
            results.push(ExtractedResource {
                name: ce.compute_environment_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_batch_job_queue
// ---------------------------------------------------------------------------

pub struct AwsBatchJobQueueConverter {
    service: Arc<BatchService>,
}

impl AwsBatchJobQueueConverter {
    pub fn new(service: Arc<BatchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBatchJobQueueConverter {
    fn resource_type(&self) -> &str {
        "aws_batch_job_queue"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_batch_compute_environment",
            "aws_batch_scheduling_policy",
        ]
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

impl AwsBatchJobQueueConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_batch_job_queue")?;
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:batch:{}:{}:job-queue/{}",
                region, ctx.default_account_id, name
            )
        });
        let state = optional_str(attrs, "state").unwrap_or_else(|| "ENABLED".to_string());
        let priority = optional_i64(attrs, "priority").unwrap_or(1) as i32;
        let tags = extract_tags(attrs);

        let ce_order: Vec<ComputeEnvironmentOrderView> = attrs
            .get("compute_environment_order")
            .or_else(|| attrs.get("compute_environments"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .enumerate()
                    .map(|(i, item)| {
                        let ce = item
                            .get("compute_environment")
                            .and_then(|v| v.as_str())
                            .or_else(|| item.as_str())
                            .unwrap_or("")
                            .to_string();
                        let order = item
                            .get("order")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(i as i64) as i32;
                        ComputeEnvironmentOrderView {
                            order,
                            compute_environment: ce,
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        let job_state_time_limit_action: Vec<serde_json::Value> = attrs
            .get("job_state_time_limit_action")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let view = JobQueueView {
            job_queue_name: name.to_string(),
            job_queue_arn: arn,
            state,
            status: "VALID".to_string(),
            status_reason: String::new(),
            priority,
            compute_environment_order: ce_order,
            tags,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            scheduling_policy_arn: optional_str(attrs, "scheduling_policy_arn"),
            job_state_time_limit_action,
        };

        let mut state_view = minimal_batch_state_view();
        state_view.job_queues.insert(name.to_string(), view);
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
        for jq in view.job_queues.values() {
            let ce_order_val: Vec<serde_json::Value> = jq
                .compute_environment_order
                .iter()
                .map(|ceo| {
                    serde_json::json!({
                        "order": ceo.order,
                        "compute_environment": ceo.compute_environment,
                    })
                })
                .collect();
            let attrs = serde_json::json!({
                "id": jq.job_queue_name,
                "name": jq.job_queue_name,
                "arn": jq.job_queue_arn,
                "state": jq.state,
                "priority": jq.priority,
                "scheduling_policy_arn": jq.scheduling_policy_arn,
                "compute_environment_order": ce_order_val,
                "job_state_time_limit_action": jq.job_state_time_limit_action,
                "tags": jq.tags,
            });
            results.push(ExtractedResource {
                name: jq.job_queue_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_batch_job_definition
// ---------------------------------------------------------------------------

pub struct AwsBatchJobDefinitionConverter {
    service: Arc<BatchService>,
}

impl AwsBatchJobDefinitionConverter {
    pub fn new(service: Arc<BatchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBatchJobDefinitionConverter {
    fn resource_type(&self) -> &str {
        "aws_batch_job_definition"
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

impl AwsBatchJobDefinitionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_batch_job_definition")?;
        let job_type = require_str(attrs, "type", "aws_batch_job_definition")?;
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:batch:{}:{}:job-definition/{}:1",
                region, ctx.default_account_id, name
            )
        });
        let tags = extract_tags(attrs);
        let _tags_all = attrs.get("tags_all");
        let _timeout = attrs.get("timeout");
        let _propagate_tags = optional_bool(attrs, "propagate_tags");
        let _scheduling_priority = attrs.get("scheduling_priority");

        let eks_properties: Vec<serde_json::Value> = attrs
            .get("eks_properties")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let retry_strategy: Vec<serde_json::Value> = attrs
            .get("retry_strategy")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        // Parse container_properties if provided
        let container_properties = attrs
            .get("container_properties")
            .and_then(|v| {
                // Terraform may encode container_properties as a JSON string
                if let Some(s) = v.as_str() {
                    serde_json::from_str::<serde_json::Value>(s).ok()
                } else {
                    Some(v.clone())
                }
            })
            .map(|cp| {
                let image = cp
                    .get("image")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let command: Vec<String> = cp
                    .get("command")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let resource_requirements: Vec<ResourceRequirementView> = cp
                    .get("resourceRequirements")
                    .or_else(|| cp.get("resource_requirements"))
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|rr| {
                                let rt = rr.get("type").and_then(|v| v.as_str())?;
                                let val = rr.get("value").and_then(|v| v.as_str())?;
                                Some(ResourceRequirementView {
                                    resource_type: rt.to_string(),
                                    value: val.to_string(),
                                })
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                ContainerPropertiesView {
                    image,
                    command,
                    resource_requirements,
                }
            });

        let view = JobDefinitionView {
            job_definition_name: name.to_string(),
            job_definition_arn: arn,
            revision: optional_i64(attrs, "revision").unwrap_or(1) as i32,
            status: "ACTIVE".to_string(),
            job_definition_type: job_type.to_string(),
            container_properties,
            tags,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            eks_properties,
            retry_strategy,
        };

        let mut state_view = minimal_batch_state_view();
        state_view
            .job_definitions
            .insert(name.to_string(), vec![view]);
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
        for revisions in view.job_definitions.values() {
            if let Some(jd) = revisions.last() {
                let container_props_json = jd.container_properties.as_ref().map(|cp| {
                    serde_json::json!({
                        "image": cp.image,
                        "command": cp.command,
                        "resourceRequirements": cp.resource_requirements.iter().map(|rr| {
                            serde_json::json!({
                                "type": rr.resource_type,
                                "value": rr.value,
                            })
                        }).collect::<Vec<_>>(),
                    })
                });
                let arn_prefix = jd
                    .job_definition_arn
                    .rsplit_once(':')
                    .map(|x| x.0)
                    .unwrap_or(&jd.job_definition_arn);
                let attrs = serde_json::json!({
                    "id": jd.job_definition_name,
                    "name": jd.job_definition_name,
                    "arn": jd.job_definition_arn,
                    "revision": jd.revision,
                    "type": jd.job_definition_type,
                    "status": jd.status,
                    "container_properties": container_props_json,
                    "eks_properties": jd.eks_properties,
                    "retry_strategy": jd.retry_strategy,
                    "tags": jd.tags,
                    "tags_all": jd.tags,
                    "arn_prefix": arn_prefix,
                    "tags_propagated": false,
                    "timeout": [],
                });
                results.push(ExtractedResource {
                    name: jd.job_definition_name.clone(),
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
// aws_batch_scheduling_policy
// ---------------------------------------------------------------------------

pub struct AwsBatchSchedulingPolicyConverter {
    service: Arc<BatchService>,
}

impl AwsBatchSchedulingPolicyConverter {
    pub fn new(service: Arc<BatchService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBatchSchedulingPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_batch_scheduling_policy"
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

impl AwsBatchSchedulingPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_batch_scheduling_policy")?;
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:batch:{}:{}:scheduling-policy/{}",
                region, ctx.default_account_id, name
            )
        });
        let tags = extract_tags(attrs);

        // Parse fair_share_policy block: [{ compute_reservation, share_decay_seconds, share_distribution: [{...}] }]
        let fairshare_policy = attrs
            .get("fair_share_policy")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|fp| {
                let compute_reservation = fp
                    .get("compute_reservation")
                    .and_then(|v| v.as_i64())
                    .map(|v| v as i32);
                let share_decay_seconds = fp
                    .get("share_decay_seconds")
                    .and_then(|v| v.as_i64())
                    .map(|v| v as i32);
                let share_distribution: Vec<ShareAttributesView> = fp
                    .get("share_distribution")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|sd| {
                                let share_identifier =
                                    sd.get("share_identifier").and_then(|v| v.as_str())?;
                                let weight_factor = sd
                                    .get("weight_factor")
                                    .and_then(|v| v.as_f64())
                                    .map(|f| f as f32);
                                Some(ShareAttributesView {
                                    share_identifier: share_identifier.to_string(),
                                    weight_factor,
                                })
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                FairsharePolicyView {
                    compute_reservation,
                    share_decay_seconds,
                    share_distribution,
                }
            });

        let view = SchedulingPolicyView {
            name: name.to_string(),
            arn,
            fairshare_policy,
            tags,
        };

        let mut state_view = minimal_batch_state_view();
        state_view
            .scheduling_policies
            .insert(name.to_string(), view);
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
        for sp in view.scheduling_policies.values() {
            let fair_share_policy_val = sp
                .fairshare_policy
                .as_ref()
                .map(|fp| {
                    let share_distribution: Vec<serde_json::Value> = fp
                        .share_distribution
                        .iter()
                        .map(|sd| {
                            serde_json::json!({
                                "share_identifier": sd.share_identifier,
                                "weight_factor": sd.weight_factor,
                            })
                        })
                        .collect();
                    serde_json::json!([{
                        "compute_reservation": fp.compute_reservation,
                        "share_decay_seconds": fp.share_decay_seconds,
                        "share_distribution": share_distribution,
                    }])
                })
                .unwrap_or(serde_json::json!([]));
            let attrs = serde_json::json!({
                "id": sp.name,
                "name": sp.name,
                "arn": sp.arn,
                "fair_share_policy": fair_share_policy_val,
                "tags": sp.tags,
            });
            results.push(ExtractedResource {
                name: sp.name.clone(),
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

fn minimal_batch_state_view() -> BatchStateView {
    BatchStateView::default()
}
