//! Terraform converters for Auto Scaling resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_autoscaling::AutoScalingService;
use winterbaume_autoscaling::views::{
    AutoScalingGroupView, AutoScalingStateView, LaunchConfigurationView, LifecycleHookView,
    NotificationConfigView, ScalingPolicyView, ScheduledActionView, TagView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::autoscaling as autoscaling_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags, optional_i64};

// ---------------------------------------------------------------------------
// aws_autoscaling_group
// ---------------------------------------------------------------------------

pub struct AwsAutoscalingGroupConverter {
    service: Arc<AutoScalingService>,
}

impl AwsAutoscalingGroupConverter {
    pub fn new(service: Arc<AutoScalingService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAutoscalingGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_autoscaling_group"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_launch_configuration"]
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

impl AwsAutoscalingGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: autoscaling_gen::AutoScalingGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_autoscaling_group", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:autoscaling:{}:{}:autoScalingGroup:{}:autoScalingGroupName/{}",
                region,
                ctx.default_account_id,
                uuid::Uuid::new_v4(),
                name
            )
        });

        let min_size = model.min_size as i32;
        let max_size = model.max_size as i32;
        let desired = optional_i64(attrs, "desired_capacity").unwrap_or(model.min_size) as i32;
        let azs = extract_string_array(attrs, "availability_zones");
        let health_check_type = model.health_check_type.unwrap_or_else(|| "EC2".to_string());
        let health_check_grace = model.health_check_grace_period as i32;
        let cooldown = model.default_cooldown as i32;
        let termination_policies = extract_string_array(attrs, "termination_policies");

        let tf_tags = extract_tags(attrs);
        let tags: Vec<TagView> = tf_tags
            .into_iter()
            .map(|(k, v)| TagView {
                key: k,
                value: v,
                resource_id: name.clone(),
                resource_type: "auto-scaling-group".to_string(),
                propagate_at_launch: true,
            })
            .collect();

        let view = AutoScalingGroupView {
            name: name.clone(),
            arn,
            min_size,
            max_size,
            desired_capacity: desired,
            launch_configuration_name: model.launch_configuration,
            vpc_zone_identifier: model.vpc_zone_identifier,
            availability_zones: azs,
            health_check_type,
            health_check_grace_period: health_check_grace,
            default_cooldown: cooldown,
            tags,
            suspended_processes: vec![],
            termination_policies,
            enabled_metrics: vec![],
            created_time: chrono::Utc::now().to_rfc3339(),
            status: None,
            notification_configurations: vec![],
            attached_load_balancers: vec![],
            attached_target_groups: vec![],
            warm_pool: None,
            instances: vec![],
        };

        let mut state_view = minimal_asg_state_view();
        state_view.groups.insert(name, view);
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
        for asg in view.groups.values() {
            let tags: HashMap<String, String> = asg
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": asg.name,
                "name": asg.name,
                "arn": asg.arn,
                "min_size": asg.min_size,
                "max_size": asg.max_size,
                "desired_capacity": asg.desired_capacity,
                "launch_configuration": asg.launch_configuration_name,
                "vpc_zone_identifier": asg.vpc_zone_identifier,
                "availability_zones": asg.availability_zones,
                "health_check_type": asg.health_check_type,
                "health_check_grace_period": asg.health_check_grace_period,
                "default_cooldown": asg.default_cooldown,
                "termination_policies": asg.termination_policies,
                "tags": tags,
            });
            results.push(ExtractedResource {
                name: asg.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_launch_configuration
// ---------------------------------------------------------------------------

pub struct AwsLaunchConfigurationConverter {
    service: Arc<AutoScalingService>,
}

impl AwsLaunchConfigurationConverter {
    pub fn new(service: Arc<AutoScalingService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLaunchConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_launch_configuration"
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

impl AwsLaunchConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: autoscaling_gen::LaunchConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_launch_configuration", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:autoscaling:{}:{}:launchConfiguration:{}:launchConfigurationName/{}",
                region,
                ctx.default_account_id,
                uuid::Uuid::new_v4(),
                name
            )
        });
        let security_groups = extract_string_array(attrs, "security_groups");
        let associate_public_ip_address = attrs
            .get("associate_public_ip_address")
            .and_then(|v| v.as_bool());

        let view = LaunchConfigurationView {
            name: name.clone(),
            arn,
            image_id: model.image_id,
            instance_type: model.instance_type,
            key_name: model.key_name,
            iam_instance_profile: model.iam_instance_profile,
            user_data: model.user_data,
            security_groups,
            ebs_optimized: model.ebs_optimized,
            associate_public_ip_address,
            spot_price: model.spot_price,
            created_time: chrono::Utc::now().to_rfc3339(),
        };

        let mut state_view = minimal_asg_state_view();
        state_view.launch_configs.insert(name, view);
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
        for lc in view.launch_configs.values() {
            let attrs = serde_json::json!({
                "id": lc.name,
                "name": lc.name,
                "arn": lc.arn,
                "image_id": lc.image_id,
                "instance_type": lc.instance_type,
                "key_name": lc.key_name,
                "iam_instance_profile": lc.iam_instance_profile,
                "security_groups": lc.security_groups,
                "ebs_optimized": lc.ebs_optimized,
                "associate_public_ip_address": lc.associate_public_ip_address,
                "spot_price": lc.spot_price,
            });
            results.push(ExtractedResource {
                name: lc.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_autoscaling_policy
// ---------------------------------------------------------------------------

pub struct AwsAutoscalingPolicyConverter {
    service: Arc<AutoScalingService>,
}

impl AwsAutoscalingPolicyConverter {
    pub fn new(service: Arc<AutoScalingService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAutoscalingPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_autoscaling_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_autoscaling_group"]
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

impl AwsAutoscalingPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: autoscaling_gen::ScalingPolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_autoscaling_policy", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let group_name = model.autoscaling_group_name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:autoscaling:{}:{}:scalingPolicy:{}:autoScalingGroupName/{}:policyName/{}",
                region,
                ctx.default_account_id,
                uuid::Uuid::new_v4(),
                group_name,
                name
            )
        });

        let view = ScalingPolicyView {
            name: name.clone(),
            arn,
            group_name,
            policy_type: model.policy_type,
            adjustment_type: model.adjustment_type,
            scaling_adjustment: optional_i64(attrs, "scaling_adjustment").map(|v| v as i32),
            cooldown: optional_i64(attrs, "cooldown").map(|v| v as i32),
            min_adjustment_magnitude: optional_i64(attrs, "min_adjustment_magnitude")
                .map(|v| v as i32),
        };

        let mut state_view = minimal_asg_state_view();
        state_view.policies.insert(name, view);
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
        for pol in view.policies.values() {
            let attrs = serde_json::json!({
                "id": pol.name,
                "name": pol.name,
                "arn": pol.arn,
                "autoscaling_group_name": pol.group_name,
                "policy_type": pol.policy_type,
                "adjustment_type": pol.adjustment_type,
                "scaling_adjustment": pol.scaling_adjustment,
                "cooldown": pol.cooldown,
            });
            results.push(ExtractedResource {
                name: pol.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_autoscaling_schedule
// ---------------------------------------------------------------------------

pub struct AwsAutoscalingScheduleConverter {
    service: Arc<AutoScalingService>,
}

impl AwsAutoscalingScheduleConverter {
    pub fn new(service: Arc<AutoScalingService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAutoscalingScheduleConverter {
    fn resource_type(&self) -> &str {
        "aws_autoscaling_schedule"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_autoscaling_group"]
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

impl AwsAutoscalingScheduleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: autoscaling_gen::ScheduledActionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_autoscaling_schedule", e))?;

        let attrs = &instance.attributes;
        let name = model.scheduled_action_name.clone();
        let group_name = model.autoscaling_group_name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:autoscaling:{}:{}:scheduledUpdateGroupAction:{}",
                region, ctx.default_account_id, name
            )
        });

        let view = ScheduledActionView {
            name: name.clone(),
            arn,
            group_name,
            desired_capacity: optional_i64(attrs, "desired_capacity").map(|v| v as i32),
            min_size: optional_i64(attrs, "min_size").map(|v| v as i32),
            max_size: optional_i64(attrs, "max_size").map(|v| v as i32),
            start_time: model.start_time,
            end_time: model.end_time,
            recurrence: model.recurrence,
            time_zone: model.time_zone,
        };

        let mut state_view = minimal_asg_state_view();
        state_view.scheduled_actions.insert(name, view);
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
        for sa in view.scheduled_actions.values() {
            let attrs = serde_json::json!({
                "id": sa.name,
                "scheduled_action_name": sa.name,
                "arn": sa.arn,
                "autoscaling_group_name": sa.group_name,
                "desired_capacity": sa.desired_capacity,
                "min_size": sa.min_size,
                "max_size": sa.max_size,
                "start_time": sa.start_time,
                "end_time": sa.end_time,
                "recurrence": sa.recurrence,
            });
            results.push(ExtractedResource {
                name: sa.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_autoscaling_attachment
// ---------------------------------------------------------------------------

pub struct AwsAutoscalingAttachmentConverter {
    service: Arc<AutoScalingService>,
}

impl AwsAutoscalingAttachmentConverter {
    pub fn new(service: Arc<AutoScalingService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAutoscalingAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_autoscaling_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_autoscaling_group"]
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Attachments are reconstructed by enumerating each group's
        // `attached_load_balancers` / `attached_target_groups` lists, which
        // the group converter already round-trips. Surface nothing here to
        // avoid double-counting.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsAutoscalingAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: autoscaling_gen::AutoscalingAttachmentTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_autoscaling_attachment", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(group) = snapshot.groups.get_mut(&model.autoscaling_group_name) {
            if let Some(elb) = model.elb {
                if !group.attached_load_balancers.contains(&elb) {
                    group.attached_load_balancers.push(elb);
                }
            }
            if let Some(tg) = model.lb_target_group_arn {
                if !group.attached_target_groups.contains(&tg) {
                    group.attached_target_groups.push(tg);
                }
            }
            self.service
                .restore(&ctx.default_account_id, &region, snapshot)
                .await?;
        } else {
            warnings.push(format!(
                "aws_autoscaling_attachment: autoscaling group '{}' not found; attachment ignored",
                model.autoscaling_group_name
            ));
        }

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_autoscaling_group_tag
// ---------------------------------------------------------------------------

pub struct AwsAutoscalingGroupTagConverter {
    service: Arc<AutoScalingService>,
}

impl AwsAutoscalingGroupTagConverter {
    pub fn new(service: Arc<AutoScalingService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAutoscalingGroupTagConverter {
    fn resource_type(&self) -> &str {
        "aws_autoscaling_group_tag"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_autoscaling_group"]
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Tags round-trip through their parent group's `tags` collection.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsAutoscalingGroupTagConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: autoscaling_gen::AutoscalingGroupTagTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_autoscaling_group_tag", e))?;

        // `tag` is a single nested block: pull `key`/`value`/`propagate_at_launch`
        // out by hand. Both flat `tag.0.key` style (when TF projects it as a
        // list) and the JSON-array form are supported.
        let tag_block = instance
            .attributes
            .get("tag")
            .and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .or_else(|| instance.attributes.get("tag"));
        let (key, value, propagate) = match tag_block {
            Some(t) => (
                t.get("key").and_then(|v| v.as_str()).map(|s| s.to_string()),
                t.get("value")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                t.get("propagate_at_launch")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
            ),
            None => (None, None, false),
        };

        let mut warnings = vec![];
        let (Some(key), Some(value)) = (key, value) else {
            warnings.push(
                "aws_autoscaling_group_tag: `tag.key` and `tag.value` are required; tag ignored"
                    .to_string(),
            );
            return Ok(ConversionResult { region, warnings });
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if let Some(group) = snapshot.groups.get_mut(&model.autoscaling_group_name) {
            // Replace any existing tag with the same key on the group.
            group.tags.retain(|t| t.key != key);
            group.tags.push(TagView {
                key,
                value,
                resource_id: model.autoscaling_group_name.clone(),
                resource_type: "auto-scaling-group".to_string(),
                propagate_at_launch: propagate,
            });
            self.service
                .restore(&ctx.default_account_id, &region, snapshot)
                .await?;
        } else {
            warnings.push(format!(
                "aws_autoscaling_group_tag: autoscaling group '{}' not found; tag ignored",
                model.autoscaling_group_name
            ));
        }

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_autoscaling_lifecycle_hook
// ---------------------------------------------------------------------------

pub struct AwsAutoscalingLifecycleHookConverter {
    service: Arc<AutoScalingService>,
}

impl AwsAutoscalingLifecycleHookConverter {
    pub fn new(service: Arc<AutoScalingService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAutoscalingLifecycleHookConverter {
    fn resource_type(&self) -> &str {
        "aws_autoscaling_lifecycle_hook"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_autoscaling_group"]
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

impl AwsAutoscalingLifecycleHookConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let attrs = &instance.attributes;
        let model: autoscaling_gen::LifecycleHookTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_autoscaling_lifecycle_hook", e))?;

        let view = LifecycleHookView {
            name: model.name.clone(),
            group_name: model.autoscaling_group_name,
            lifecycle_transition: model.lifecycle_transition,
            notification_target_arn: model.notification_target_arn,
            role_arn: model.role_arn,
            notification_metadata: model.notification_metadata,
            heartbeat_timeout: optional_i64(attrs, "heartbeat_timeout").map(|v| v as i32),
            default_result: model.default_result,
        };

        let mut state_view = minimal_asg_state_view();
        state_view.lifecycle_hooks.insert(model.name, view);
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
        for hook in view.lifecycle_hooks.values() {
            let attrs = serde_json::json!({
                "id": hook.name,
                "name": hook.name,
                "autoscaling_group_name": hook.group_name,
                "lifecycle_transition": hook.lifecycle_transition,
                "notification_target_arn": hook.notification_target_arn,
                "role_arn": hook.role_arn,
                "notification_metadata": hook.notification_metadata,
                "heartbeat_timeout": hook.heartbeat_timeout,
                "default_result": hook.default_result,
            });
            results.push(ExtractedResource {
                name: hook.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_autoscaling_notification
// ---------------------------------------------------------------------------

pub struct AwsAutoscalingNotificationConverter {
    service: Arc<AutoScalingService>,
}

impl AwsAutoscalingNotificationConverter {
    pub fn new(service: Arc<AutoScalingService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAutoscalingNotificationConverter {
    fn resource_type(&self) -> &str {
        "aws_autoscaling_notification"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_autoscaling_group"]
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Notification configurations round-trip through each group's
        // `notification_configurations` list. Avoid emitting duplicates here.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsAutoscalingNotificationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let attrs = &instance.attributes;
        let model: autoscaling_gen::AutoscalingNotificationTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_autoscaling_notification", e))?;

        let group_names = extract_string_array(attrs, "group_names");
        let notification_types = extract_string_array(attrs, "notifications");

        let mut warnings = vec![];
        if group_names.is_empty() {
            warnings.push(
                "aws_autoscaling_notification: `group_names` is empty; notification ignored"
                    .to_string(),
            );
            return Ok(ConversionResult { region, warnings });
        }

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        for group_name in &group_names {
            let Some(group) = snapshot.groups.get_mut(group_name) else {
                warnings.push(format!(
                    "aws_autoscaling_notification: autoscaling group '{}' not found; skipped",
                    group_name
                ));
                continue;
            };
            for notification_type in &notification_types {
                let already = group.notification_configurations.iter().any(|n| {
                    n.notification_type == *notification_type && n.topic_arn == model.topic_arn
                });
                if already {
                    continue;
                }
                group
                    .notification_configurations
                    .push(NotificationConfigView {
                        group_name: group_name.clone(),
                        notification_type: notification_type.clone(),
                        topic_arn: model.topic_arn.clone(),
                    });
            }
        }
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_autoscaling_traffic_source_attachment
// ---------------------------------------------------------------------------

pub struct AwsAutoscalingTrafficSourceAttachmentConverter {
    service: Arc<AutoScalingService>,
}

impl AwsAutoscalingTrafficSourceAttachmentConverter {
    pub fn new(service: Arc<AutoScalingService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAutoscalingTrafficSourceAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_autoscaling_traffic_source_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_autoscaling_group"]
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Traffic-source attachments are reconstructed alongside their parent
        // group's `attached_target_groups`. Avoid duplicate extraction here.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsAutoscalingTrafficSourceAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let attrs = &instance.attributes;
        let model: autoscaling_gen::AutoscalingTrafficSourceAttachmentTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_autoscaling_traffic_source_attachment", e)
            })?;

        // `traffic_source` is a single nested block: `{ identifier, type }`.
        let ts_block = attrs
            .get("traffic_source")
            .and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .or_else(|| attrs.get("traffic_source"));
        let identifier = ts_block
            .and_then(|t| t.get("identifier"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mut warnings = vec![];
        let Some(identifier) = identifier else {
            warnings.push(
                "aws_autoscaling_traffic_source_attachment: `traffic_source.identifier` is required; attachment ignored"
                    .to_string(),
            );
            return Ok(ConversionResult { region, warnings });
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if let Some(group) = snapshot.groups.get_mut(&model.autoscaling_group_name) {
            if !group.attached_target_groups.contains(&identifier) {
                group.attached_target_groups.push(identifier);
            }
            self.service
                .restore(&ctx.default_account_id, &region, snapshot)
                .await?;
        } else {
            warnings.push(format!(
                "aws_autoscaling_traffic_source_attachment: autoscaling group '{}' not found; attachment ignored",
                model.autoscaling_group_name
            ));
        }

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn minimal_asg_state_view() -> AutoScalingStateView {
    AutoScalingStateView {
        groups: HashMap::new(),
        launch_configs: HashMap::new(),
        policies: HashMap::new(),
        lifecycle_hooks: HashMap::new(),
        scheduled_actions: HashMap::new(),
        activities: vec![],
    }
}

fn extract_string_array(attrs: &serde_json::Value, key: &str) -> Vec<String> {
    attrs
        .get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}
