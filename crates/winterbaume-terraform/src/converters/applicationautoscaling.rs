//! Terraform converters for Application Auto Scaling resources.
//!
//! `ScalableTargetTfModel` and `ScalingPolicyTfModel` are generated from
//! `specs/applicationautoscaling.toml`. The `suspended_state` nested
//! block (target), the `target_tracking_scaling_policy_configuration` and
//! `step_scaling_policy_configuration` nested blocks (policy), the
//! `creation_time` constants, and the `policy_type` default are wired up
//! here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_applicationautoscaling::ApplicationAutoScalingService;
use winterbaume_applicationautoscaling::views::{
    ApplicationAutoScalingStateView, ScalableTargetView, ScalingPolicyView, SuspendedStateView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::applicationautoscaling as appautoscaling_gen;
use crate::util::{classify_deserialize_error, extract_region};

fn minimal_state_view() -> ApplicationAutoScalingStateView {
    ApplicationAutoScalingStateView {
        scalable_targets: vec![],
        scaling_policies: vec![],
        scheduled_actions: vec![],
        tags: HashMap::new(),
    }
}

// ---------------------------------------------------------------------------
// aws_appautoscaling_target
// ---------------------------------------------------------------------------

pub struct AwsAppautoscalingTargetConverter {
    service: Arc<ApplicationAutoScalingService>,
}

impl AwsAppautoscalingTargetConverter {
    pub fn new(service: Arc<ApplicationAutoScalingService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppautoscalingTargetConverter {
    fn resource_type(&self) -> &str {
        "aws_appautoscaling_target"
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

impl AwsAppautoscalingTargetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: appautoscaling_gen::ScalableTargetTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_appautoscaling_target", e))?;

        let attrs = &instance.attributes;

        // suspended_state is a nested-block array: [{...}]
        let suspended_state = attrs.get("suspended_state").map(|v| {
            let obj = v.as_array().and_then(|a| a.first()).unwrap_or(v);
            let dynamic_scaling_in_suspended = obj
                .get("dynamic_scaling_in_suspended")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let dynamic_scaling_out_suspended = obj
                .get("dynamic_scaling_out_suspended")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let scheduled_scaling_suspended = obj
                .get("scheduled_scaling_suspended")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            SuspendedStateView {
                dynamic_scaling_in_suspended,
                dynamic_scaling_out_suspended,
                scheduled_scaling_suspended,
            }
        });

        let target_view = ScalableTargetView {
            service_namespace: model.service_namespace,
            resource_id: model.resource_id,
            scalable_dimension: model.scalable_dimension,
            min_capacity: model.min_capacity,
            max_capacity: model.max_capacity,
            role_arn: model.role_arn.unwrap_or_default(),
            creation_time: chrono::Utc::now().to_rfc3339(),
            suspended_state,
            scalable_target_arn: model.arn.unwrap_or_default(),
        };

        let mut state_view = minimal_state_view();
        state_view.scalable_targets.push(target_view);
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
        for target in &view.scalable_targets {
            let suspended_state = target.suspended_state.as_ref().map(|ss| {
                serde_json::json!([{
                    "dynamic_scaling_in_suspended": ss.dynamic_scaling_in_suspended,
                    "dynamic_scaling_out_suspended": ss.dynamic_scaling_out_suspended,
                    "scheduled_scaling_suspended": ss.scheduled_scaling_suspended,
                }])
            });
            let attrs = serde_json::json!({
                "id": format!("{}/{}/{}", target.service_namespace, target.resource_id, target.scalable_dimension),
                "service_namespace": target.service_namespace,
                "resource_id": target.resource_id,
                "scalable_dimension": target.scalable_dimension,
                "min_capacity": target.min_capacity,
                "max_capacity": target.max_capacity,
                "role_arn": target.role_arn,
                "arn": target.scalable_target_arn,
                "suspended_state": suspended_state,
            });
            results.push(ExtractedResource {
                name: format!(
                    "{}/{}/{}",
                    target.service_namespace, target.resource_id, target.scalable_dimension
                ),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_appautoscaling_policy
// ---------------------------------------------------------------------------

pub struct AwsAppautoscalingPolicyConverter {
    service: Arc<ApplicationAutoScalingService>,
}

impl AwsAppautoscalingPolicyConverter {
    pub fn new(service: Arc<ApplicationAutoScalingService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAppautoscalingPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_appautoscaling_policy"
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

impl AwsAppautoscalingPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: appautoscaling_gen::ScalingPolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_appautoscaling_policy", e))?;

        let attrs = &instance.attributes;

        // Preserve the raw JSON for target_tracking and step_scaling configs
        let target_tracking_scaling_policy_configuration = attrs
            .get("target_tracking_scaling_policy_configuration")
            .and_then(|v| {
                if let Some(arr) = v.as_array() {
                    arr.first().cloned()
                } else {
                    Some(v.clone())
                }
            });
        let step_scaling_policy_configuration = attrs
            .get("step_scaling_policy_configuration")
            .and_then(|v| {
                if let Some(arr) = v.as_array() {
                    arr.first().cloned()
                } else {
                    Some(v.clone())
                }
            });

        let policy_view = ScalingPolicyView {
            policy_arn: model.arn.unwrap_or_default(),
            policy_name: model.name,
            service_namespace: model.service_namespace,
            resource_id: model.resource_id,
            scalable_dimension: model.scalable_dimension,
            policy_type: model
                .policy_type
                .unwrap_or_else(|| "TargetTrackingScaling".to_string()),
            creation_time: chrono::Utc::now().to_rfc3339(),
            target_tracking_scaling_policy_configuration,
            step_scaling_policy_configuration,
        };

        let mut state_view = minimal_state_view();
        state_view.scaling_policies.push(policy_view);
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
        for policy in &view.scaling_policies {
            let mut attrs = serde_json::json!({
                "id": format!("{}/{}/{}/{}", policy.service_namespace, policy.resource_id, policy.scalable_dimension, policy.policy_name),
                "name": policy.policy_name,
                "arn": policy.policy_arn,
                "service_namespace": policy.service_namespace,
                "resource_id": policy.resource_id,
                "scalable_dimension": policy.scalable_dimension,
                "policy_type": policy.policy_type,
            });
            if let Some(ref ttc) = policy.target_tracking_scaling_policy_configuration {
                attrs["target_tracking_scaling_policy_configuration"] = serde_json::json!([ttc]);
            }
            if let Some(ref ssc) = policy.step_scaling_policy_configuration {
                attrs["step_scaling_policy_configuration"] = serde_json::json!([ssc]);
            }
            results.push(ExtractedResource {
                name: format!(
                    "{}/{}/{}/{}",
                    policy.service_namespace,
                    policy.resource_id,
                    policy.scalable_dimension,
                    policy.policy_name,
                ),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
