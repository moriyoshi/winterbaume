//! Terraform converters for CloudFormation resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_cloudformation::CloudFormationService;
use winterbaume_cloudformation::types::RegisteredType;
use winterbaume_cloudformation::views::{
    CloudFormationStateView, StackInstanceView, StackSetView, StackView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::cloudformation as cloudformation_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cloudformation_gen::StackTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudformation_stack", e))?;

        let stack_name = model.name.clone();
        let stack_id = model.id.unwrap_or_else(|| {
            format!(
                "arn:aws:cloudformation:{region}:{}:stack/{stack_name}/{}",
                ctx.default_account_id,
                uuid::Uuid::new_v4()
            )
        });

        let attrs = &instance.attributes;
        let stack_status = "CREATE_COMPLETE".to_string();

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
            template_body: model.template_body,
            stack_policy_body: None,
            parameters,
            outputs: vec![],
            tags,
            capabilities: vec![],
            resources: vec![],
            events: vec![],
            change_sets: vec![],
            exports: vec![],
            role_arn: model.iam_role_arn,
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

// ---------------------------------------------------------------------------
// aws_cloudformation_stack_set
// ---------------------------------------------------------------------------

pub struct AwsCloudformationStackSetConverter {
    service: Arc<CloudFormationService>,
}

impl AwsCloudformationStackSetConverter {
    pub fn new(service: Arc<CloudFormationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudformationStackSetConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudformation_stack_set"
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

impl AwsCloudformationStackSetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cloudformation_gen::StackSetTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudformation_stack_set", e))?;

        let attrs = &instance.attributes;
        let stack_set_id = model
            .stack_set_id
            .or(model.id)
            .unwrap_or_else(|| format!("{}:{}", model.name, uuid::Uuid::new_v4()));
        let stack_set_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:cloudformation:{region}:{}:stackset/{}:{}",
                ctx.default_account_id, model.name, stack_set_id
            )
        });

        // Parameters/tags/capabilities are HCL list-of-blocks; read raw.
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
        let capabilities = attrs
            .get("capabilities")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let ss_view = StackSetView {
            stack_set_id,
            stack_set_name: model.name.clone(),
            stack_set_arn,
            status: "ACTIVE".to_string(),
            description: model.description,
            template_body: model.template_body,
            parameters,
            tags,
            capabilities,
            operations: vec![],
        };

        let mut view = CloudFormationStateView::default();
        view.stack_sets.insert(model.name, ss_view);
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
        let snap = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut result = vec![];
        for (name, ss) in &snap.stack_sets {
            let tags_map: serde_json::Value = serde_json::json!(
                ss.tags
                    .iter()
                    .map(|t| (t.key.clone(), t.value.clone()))
                    .collect::<std::collections::HashMap<String, String>>()
            );
            let attrs = serde_json::json!({
                "id": ss.stack_set_id,
                "name": name,
                "stack_set_id": ss.stack_set_id,
                "arn": ss.stack_set_arn,
                "description": ss.description,
                "template_body": ss.template_body,
                "tags": tags_map,
                "capabilities": ss.capabilities,
            });
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

// ---------------------------------------------------------------------------
// aws_cloudformation_stack_set_instance
// ---------------------------------------------------------------------------

pub struct AwsCloudformationStackSetInstanceConverter {
    service: Arc<CloudFormationService>,
}

impl AwsCloudformationStackSetInstanceConverter {
    pub fn new(service: Arc<CloudFormationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudformationStackSetInstanceConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudformation_stack_set_instance"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cloudformation_stack_set"]
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

impl AwsCloudformationStackSetInstanceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cloudformation_gen::StackSetInstanceTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_cloudformation_stack_set_instance", e)
            })?;

        let account = model
            .account_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let inst_region = model.region.unwrap_or_else(|| region.clone());

        // Look up the parent stack_set_id from the existing stack_sets state.
        let snap = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let stack_set_id = snap
            .stack_sets
            .get(&model.stack_set_name)
            .map(|ss| ss.stack_set_id.clone())
            .unwrap_or_default();

        let inst_view = StackInstanceView {
            stack_set_name: model.stack_set_name.clone(),
            account,
            region: inst_region,
            stack_id: model.stack_id,
            status: "CURRENT".to_string(),
            status_reason: None,
            stack_set_id,
            parameter_overrides: vec![],
        };

        let mut view = CloudFormationStateView::default();
        view.stack_instances.push(inst_view);
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
        let snap = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut result = vec![];
        for inst in &snap.stack_instances {
            let attrs = serde_json::json!({
                "id": format!("{},{},{}", inst.stack_set_name, inst.account, inst.region),
                "stack_set_name": inst.stack_set_name,
                "account_id": inst.account,
                "region": inst.region,
                "stack_id": inst.stack_id,
            });
            result.push(ExtractedResource {
                name: format!(
                    "{}_{}_{}",
                    inst.stack_set_name,
                    inst.account,
                    inst.region.replace('-', "_")
                ),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(result)
    }
}

// ---------------------------------------------------------------------------
// aws_cloudformation_stack_instances (bulk)
// ---------------------------------------------------------------------------

pub struct AwsCloudformationStackInstancesConverter {
    service: Arc<CloudFormationService>,
}

impl AwsCloudformationStackInstancesConverter {
    pub fn new(service: Arc<CloudFormationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudformationStackInstancesConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudformation_stack_instances"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_cloudformation_stack_set"]
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
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsCloudformationStackInstancesConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cloudformation_gen::StackInstancesTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudformation_stack_instances", e))?;

        let attrs = &instance.attributes;
        let accounts: Vec<String> = attrs
            .get("accounts")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_else(|| vec![ctx.default_account_id.clone()]);
        let regions: Vec<String> = attrs
            .get("regions")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_else(|| vec![region.clone()]);

        let snap = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let stack_set_id = snap
            .stack_sets
            .get(&model.stack_set_name)
            .map(|ss| ss.stack_set_id.clone())
            .unwrap_or_default();

        let mut view = CloudFormationStateView::default();
        for account in &accounts {
            for inst_region in &regions {
                view.stack_instances.push(StackInstanceView {
                    stack_set_name: model.stack_set_name.clone(),
                    account: account.clone(),
                    region: inst_region.clone(),
                    stack_id: None,
                    status: "CURRENT".to_string(),
                    status_reason: None,
                    stack_set_id: stack_set_id.clone(),
                    parameter_overrides: vec![],
                });
            }
        }
        self.service
            .merge(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_cloudformation_type
// ---------------------------------------------------------------------------

pub struct AwsCloudformationTypeConverter {
    service: Arc<CloudFormationService>,
}

impl AwsCloudformationTypeConverter {
    pub fn new(service: Arc<CloudFormationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsCloudformationTypeConverter {
    fn resource_type(&self) -> &str {
        "aws_cloudformation_type"
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

impl AwsCloudformationTypeConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: cloudformation_gen::TypeTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_cloudformation_type", e))?;

        let registered = RegisteredType {
            type_name: model.type_name,
            type_kind: model.type_kind.unwrap_or_else(|| "RESOURCE".to_string()),
            type_arn: model.arn,
            default_version_id: model.default_version_id,
            last_updated: Some(chrono::Utc::now().to_rfc3339()),
            description: model.description,
        };

        let mut view = CloudFormationStateView::default();
        view.registered_types.push(registered);
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
        let snap = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut result = vec![];
        for rt in &snap.registered_types {
            let attrs = serde_json::json!({
                "id": rt.type_arn,
                "type_name": rt.type_name,
                "type": rt.type_kind,
                "arn": rt.type_arn,
                "default_version_id": rt.default_version_id,
                "description": rt.description,
            });
            result.push(ExtractedResource {
                name: format!("{}_{}", rt.type_kind, rt.type_name.replace("::", "_")),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(result)
    }
}
