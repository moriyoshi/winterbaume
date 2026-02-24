//! Terraform converters for SSO Admin (IAM Identity Center) resources.

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
use crate::util::{extract_region, extract_tags, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let name = require_str(attrs, "name", "aws_ssoadmin_permission_set")?;
        let arn = optional_str(attrs, "arn")
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:sso:::permissionSet/ssoins-0123456789abcdef/{}",
                    name.replace(' ', "-").to_lowercase()
                )
            });

        let ps_view = PermissionSetView {
            permission_set_arn: arn.clone(),
            name: name.to_string(),
            description: optional_str(attrs, "description"),
            session_duration: optional_str(attrs, "session_duration"),
            relay_state: optional_str(attrs, "relay_state"),
            inline_policy: None,
            managed_policies: vec![],
            customer_managed_policies: vec![],
            tags: extract_tags(attrs),
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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let permission_set_arn = require_str(
            attrs,
            "permission_set_arn",
            "aws_ssoadmin_account_assignment",
        )?;
        let principal_id = require_str(attrs, "principal_id", "aws_ssoadmin_account_assignment")?;
        let principal_type =
            optional_str(attrs, "principal_type").unwrap_or_else(|| "USER".to_string());
        let target_id = require_str(attrs, "target_id", "aws_ssoadmin_account_assignment")?;
        let _target_type =
            optional_str(attrs, "target_type").unwrap_or_else(|| "AWS_ACCOUNT".to_string());

        let assignment_view = AccountAssignmentView {
            account_id: target_id.to_string(),
            permission_set_arn: permission_set_arn.to_string(),
            principal_type,
            principal_id: principal_id.to_string(),
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
