//! Terraform converters for CloudFormation resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_cloudformation::CloudFormationService;
use winterbaume_cloudformation::views::{CloudFormationStateView, StackView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_cloudformation_stack
// ---------------------------------------------------------------------------

/// Converts `aws_cloudformation_stack` Terraform resources into CloudFormation state.
pub struct AwsCloudformationStackConverter {
    service: Arc<CloudFormationService>,
}

impl AwsCloudformationStackConverter {
    pub fn new(service: Arc<CloudFormationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudformationStackConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudformation_stack"
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

impl AwsCloudformationStackConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let stack_name = require_str(attrs, "name", "aws_cloudformation_stack")?.to_string();

        let stack_id = optional_str(attrs, "id").unwrap_or_else(|| {
            format!(
                "arn:aws:cloudformation:{region}:{}:stack/{stack_name}/{}",
                ctx.default_account_id,
                uuid::Uuid::new_v4()
            )
        });

        let template_body = optional_str(attrs, "template_body");
        let _tags_all = attrs.get("tags_all");
        let _capabilities = attrs.get("capabilities");
        let _on_failure = optional_str(attrs, "on_failure");
        let stack_status = optional_str(attrs, "timeout_in_minutes")
            .map(|_| "CREATE_COMPLETE")
            .unwrap_or("CREATE_COMPLETE")
            .to_string();

        // Parse parameters from a JSON object attribute
        let parameters = attrs
            .get("parameters")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| winterbaume_cloudformation::types::StackParameter {
                        parameter_key: k.clone(),
                        parameter_value: v.as_str().unwrap_or("").to_string(),
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        // Parse tags from a JSON object attribute
        let tags = attrs
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| winterbaume_cloudformation::types::StackTag {
                        key: k.clone(),
                        value: v.as_str().unwrap_or("").to_string(),
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let stack_view = StackView {
            stack_id,
            stack_name: stack_name.clone(),
            stack_status,
            creation_time: chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S.000Z")
                .to_string(),
            last_updated_time: None,
            deletion_time: None,
            description: None,
            template_body,
            stack_policy_body: None,
            parameters,
            outputs: vec![],
            tags,
            capabilities: vec![],
            resources: vec![],
            events: vec![],
            change_sets: vec![],
            exports: vec![],
            role_arn: optional_str(attrs, "iam_role_arn"),
            timeout_in_minutes: attrs
                .get("timeout_in_minutes")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32),
            disable_rollback: attrs
                .get("disable_rollback")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            enable_termination_protection: false,
        };

        let mut view = CloudFormationStateView::default();
        view.stacks.insert(stack_name, stack_view);
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
        let region = ctx.default_region.clone();
        let snap = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut result = Vec::new();

        for (name, stack) in &snap.stacks {
            if stack.stack_status == "DELETE_COMPLETE" {
                continue;
            }
            let tags_map: serde_json::Value = serde_json::json!(
                stack
                    .tags
                    .iter()
                    .map(|t| (t.key.clone(), t.value.clone()))
                    .collect::<std::collections::HashMap<String, String>>()
            );
            let outputs_map: serde_json::Value = serde_json::json!(
                stack
                    .outputs
                    .iter()
                    .map(|o| (o.output_key.clone(), o.output_value.clone()))
                    .collect::<std::collections::HashMap<String, String>>()
            );
            let mut attrs = serde_json::json!({
                "id": stack.stack_id,
                "name": name,
                "stack_status": stack.stack_status,
                "tags_all": tags_map,
                "outputs": outputs_map,
                "capabilities": stack.capabilities,
                "on_failure": null,
                "disable_rollback": stack.disable_rollback,
                "notification_arns": [],
                "iam_role_arn": stack.role_arn,
                "timeout_in_minutes": stack.timeout_in_minutes,
            });
            if let Some(tb) = &stack.template_body {
                attrs["template_body"] = serde_json::Value::String(tb.clone());
            }
            result.push(ExtractedResource {
                name: name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }

        Ok(result)
    }
}
