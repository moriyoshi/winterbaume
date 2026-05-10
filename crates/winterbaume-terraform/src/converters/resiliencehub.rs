//! Terraform converter for Resilience Hub resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_resiliencehub::ResilienceHubService;
use winterbaume_resiliencehub::views::{
    FailurePolicyView, ResilienceHubStateView, ResiliencyPolicyView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::resiliencehub as resiliencehub_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags};

// ---------------------------------------------------------------------------
// aws_resiliencehub_resiliency_policy
// ---------------------------------------------------------------------------

/// Converts `aws_resiliencehub_resiliency_policy` Terraform resources to/from Resilience Hub state.
pub struct AwsResilienceHubResiliencyPolicyConverter {
    service: Arc<ResilienceHubService>,
}

impl AwsResilienceHubResiliencyPolicyConverter {
    pub fn new(service: Arc<ResilienceHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsResilienceHubResiliencyPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_resiliencehub_resiliency_policy"
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

impl AwsResilienceHubResiliencyPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: resiliencehub_gen::ResiliencyPolicyTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_resiliencehub_resiliency_policy", e)
            })?;

        let policy_name = model.policy_name.clone();
        let policy_description = model.policy_description.unwrap_or_default();
        let data_location_constraint = model
            .data_location_constraint
            .unwrap_or_else(|| "AnyLocation".to_string());
        let tier = model.tier.unwrap_or_else(|| "NonCritical".to_string());
        let policy_arn = model.policy_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:resiliencehub:{}:{}:resiliency-policy/{}",
                region, ctx.default_account_id, policy_name
            )
        });

        // Extract failure policies from the "policy" attribute (map of disruption type -> {rpo, rto})
        let mut policy_map: HashMap<String, FailurePolicyView> = HashMap::new();
        if let Some(policy_obj) = instance
            .attributes
            .get("policy")
            .and_then(|v| v.as_object())
        {
            for (disruption_type, fp_val) in policy_obj {
                let rpo = fp_val
                    .get("rpo_in_secs")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;
                let rto = fp_val
                    .get("rto_in_secs")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;
                policy_map.insert(
                    disruption_type.clone(),
                    FailurePolicyView {
                        rpo_in_secs: rpo,
                        rto_in_secs: rto,
                    },
                );
            }
        }

        let policy_view = ResiliencyPolicyView {
            policy_arn: policy_arn.clone(),
            policy_name: policy_name.clone(),
            policy_description,
            data_location_constraint,
            tier,
            policy: policy_map,
            creation_time: chrono::Utc::now(),
            tags: extract_tags(&instance.attributes),
        };

        let mut state_view = ResilienceHubStateView {
            apps: HashMap::new(),
            policies: HashMap::new(),
            tags: HashMap::new(),
        };
        state_view.policies.insert(policy_arn, policy_view);
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
        for policy in view.policies.values() {
            let policy_json: HashMap<String, serde_json::Value> = policy
                .policy
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        serde_json::json!({
                            "rpo_in_secs": v.rpo_in_secs,
                            "rto_in_secs": v.rto_in_secs,
                        }),
                    )
                })
                .collect();

            let attrs = serde_json::json!({
                "id": policy.policy_arn,
                "policy_arn": policy.policy_arn,
                "policy_name": policy.policy_name,
                "policy_description": policy.policy_description,
                "data_location_constraint": policy.data_location_constraint,
                "tier": policy.tier,
                "policy": policy_json,
                "creation_time": policy.creation_time.to_rfc3339(),
                "tags": policy.tags,
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: policy.policy_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
