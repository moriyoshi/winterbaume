//! Terraform converters for SSO Admin (IAM Identity Center) resources.
//!
//! `PermissionSetTfModel` and `AccountAssignmentTfModel` are generated
//! from `specs/ssoadmin.toml`. The synthesised permission-set ARN
//! template, the `id` fallback, and the `target_type` default
//! ("AWS_ACCOUNT") are wired up here.
//!
//! The policy-attachment converters (`managed_policy_attachment`,
//! `customer_managed_policy_attachment`, `permission_set_inline_policy`)
//! mutate the relevant fields of the targeted `PermissionSetView` in the
//! ssoadmin state. The remaining resource types — `application`,
//! `application_access_scope`, `application_assignment`,
//! `application_assignment_configuration`,
//! `instance_access_control_attributes`,
//! `permissions_boundary_attachment`, `trusted_token_issuer` — have no
//! corresponding slot in `winterbaume_ssoadmin::state::SsoAdminState`
//! today, so their converters parse and validate the TF model but
//! emit a warning and perform a no-op inject. They are wired up so
//! that terraform plans referencing them succeed end-to-end without
//! losing fidelity in the parsed model.

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
            ..Default::default()
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
            account_assignments: vec![assignment_view],
            ..Default::default()
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

// ---------------------------------------------------------------------------
// aws_ssoadmin_managed_policy_attachment
// ---------------------------------------------------------------------------

/// Converts `aws_ssoadmin_managed_policy_attachment` Terraform resources
/// into the targeted permission set's `managed_policies` list in
/// ssoadmin state.
pub struct AwsSsoadminManagedPolicyAttachmentConverter {
    service: Arc<SsoAdminService>,
}

impl AwsSsoadminManagedPolicyAttachmentConverter {
    pub fn new(service: Arc<SsoAdminService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsoadminManagedPolicyAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_ssoadmin_managed_policy_attachment"
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

impl AwsSsoadminManagedPolicyAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssoadmin_gen::ManagedPolicyAttachmentTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_ssoadmin_managed_policy_attachment", e)
            })?;

        // Look up the existing permission set; if absent, synthesise a
        // minimal one so the attachment can still be recorded.
        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let existing = snapshot
            .permission_sets
            .get(&model.permission_set_arn)
            .cloned();
        let mut ps_view = existing.unwrap_or_else(|| PermissionSetView {
            permission_set_arn: model.permission_set_arn.clone(),
            name: model
                .permission_set_arn
                .rsplit('/')
                .next()
                .unwrap_or("permission-set")
                .to_string(),
            description: None,
            session_duration: None,
            relay_state: None,
            inline_policy: None,
            managed_policies: vec![],
            customer_managed_policies: vec![],
            tags: HashMap::new(),
            created_date: 0.0,
        });
        if !ps_view.managed_policies.contains(&model.managed_policy_arn) {
            ps_view.managed_policies.push(model.managed_policy_arn);
        }

        let mut ps_map = HashMap::new();
        ps_map.insert(model.permission_set_arn, ps_view);
        let view = SsoAdminStateView {
            permission_sets: ps_map,
            ..Default::default()
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
        let mut resources = Vec::new();
        for ps in snapshot.permission_sets.values() {
            for managed in &ps.managed_policies {
                let id = format!("{},{}", managed, ps.permission_set_arn);
                let mut attrs: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                attrs.insert("id".into(), serde_json::Value::String(id.clone()));
                attrs.insert(
                    "permission_set_arn".into(),
                    serde_json::Value::String(ps.permission_set_arn.clone()),
                );
                attrs.insert(
                    "managed_policy_arn".into(),
                    serde_json::Value::String(managed.clone()),
                );
                let name = managed.rsplit('/').next().unwrap_or(managed).to_string();
                attrs.insert(
                    "managed_policy_name".into(),
                    serde_json::Value::String(name),
                );
                resources.push(ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: serde_json::Value::Object(attrs),
                });
            }
        }
        Ok(resources)
    }
}

// ---------------------------------------------------------------------------
// aws_ssoadmin_customer_managed_policy_attachment
// ---------------------------------------------------------------------------

/// Converts `aws_ssoadmin_customer_managed_policy_attachment` resources
/// into the targeted permission set's `customer_managed_policies` list.
///
/// Reads the nested `customer_managed_policy_reference` block directly
/// from the raw TF attributes (the codegen model_only projection only
/// captures the scalar identity fields).
pub struct AwsSsoadminCustomerManagedPolicyAttachmentConverter {
    service: Arc<SsoAdminService>,
}

impl AwsSsoadminCustomerManagedPolicyAttachmentConverter {
    pub fn new(service: Arc<SsoAdminService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsoadminCustomerManagedPolicyAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_ssoadmin_customer_managed_policy_attachment"
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

impl AwsSsoadminCustomerManagedPolicyAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssoadmin_gen::CustomerManagedPolicyAttachmentTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_ssoadmin_customer_managed_policy_attachment", e)
            })?;

        // Parse the nested customer_managed_policy_reference block. It can
        // appear either as a single object (legacy `customer_managed_policy_reference {}`
        // block) or as a single-element list.
        let (policy_name, policy_path) = parse_cmpr(&instance.attributes);

        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let existing = snapshot
            .permission_sets
            .get(&model.permission_set_arn)
            .cloned();
        let mut ps_view = existing.unwrap_or_else(|| PermissionSetView {
            permission_set_arn: model.permission_set_arn.clone(),
            name: model
                .permission_set_arn
                .rsplit('/')
                .next()
                .unwrap_or("permission-set")
                .to_string(),
            description: None,
            session_duration: None,
            relay_state: None,
            inline_policy: None,
            managed_policies: vec![],
            customer_managed_policies: vec![],
            tags: HashMap::new(),
            created_date: 0.0,
        });
        if let Some(name) = policy_name {
            let entry = (name, policy_path);
            if !ps_view.customer_managed_policies.contains(&entry) {
                ps_view.customer_managed_policies.push(entry);
            }
        }

        let mut ps_map = HashMap::new();
        ps_map.insert(model.permission_set_arn, ps_view);
        let view = SsoAdminStateView {
            permission_sets: ps_map,
            ..Default::default()
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
        let mut resources = Vec::new();
        for ps in snapshot.permission_sets.values() {
            for (name, path) in &ps.customer_managed_policies {
                let id = format!(
                    "{},{},{}",
                    name,
                    path.as_deref().unwrap_or("/"),
                    ps.permission_set_arn,
                );
                let mut attrs: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                attrs.insert("id".into(), serde_json::Value::String(id.clone()));
                attrs.insert(
                    "permission_set_arn".into(),
                    serde_json::Value::String(ps.permission_set_arn.clone()),
                );
                let mut cmpr: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                cmpr.insert("name".into(), serde_json::Value::String(name.clone()));
                if let Some(p) = path {
                    cmpr.insert("path".into(), serde_json::Value::String(p.clone()));
                }
                attrs.insert(
                    "customer_managed_policy_reference".into(),
                    serde_json::Value::Array(vec![serde_json::Value::Object(cmpr)]),
                );
                resources.push(ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: serde_json::Value::Object(attrs),
                });
            }
        }
        Ok(resources)
    }
}

/// Extract `(name, path)` from a `customer_managed_policy_reference`
/// block, which TF state usually encodes as a single-element array of
/// objects but can also surface as a bare object.
fn parse_cmpr(attrs: &serde_json::Value) -> (Option<String>, Option<String>) {
    let obj = match attrs.get("customer_managed_policy_reference") {
        Some(serde_json::Value::Array(arr)) => arr.first(),
        Some(other) => Some(other),
        None => None,
    };
    let Some(serde_json::Value::Object(map)) = obj else {
        return (None, None);
    };
    let name = map.get("name").and_then(|v| v.as_str()).map(str::to_owned);
    let path = map.get("path").and_then(|v| v.as_str()).map(str::to_owned);
    (name, path)
}

// ---------------------------------------------------------------------------
// aws_ssoadmin_permission_set_inline_policy
// ---------------------------------------------------------------------------

/// Converts `aws_ssoadmin_permission_set_inline_policy` resources into
/// the targeted permission set's `inline_policy` slot.
pub struct AwsSsoadminPermissionSetInlinePolicyConverter {
    service: Arc<SsoAdminService>,
}

impl AwsSsoadminPermissionSetInlinePolicyConverter {
    pub fn new(service: Arc<SsoAdminService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsoadminPermissionSetInlinePolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_ssoadmin_permission_set_inline_policy"
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

impl AwsSsoadminPermissionSetInlinePolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssoadmin_gen::PermissionSetInlinePolicyTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_ssoadmin_permission_set_inline_policy", e)
            })?;

        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let existing = snapshot
            .permission_sets
            .get(&model.permission_set_arn)
            .cloned();
        let mut ps_view = existing.unwrap_or_else(|| PermissionSetView {
            permission_set_arn: model.permission_set_arn.clone(),
            name: model
                .permission_set_arn
                .rsplit('/')
                .next()
                .unwrap_or("permission-set")
                .to_string(),
            description: None,
            session_duration: None,
            relay_state: None,
            inline_policy: None,
            managed_policies: vec![],
            customer_managed_policies: vec![],
            tags: HashMap::new(),
            created_date: 0.0,
        });
        ps_view.inline_policy = Some(model.inline_policy);

        let mut ps_map = HashMap::new();
        ps_map.insert(model.permission_set_arn, ps_view);
        let view = SsoAdminStateView {
            permission_sets: ps_map,
            ..Default::default()
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
        let mut resources = Vec::new();
        for ps in snapshot.permission_sets.values() {
            if let Some(policy) = &ps.inline_policy {
                let id = format!("{},{}", ps.permission_set_arn, ps.permission_set_arn);
                let mut attrs: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                attrs.insert("id".into(), serde_json::Value::String(id.clone()));
                attrs.insert(
                    "permission_set_arn".into(),
                    serde_json::Value::String(ps.permission_set_arn.clone()),
                );
                attrs.insert(
                    "inline_policy".into(),
                    serde_json::Value::String(policy.clone()),
                );
                resources.push(ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: serde_json::Value::Object(attrs),
                });
            }
        }
        Ok(resources)
    }
}

// ---------------------------------------------------------------------------
// Warning-only converters
// ---------------------------------------------------------------------------
//
// The following resource types have no corresponding slot in the
// ssoadmin state machine in `winterbaume_ssoadmin::state`. The
// converters validate the TF-shaped projection, log a single warning,
// and otherwise no-op. Extract returns an empty vector.

macro_rules! ssoadmin_warning_only_converter {
    (
        struct_name = $struct_name:ident,
        resource_type = $resource_type:expr,
        model_type = $model_type:ident,
        warn_msg = $warn_msg:expr $(,)?
    ) => {
        pub struct $struct_name {
            #[allow(dead_code)]
            service: Arc<SsoAdminService>,
        }

        impl $struct_name {
            pub fn new(service: Arc<SsoAdminService>) -> Self {
                Self { service }
            }
        }

        impl TerraformResourceConverter for $struct_name {
            fn resource_type(&self) -> &str {
                $resource_type
            }

            fn inject<'a>(
                &'a self,
                instance: &'a ResourceInstance,
                ctx: &'a ConversionContext,
            ) -> Pin<
                Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>,
            > {
                Box::pin(async move { self.do_inject(instance, ctx).await })
            }

            fn extract<'a>(
                &'a self,
                _ctx: &'a ConversionContext,
            ) -> Pin<
                Box<
                    dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>>
                        + Send
                        + 'a,
                >,
            > {
                Box::pin(async move { Ok(vec![]) })
            }
        }

        impl $struct_name {
            async fn do_inject(
                &self,
                instance: &ResourceInstance,
                ctx: &ConversionContext,
            ) -> Result<ConversionResult, ConversionError> {
                let attrs = &instance.attributes;
                let region = extract_region(attrs, &ctx.default_region);
                let _model: ssoadmin_gen::$model_type = serde_json::from_value(attrs.clone())
                    .map_err(|e| classify_deserialize_error($resource_type, e))?;
                eprintln!("warning: {}: {}", $resource_type, $warn_msg);
                Ok(ConversionResult {
                    region,
                    warnings: vec![format!("{}: {}", $resource_type, $warn_msg)],
                })
            }
        }
    };
}

ssoadmin_warning_only_converter! {
    struct_name = AwsSsoadminApplicationConverter,
    resource_type = "aws_ssoadmin_application",
    model_type = ApplicationTfModel,
    warn_msg = "no state slot in winterbaume_ssoadmin; inject is a no-op",
}

ssoadmin_warning_only_converter! {
    struct_name = AwsSsoadminApplicationAccessScopeConverter,
    resource_type = "aws_ssoadmin_application_access_scope",
    model_type = ApplicationAccessScopeTfModel,
    warn_msg = "no state slot in winterbaume_ssoadmin; inject is a no-op",
}

ssoadmin_warning_only_converter! {
    struct_name = AwsSsoadminApplicationAssignmentConverter,
    resource_type = "aws_ssoadmin_application_assignment",
    model_type = ApplicationAssignmentTfModel,
    warn_msg = "no state slot in winterbaume_ssoadmin; inject is a no-op",
}

ssoadmin_warning_only_converter! {
    struct_name = AwsSsoadminApplicationAssignmentConfigurationConverter,
    resource_type = "aws_ssoadmin_application_assignment_configuration",
    model_type = ApplicationAssignmentConfigurationTfModel,
    warn_msg = "no state slot in winterbaume_ssoadmin; inject is a no-op",
}

ssoadmin_warning_only_converter! {
    struct_name = AwsSsoadminInstanceAccessControlAttributesConverter,
    resource_type = "aws_ssoadmin_instance_access_control_attributes",
    model_type = InstanceAccessControlAttributesTfModel,
    warn_msg = "no state slot in winterbaume_ssoadmin; inject is a no-op",
}

ssoadmin_warning_only_converter! {
    struct_name = AwsSsoadminPermissionsBoundaryAttachmentConverter,
    resource_type = "aws_ssoadmin_permissions_boundary_attachment",
    model_type = PermissionsBoundaryAttachmentTfModel,
    warn_msg = "no permissions_boundary slot in winterbaume_ssoadmin; inject is a no-op",
}

ssoadmin_warning_only_converter! {
    struct_name = AwsSsoadminTrustedTokenIssuerConverter,
    resource_type = "aws_ssoadmin_trusted_token_issuer",
    model_type = TrustedTokenIssuerTfModel,
    warn_msg = "no state slot in winterbaume_ssoadmin; inject is a no-op",
}
