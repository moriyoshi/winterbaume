//! Terraform converter for Inspector2 resources.
//!
//! `EnablerTfModel` is generated from `specs/inspector2.toml` as a marker;
//! the resource's only inputs are `account_ids` (Vec<String>) and
//! `resource_types` (Vec<String>), which the spec format does not express.
//! Both are read directly from `instance.attributes`.

use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_inspector2::Inspector2Service;
use winterbaume_inspector2::views::{
    AutoEnableView, DelegatedAdminView, FilterView, Inspector2StateView, MemberView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::inspector2 as inspector2_gen;
use crate::util::{classify_deserialize_error, extract_region};

pub struct AwsInspector2EnablerConverter {
    service: Arc<Inspector2Service>,
}

impl AwsInspector2EnablerConverter {
    pub fn new(service: Arc<Inspector2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsInspector2EnablerConverter {
    fn resource_type(&self) -> &str {
        "aws_inspector2_enabler"
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

impl AwsInspector2EnablerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let _account_ids = attrs.get("account_ids");

        let resource_types: HashSet<String> = attrs
            .get("resource_types")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let mut enabled_resource_types: HashMap<String, HashSet<String>> = HashMap::new();
        enabled_resource_types.insert(ctx.default_account_id.clone(), resource_types);
        let state_view = Inspector2StateView {
            enabled_resource_types,
            ..Default::default()
        };
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
        if view.enabled_resource_types.is_empty() {
            return Ok(vec![]);
        }
        let resource_types: Vec<String> = view
            .enabled_resource_types
            .values()
            .flat_map(|set| set.iter().cloned())
            .collect();
        let id = format!("{}:{}", ctx.default_account_id, ctx.default_region);
        let attrs = serde_json::json!({
            "id": id,
            "account_ids": [ctx.default_account_id],
            "resource_types": resource_types,
        });
        Ok(vec![ExtractedResource {
            name: id,
            account_id: ctx.default_account_id.clone(),
            region: ctx.default_region.clone(),
            attributes: attrs,
        }])
    }
}

// ---------------------------------------------------------------------------
// aws_inspector2_delegated_admin_account
// ---------------------------------------------------------------------------

pub struct AwsInspector2DelegatedAdminAccountConverter {
    service: Arc<Inspector2Service>,
}

impl AwsInspector2DelegatedAdminAccountConverter {
    pub fn new(service: Arc<Inspector2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsInspector2DelegatedAdminAccountConverter {
    fn resource_type(&self) -> &str {
        "aws_inspector2_delegated_admin_account"
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

impl AwsInspector2DelegatedAdminAccountConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: inspector2_gen::DelegatedAdminAccountTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_inspector2_delegated_admin_account", e)
            })?;

        let admin = DelegatedAdminView {
            account_id: model.account_id.clone(),
            status: "ENABLED".to_string(),
        };

        let mut state_view = Inspector2StateView::default();
        state_view
            .delegated_admin_accounts
            .insert(model.account_id, admin);
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
        for admin in view.delegated_admin_accounts.values() {
            let attrs = serde_json::json!({
                "id": admin.account_id,
                "account_id": admin.account_id,
            });
            results.push(ExtractedResource {
                name: admin.account_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_inspector2_filter
// ---------------------------------------------------------------------------

pub struct AwsInspector2FilterConverter {
    service: Arc<Inspector2Service>,
}

impl AwsInspector2FilterConverter {
    pub fn new(service: Arc<Inspector2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsInspector2FilterConverter {
    fn resource_type(&self) -> &str {
        "aws_inspector2_filter"
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

impl AwsInspector2FilterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: inspector2_gen::FilterTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_inspector2_filter", e))?;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:inspector2:{}:{}:owner/{}/filter/{}",
                region, ctx.default_account_id, ctx.default_account_id, model.name
            )
        });
        let now = chrono::Utc::now().to_rfc3339();
        let tags = if model.tags.is_empty() {
            None
        } else {
            Some(model.tags)
        };

        let filter = FilterView {
            arn: arn.clone(),
            name: model.name,
            action: model.action,
            description: model.description,
            created_at: now.clone(),
            updated_at: now,
            owner_id: ctx.default_account_id.clone(),
            tags,
        };

        let mut state_view = Inspector2StateView::default();
        state_view.filters.insert(arn, filter);
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
        for filter in view.filters.values() {
            let attrs = serde_json::json!({
                "id": filter.arn,
                "arn": filter.arn,
                "name": filter.name,
                "action": filter.action,
                "description": filter.description,
                "tags": filter.tags,
            });
            results.push(ExtractedResource {
                name: filter.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_inspector2_member_association
// ---------------------------------------------------------------------------

pub struct AwsInspector2MemberAssociationConverter {
    service: Arc<Inspector2Service>,
}

impl AwsInspector2MemberAssociationConverter {
    pub fn new(service: Arc<Inspector2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsInspector2MemberAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_inspector2_member_association"
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

impl AwsInspector2MemberAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: inspector2_gen::MemberAssociationTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_inspector2_member_association", e))?;

        let member = MemberView {
            account_id: model.account_id.clone(),
            relationship_status: "ENABLED".to_string(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        let mut state_view = Inspector2StateView::default();
        state_view.members.insert(model.account_id, member);
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
        for member in view.members.values() {
            let attrs = serde_json::json!({
                "id": member.account_id,
                "account_id": member.account_id,
                "relationship_status": member.relationship_status,
            });
            results.push(ExtractedResource {
                name: member.account_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_inspector2_organization_configuration
// ---------------------------------------------------------------------------

pub struct AwsInspector2OrganizationConfigurationConverter {
    service: Arc<Inspector2Service>,
}

impl AwsInspector2OrganizationConfigurationConverter {
    pub fn new(service: Arc<Inspector2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsInspector2OrganizationConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_inspector2_organization_configuration"
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

impl AwsInspector2OrganizationConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let _model: inspector2_gen::OrganizationConfigurationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_inspector2_organization_configuration", e)
            })?;

        // The Terraform schema has an `auto_enable` nested block with ec2/ecr/lambda/
        // lambda_code/code_repository booleans. Read raw.
        let block = attrs
            .get("auto_enable")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .cloned()
            .or_else(|| attrs.get("auto_enable").cloned())
            .unwrap_or(serde_json::Value::Null);

        let ec2 = block.get("ec2").and_then(|v| v.as_bool()).unwrap_or(false);
        let ecr = block.get("ecr").and_then(|v| v.as_bool()).unwrap_or(false);
        let lambda = block.get("lambda").and_then(|v| v.as_bool());
        let lambda_code = block.get("lambda_code").and_then(|v| v.as_bool());
        let code_repository = block.get("code_repository").and_then(|v| v.as_bool());

        let state_view = Inspector2StateView {
            auto_enable: Some(AutoEnableView {
                ec2,
                ecr,
                lambda,
                lambda_code,
                code_repository,
            }),
            ..Default::default()
        };
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
        let Some(auto) = view.auto_enable else {
            return Ok(vec![]);
        };
        let attrs = serde_json::json!({
            "id": ctx.default_account_id,
            "auto_enable": [{
                "ec2": auto.ec2,
                "ecr": auto.ecr,
                "lambda": auto.lambda,
                "lambda_code": auto.lambda_code,
                "code_repository": auto.code_repository,
            }],
        });
        Ok(vec![ExtractedResource {
            name: format!("org_config_{}", ctx.default_account_id),
            account_id: ctx.default_account_id.clone(),
            region: ctx.default_region.clone(),
            attributes: attrs,
        }])
    }
}
