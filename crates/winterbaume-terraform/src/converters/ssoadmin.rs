//! Terraform converters for SSO Admin (IAM Identity Center) resources.
//!
//! `PermissionSetTfModel` and `AccountAssignmentTfModel` are generated
//! from `specs/ssoadmin.toml`. The synthesised permission-set ARN
//! template, the `id` fallback, and the `target_type` default
//! ("AWS_ACCOUNT") are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_ssoadmin::SsoAdminService;
use winterbaume_ssoadmin::views::{AccountAssignmentView, PermissionSetView, SsoAdminStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::ssoadmin as ssoadmin_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_ssoadmin_permission_set
// ---------------------------------------------------------------------------

/// Converts `aws_ssoadmin_permission_set` Terraform resources to/from SSO Admin state.
pub struct AwsSsoadminPermissionSetConverter {
    service: Arc<SsoAdminService>,
}

impl AwsSsoadminPermissionSetConverter {
    pub fn new(service: Arc<SsoAdminService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsoadminPermissionSetConverter {
    fn resource_type(&self) -> &str {
        "aws_ssoadmin_permission_set"
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

impl AwsSsoadminPermissionSetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssoadmin_gen::PermissionSetTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ssoadmin_permission_set", e))?;

        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:sso:::permissionSet/ssoins-0123456789abcdef/{}",
                model.name.replace(' ', "-").to_lowercase()
            )
        });

        let ps_view = PermissionSetView {
            permission_set_arn: arn.clone(),
            name: model.name,
            description: model.description,
            session_duration: model.session_duration,
            relay_state: model.relay_state,
            inline_policy: None,
            managed_policies: vec![],
            customer_managed_policies: vec![],
            tags: model.tags,
            created_date: 0.0,
        };

        let mut ps_map = HashMap::new();
        ps_map.insert(arn, ps_view);

        let view = SsoAdminStateView {
            permission_sets: ps_map,
            account_assignments: vec![],
            assignment_statuses: HashMap::new(),
        };

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
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let resources = snapshot
            .permission_sets
            .values()
            .map(|ps| {
                let mut attrs: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                attrs.insert(
                    "arn".into(),
                    serde_json::Value::String(ps.permission_set_arn.clone()),
                );
                attrs.insert(
                    "id".into(),
                    serde_json::Value::String(ps.permission_set_arn.clone()),
                );
                attrs.insert("name".into(), serde_json::Value::String(ps.name.clone()));
                if let Some(desc) = &ps.description {
                    attrs.insert(
                        "description".into(),
                        serde_json::Value::String(desc.clone()),
                    );
                }
                if let Some(sd) = &ps.session_duration {
                    attrs.insert(
                        "session_duration".into(),
                        serde_json::Value::String(sd.clone()),
                    );
                }
                if let Some(rs) = &ps.relay_state {
                    attrs.insert("relay_state".into(), serde_json::Value::String(rs.clone()));
                }
                let tags_obj: serde_json::Map<_, _> = ps
                    .tags
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                    .collect();
                attrs.insert("tags".into(), serde_json::Value::Object(tags_obj));
                ExtractedResource {
                    name: ps.permission_set_arn.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: serde_json::Value::Object(attrs),
                }
            })
            .collect();
        Ok(resources)
    }
}

// ---------------------------------------------------------------------------
// aws_ssoadmin_account_assignment
// ---------------------------------------------------------------------------

/// Converts `aws_ssoadmin_account_assignment` Terraform resources to/from SSO Admin state.
pub struct AwsSsoadminAccountAssignmentConverter {
    service: Arc<SsoAdminService>,
}

impl AwsSsoadminAccountAssignmentConverter {
    pub fn new(service: Arc<SsoAdminService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsoadminAccountAssignmentConverter {
    fn resource_type(&self) -> &str {
        "aws_ssoadmin_account_assignment"
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

impl AwsSsoadminAccountAssignmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssoadmin_gen::AccountAssignmentTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ssoadmin_account_assignment", e))?;

        let principal_type = model.principal_type.unwrap_or_else(|| "USER".to_string());
        let _target_type = model
            .target_type
            .unwrap_or_else(|| "AWS_ACCOUNT".to_string());

        let assignment_view = AccountAssignmentView {
            account_id: model.target_id,
            permission_set_arn: model.permission_set_arn,
            principal_type,
            principal_id: model.principal_id,
        };

        let view = SsoAdminStateView {
            permission_sets: HashMap::new(),
            account_assignments: vec![assignment_view],
            assignment_statuses: HashMap::new(),
        };

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
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let resources = snapshot
            .account_assignments
            .iter()
            .map(|a| {
                let id = format!(
                    "{}/{}/{}/{}",
                    a.principal_id, a.principal_type, a.permission_set_arn, a.account_id
                );
                let mut attrs: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                attrs.insert("id".into(), serde_json::Value::String(id.clone()));
                attrs.insert(
                    "permission_set_arn".into(),
                    serde_json::Value::String(a.permission_set_arn.clone()),
                );
                attrs.insert(
                    "principal_id".into(),
                    serde_json::Value::String(a.principal_id.clone()),
                );
                attrs.insert(
                    "principal_type".into(),
                    serde_json::Value::String(a.principal_type.clone()),
                );
                attrs.insert(
                    "target_id".into(),
                    serde_json::Value::String(a.account_id.clone()),
                );
                attrs.insert(
                    "target_type".into(),
                    serde_json::Value::String("AWS_ACCOUNT".to_string()),
                );
                ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: serde_json::Value::Object(attrs),
                }
            })
            .collect();
        Ok(resources)
    }
}
