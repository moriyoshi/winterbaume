//! Terraform converters for Application Auto Scaling resources.

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
use crate::util::{extract_region, optional_i64, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let service_namespace =
            require_str(attrs, "service_namespace", "aws_appautoscaling_target")?;
        let resource_id = require_str(attrs, "resource_id", "aws_appautoscaling_target")?;
        let scalable_dimension =
            require_str(attrs, "scalable_dimension", "aws_appautoscaling_target")?;
        let min_capacity = optional_i64(attrs, "min_capacity").unwrap_or(0);
        let max_capacity = optional_i64(attrs, "max_capacity").unwrap_or(0);
        let role_arn = optional_str(attrs, "role_arn").unwrap_or_default();
        let scalable_target_arn = optional_str(attrs, "arn").unwrap_or_default();

        let suspended_state = attrs.get("suspended_state").map(|v| {
            // Terraform represents this as a list with one element
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
            service_namespace: service_namespace.to_string(),
            resource_id: resource_id.to_string(),
            scalable_dimension: scalable_dimension.to_string(),
            min_capacity,
            max_capacity,
            role_arn,
            creation_time: chrono::Utc::now().to_rfc3339(),
            suspended_state,
            scalable_target_arn,
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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let policy_name = require_str(attrs, "name", "aws_appautoscaling_policy")?;
        let service_namespace =
            require_str(attrs, "service_namespace", "aws_appautoscaling_policy")?;
        let resource_id = require_str(attrs, "resource_id", "aws_appautoscaling_policy")?;
        let scalable_dimension =
            require_str(attrs, "scalable_dimension", "aws_appautoscaling_policy")?;
        let policy_type = optional_str(attrs, "policy_type")
            .unwrap_or_else(|| "TargetTrackingScaling".to_string());
        let policy_arn = optional_str(attrs, "arn").unwrap_or_default();

        // Preserve the raw JSON for target_tracking and step_scaling configs
        let target_tracking_scaling_policy_configuration = attrs
            .get("target_tracking_scaling_policy_configuration")
            .and_then(|v| {
                // Terraform wraps in array
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
            policy_arn,
            policy_name: policy_name.to_string(),
            service_namespace: service_namespace.to_string(),
            resource_id: resource_id.to_string(),
            scalable_dimension: scalable_dimension.to_string(),
            policy_type,
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
