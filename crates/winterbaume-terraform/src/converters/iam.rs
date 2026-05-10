//! Terraform converters for IAM resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_iam::IamService;
use winterbaume_iam::views::{
    AccessKeyView, AccountPasswordPolicyView, AttachedPolicyView, GroupView, IamStateView,
    InstanceProfileView, LoginProfileView, ManagedPolicyView, OidcProviderView, PolicyVersionView,
    RoleView, SamlProviderView, ServerCertificateView, ServiceSpecificCredentialView,
    SigningCertificateView, SshPublicKeyView, UserView, VirtualMfaDeviceView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::iam as iam_gen;
use crate::util::{classify_deserialize_error, extract_tags, require_str};

/// Generate a fake IAM resource ID with the given prefix.
fn generate_id(prefix: &str) -> String {
    format!(
        "{}{}",
        prefix,
        uuid::Uuid::new_v4()
            .to_string()
            .replace("-", "")
            .chars()
            .take(17)
            .collect::<String>()
            .to_uppercase()
    )
}

// ---------------------------------------------------------------------------
// aws_iam_user
// ---------------------------------------------------------------------------

/// Converts `aws_iam_user` Terraform resources to/from IAM user state.
pub struct AwsIamUserConverter {
    service: Arc<IamService>,
}

impl AwsIamUserConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamUserConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_user"
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
impl AwsIamUserConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let model: iam_gen::IamUserTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_iam_user", e))?;

        let name = model.name.clone();
        let path = model.path.unwrap_or_else(|| "/".to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iam::{}:user{}{}",
                ctx.default_account_id, path, name
            )
        });
        let user_id = model.unique_id.unwrap_or_else(|| generate_id("AIDA"));

        let user_view = UserView {
            name: name.clone(),
            user_id,
            account_id: ctx.default_account_id.clone(),
            path,
            arn,
            create_date: None,
            tags: extract_tags(attrs),
            attached_policies: vec![],
            inline_policies: vec![],
            permissions_boundary: None,
        };

        let view = IamStateView {
            users: HashMap::from([(name, user_view)]),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        for user in view.users.values() {
            let attrs = serde_json::json!({
                "id": user.name,
                "name": user.name,
                "arn": user.arn,
                "unique_id": user.user_id,
                "path": user.path,
            });
            results.push(ExtractedResource {
                name: user.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_role
// ---------------------------------------------------------------------------

/// Converts `aws_iam_role` Terraform resources to/from IAM role state.
pub struct AwsIamRoleConverter {
    service: Arc<IamService>,
}

impl AwsIamRoleConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamRoleConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_role"
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
impl AwsIamRoleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let model: iam_gen::IamRoleTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_iam_role", e))?;

        let name = model.name.clone();
        let path = model.path.unwrap_or_else(|| "/".to_string());
        let assume_role_policy = model.assume_role_policy.unwrap_or_default();
        let description = model.description.unwrap_or_default();
        let max_session_duration = model.max_session_duration as i32;
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iam::{}:role{}{}",
                ctx.default_account_id, path, name
            )
        });
        let role_id = model.unique_id.unwrap_or_else(|| generate_id("AROA"));

        let role_view = RoleView {
            name: name.clone(),
            role_id,
            account_id: ctx.default_account_id.clone(),
            path,
            arn,
            assume_role_policy_document: assume_role_policy,
            description,
            create_date: None,
            max_session_duration,
            tags: extract_tags(attrs),
            attached_policies: vec![],
            inline_policies: vec![],
            permissions_boundary: model.permissions_boundary,
        };

        let view = IamStateView {
            roles: HashMap::from([(name, role_view)]),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        for role in view.roles.values() {
            let attrs = serde_json::json!({
                "id": role.name,
                "name": role.name,
                "arn": role.arn,
                "unique_id": role.role_id,
                "path": role.path,
                "assume_role_policy": role.assume_role_policy_document,
                "description": role.description,
                "max_session_duration": role.max_session_duration,
                "tags": role.tags,
                "tags_all": role.tags,
            });
            results.push(ExtractedResource {
                name: role.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_policy
// ---------------------------------------------------------------------------

/// Converts `aws_iam_policy` Terraform resources to/from IAM managed policy state.
pub struct AwsIamPolicyConverter {
    service: Arc<IamService>,
}

impl AwsIamPolicyConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_policy"
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
impl AwsIamPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        // `name` falls back to `policy_name`; the spec model only carries the
        // shared scalars, so name resolution stays hand-written.
        let name = require_str(attrs, "name", "aws_iam_policy")
            .or_else(|_| require_str(attrs, "policy_name", "aws_iam_policy"))?
            .to_string();
        let model: iam_gen::IamPolicyTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_iam_policy", e))?;

        let path = model.path.unwrap_or_else(|| "/".to_string());
        let description = model.description.unwrap_or_default();
        let document = model.policy.unwrap_or_default();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iam::{}:policy{}{}",
                ctx.default_account_id, path, name
            )
        });
        let policy_id = model.policy_id.unwrap_or_else(|| generate_id("ANPA"));

        let policy_view = ManagedPolicyView {
            policy_name: name,
            policy_id,
            arn: arn.clone(),
            path,
            default_version_id: "v1".to_string(),
            description,
            create_date: None,
            update_date: None,
            is_attachable: true,
            document: document.clone(),
            versions: vec![PolicyVersionView {
                version_id: "v1".to_string(),
                document,
                is_default_version: true,
                create_date: None,
            }],
            tags: extract_tags(attrs),
            attachment_count: 0,
        };

        let view = IamStateView {
            policies: HashMap::from([(arn, policy_view)]),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
            let attrs = serde_json::json!({
                "id": policy.arn,
                "name": policy.policy_name,
                "arn": policy.arn,
                "policy_id": policy.policy_id,
                "path": policy.path,
                "description": policy.description,
                "policy": policy.document,
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

// ---------------------------------------------------------------------------
// aws_iam_group
// ---------------------------------------------------------------------------

/// Converts `aws_iam_group` Terraform resources to/from IAM group state.
pub struct AwsIamGroupConverter {
    service: Arc<IamService>,
}

impl AwsIamGroupConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_group"
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
impl AwsIamGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let model: iam_gen::IamGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iam_group", e))?;

        let name = model.name.clone();
        let path = model.path.unwrap_or_else(|| "/".to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iam::{}:group{}{}",
                ctx.default_account_id, path, name
            )
        });
        let group_id = model.unique_id.unwrap_or_else(|| generate_id("AGPA"));

        let group_view = GroupView {
            name: name.clone(),
            group_id,
            account_id: ctx.default_account_id.clone(),
            path,
            arn,
            create_date: None,
            members: vec![],
            attached_policies: vec![],
            inline_policies: vec![],
        };

        let view = IamStateView {
            groups: HashMap::from([(name, group_view)]),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        for group in view.groups.values() {
            let attrs = serde_json::json!({
                "id": group.name,
                "name": group.name,
                "arn": group.arn,
                "unique_id": group.group_id,
                "path": group.path,
            });
            results.push(ExtractedResource {
                name: group.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_instance_profile
// ---------------------------------------------------------------------------

/// Converts `aws_iam_instance_profile` Terraform resources to/from IAM instance profile state.
pub struct AwsIamInstanceProfileConverter {
    service: Arc<IamService>,
}

impl AwsIamInstanceProfileConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamInstanceProfileConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_instance_profile"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_iam_role"]
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
impl AwsIamInstanceProfileConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let model: iam_gen::IamInstanceProfileTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_iam_instance_profile", e))?;

        let name = model.name.clone();
        let path = model.path.unwrap_or_else(|| "/".to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iam::{}:instance-profile{}{}",
                ctx.default_account_id, path, name
            )
        });
        let instance_profile_id = model.unique_id.unwrap_or_else(|| generate_id("AIPA"));

        let mut roles = vec![];
        if let Some(role) = model.role {
            roles.push(role);
        }

        let ip_view = InstanceProfileView {
            name: name.clone(),
            instance_profile_id,
            account_id: ctx.default_account_id.clone(),
            path,
            arn,
            create_date: None,
            roles,
            tags: extract_tags(attrs),
        };

        let view = IamStateView {
            instance_profiles: HashMap::from([(name, ip_view)]),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        for ip in view.instance_profiles.values() {
            let attrs = serde_json::json!({
                "id": ip.name,
                "name": ip.name,
                "arn": ip.arn,
                "unique_id": ip.instance_profile_id,
                "path": ip.path,
                "role": ip.roles.first().unwrap_or(&String::new()),
            });
            results.push(ExtractedResource {
                name: ip.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_role_policy_attachment
// ---------------------------------------------------------------------------

/// Converts `aws_iam_role_policy_attachment` Terraform resources.
///
/// This converter depends on `aws_iam_role` and `aws_iam_policy` being
/// processed first, as it modifies existing role state to add attachments.
/// It snapshots+modifies live state rather than ingesting a fresh view, so
/// it deliberately does not use a generated TfModel.
pub struct AwsIamRolePolicyAttachmentConverter {
    service: Arc<IamService>,
}

impl AwsIamRolePolicyAttachmentConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamRolePolicyAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_role_policy_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_iam_role", "aws_iam_policy"]
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

impl AwsIamRolePolicyAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let role_name = require_str(attrs, "role", "aws_iam_role_policy_attachment")?;
        let policy_arn = require_str(attrs, "policy_arn", "aws_iam_role_policy_attachment")?;

        // Snapshot current state, modify, restore
        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;

        let mut warnings = vec![];
        if let Some(role) = view.roles.get_mut(role_name) {
            let policy_name = policy_arn.rsplit('/').next().unwrap_or(policy_arn);
            // Avoid duplicate attachments
            if !role
                .attached_policies
                .iter()
                .any(|p| p.policy_arn == policy_arn)
            {
                role.attached_policies.push(AttachedPolicyView {
                    policy_name: policy_name.to_string(),
                    policy_arn: policy_arn.to_string(),
                });
            }
        } else {
            warnings.push(format!(
                "role '{}' not found in state; attachment skipped",
                role_name
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
            warnings,
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
        for role in view.roles.values() {
            for ap in &role.attached_policies {
                let attrs = serde_json::json!({
                    "id": format!("{}/{}", role.name, ap.policy_arn),
                    "policy_arn": ap.policy_arn,
                    "role": role.name,
                });
                results.push(ExtractedResource {
                    name: format!("{}/{}", role.name, ap.policy_arn),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_user_policy_attachment
// ---------------------------------------------------------------------------

/// Converts `aws_iam_user_policy_attachment` Terraform resources.
///
/// This converter depends on `aws_iam_user` and `aws_iam_policy` being
/// processed first, as it modifies existing user state to add attachments.
/// It snapshots+modifies live state rather than ingesting a fresh view, so
/// it deliberately does not use a generated TfModel.
pub struct AwsIamUserPolicyAttachmentConverter {
    service: Arc<IamService>,
}

impl AwsIamUserPolicyAttachmentConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamUserPolicyAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_user_policy_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_iam_user", "aws_iam_policy"]
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

impl AwsIamUserPolicyAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let user_name = require_str(attrs, "user", "aws_iam_user_policy_attachment")?;
        let policy_arn = require_str(attrs, "policy_arn", "aws_iam_user_policy_attachment")?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;

        let mut warnings = vec![];
        if let Some(user) = view.users.get_mut(user_name) {
            let policy_name = policy_arn.rsplit('/').next().unwrap_or(policy_arn);
            if !user
                .attached_policies
                .iter()
                .any(|p| p.policy_arn == policy_arn)
            {
                user.attached_policies.push(AttachedPolicyView {
                    policy_name: policy_name.to_string(),
                    policy_arn: policy_arn.to_string(),
                });
            }
        } else {
            warnings.push(format!(
                "user '{}' not found in state; attachment skipped",
                user_name
            ));
        }

        self.service
            .restore(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
            warnings,
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
        for user in view.users.values() {
            for ap in &user.attached_policies {
                let attrs = serde_json::json!({
                    "id": format!("{}/{}", user.name, ap.policy_arn),
                    "policy_arn": ap.policy_arn,
                    "user": user.name,
                });
                results.push(ExtractedResource {
                    name: format!("{}/{}", user.name, ap.policy_arn),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_openid_connect_provider
// ---------------------------------------------------------------------------

/// Converts `aws_iam_openid_connect_provider` Terraform resources.
///
/// `client_id_list` and `thumbprint_list` are read directly from the raw
/// `instance.attributes` because the spec format does not currently
/// express `Vec<String>` fields.
pub struct AwsIamOpenidConnectProviderConverter {
    service: Arc<IamService>,
}

impl AwsIamOpenidConnectProviderConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamOpenidConnectProviderConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_openid_connect_provider"
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

impl AwsIamOpenidConnectProviderConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let model: iam_gen::IamOpenidConnectProviderTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_iam_openid_connect_provider", e))?;

        // Extract host portion of the URL for ARN synthesis. Strip the
        // scheme prefix if present (Terraform usually stores the canonical
        // `https://` URL, but AWS-side ARNs use the host only).
        let arn = model.arn.unwrap_or_else(|| {
            let host = model
                .url
                .strip_prefix("https://")
                .or_else(|| model.url.strip_prefix("http://"))
                .unwrap_or(model.url.as_str());
            format!(
                "arn:aws:iam::{}:oidc-provider/{}",
                ctx.default_account_id, host
            )
        });

        let client_id_list: Vec<String> = attrs
            .get("client_id_list")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let thumbprint_list: Vec<String> = attrs
            .get("thumbprint_list")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let provider_view = OidcProviderView {
            arn: arn.clone(),
            url: model.url,
            client_id_list,
            thumbprint_list,
            create_date: None,
            tags: extract_tags(attrs),
        };

        let view = IamStateView {
            oidc_providers: HashMap::from([(arn, provider_view)]),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        for provider in view.oidc_providers.values() {
            let attrs = serde_json::json!({
                "id": provider.arn,
                "arn": provider.arn,
                "url": provider.url,
                "client_id_list": provider.client_id_list,
                "thumbprint_list": provider.thumbprint_list,
                "tags": provider.tags,
                "tags_all": provider.tags,
            });
            results.push(ExtractedResource {
                name: provider.arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_saml_provider
// ---------------------------------------------------------------------------

/// Converts `aws_iam_saml_provider` Terraform resources.
pub struct AwsIamSamlProviderConverter {
    service: Arc<IamService>,
}

impl AwsIamSamlProviderConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamSamlProviderConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_saml_provider"
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

impl AwsIamSamlProviderConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let model: iam_gen::IamSamlProviderTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_iam_saml_provider", e))?;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iam::{}:saml-provider/{}",
                ctx.default_account_id, model.name
            )
        });

        let provider_view = SamlProviderView {
            arn: arn.clone(),
            name: model.name,
            saml_metadata_document: model.saml_metadata_document.unwrap_or_default(),
            create_date: None,
            valid_until: None,
            tags: extract_tags(attrs),
        };

        let view = IamStateView {
            saml_providers: HashMap::from([(arn, provider_view)]),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        for provider in view.saml_providers.values() {
            let attrs = serde_json::json!({
                "id": provider.arn,
                "arn": provider.arn,
                "name": provider.name,
                "saml_metadata_document": provider.saml_metadata_document,
                "valid_until": provider.valid_until,
                "tags": provider.tags,
                "tags_all": provider.tags,
            });
            results.push(ExtractedResource {
                name: provider.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_virtual_mfa_device
// ---------------------------------------------------------------------------

/// Converts `aws_iam_virtual_mfa_device` Terraform resources.
///
/// `base32_string_seed` and `qr_code_png` are AWS-generated secrets with
/// no TF input counterpart. The converter sets them to empty strings; the
/// TF resource only stores the device serial number.
pub struct AwsIamVirtualMfaDeviceConverter {
    service: Arc<IamService>,
}

impl AwsIamVirtualMfaDeviceConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamVirtualMfaDeviceConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_virtual_mfa_device"
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

impl AwsIamVirtualMfaDeviceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let model: iam_gen::IamVirtualMfaDeviceTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_iam_virtual_mfa_device", e))?;

        let path = model.path.unwrap_or_else(|| "/".to_string());
        let serial_number = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iam::{}:mfa{}{}",
                ctx.default_account_id, path, model.virtual_mfa_device_name
            )
        });

        let device_view = VirtualMfaDeviceView {
            serial_number: serial_number.clone(),
            base32_string_seed: String::new(),
            qr_code_png: String::new(),
            user_name: None,
            enable_date: None,
            tags: extract_tags(attrs),
        };

        let view = IamStateView {
            virtual_mfa_devices: HashMap::from([(serial_number, device_view)]),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        for device in view.virtual_mfa_devices.values() {
            // Recover the bare device name from the serial-number ARN
            // (`arn:aws:iam::ACCT:mfa/PATH/NAME`) for the TF
            // `virtual_mfa_device_name` attribute.
            let name = device
                .serial_number
                .rsplit('/')
                .next()
                .unwrap_or(&device.serial_number)
                .to_string();
            let attrs = serde_json::json!({
                "id": device.serial_number,
                "arn": device.serial_number,
                "virtual_mfa_device_name": name,
                "tags": device.tags,
                "tags_all": device.tags,
            });
            results.push(ExtractedResource {
                name,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_server_certificate
// ---------------------------------------------------------------------------

/// Converts `aws_iam_server_certificate` Terraform resources.
///
/// `name` falls back to `name_prefix` (Terraform's auto-generated prefix
/// scheme); the spec only supports a single hcl key per field, so the
/// fallback stays hand-written.
pub struct AwsIamServerCertificateConverter {
    service: Arc<IamService>,
}

impl AwsIamServerCertificateConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamServerCertificateConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_server_certificate"
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

impl AwsIamServerCertificateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_iam_server_certificate")
            .or_else(|_| require_str(attrs, "name_prefix", "aws_iam_server_certificate"))?
            .to_string();
        let model: iam_gen::IamServerCertificateTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_iam_server_certificate", e))?;

        let path = model.path.unwrap_or_else(|| "/".to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iam::{}:server-certificate{}{}",
                ctx.default_account_id, path, name
            )
        });
        let server_certificate_id = generate_id("ASCA");

        let cert_view = ServerCertificateView {
            server_certificate_name: name.clone(),
            server_certificate_id,
            arn,
            path,
            certificate_body: model.certificate_body.unwrap_or_default(),
            certificate_chain: model.certificate_chain,
            private_key: model.private_key.unwrap_or_default(),
            upload_date: None,
            expiration: None,
            tags: extract_tags(attrs),
        };

        let view = IamStateView {
            server_certificates: HashMap::from([(name, cert_view)]),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        for cert in view.server_certificates.values() {
            let attrs = serde_json::json!({
                "id": cert.server_certificate_id,
                "name": cert.server_certificate_name,
                "arn": cert.arn,
                "path": cert.path,
                "certificate_body": cert.certificate_body,
                "certificate_chain": cert.certificate_chain,
                "upload_date": cert.upload_date,
                "expiration": cert.expiration,
                "tags": cert.tags,
                "tags_all": cert.tags,
            });
            results.push(ExtractedResource {
                name: cert.server_certificate_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_signing_certificate
// ---------------------------------------------------------------------------

/// Converts `aws_iam_signing_certificate` Terraform resources.
pub struct AwsIamSigningCertificateConverter {
    service: Arc<IamService>,
}

impl AwsIamSigningCertificateConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamSigningCertificateConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_signing_certificate"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_iam_user"]
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

impl AwsIamSigningCertificateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let model: iam_gen::IamSigningCertificateTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_iam_signing_certificate", e))?;

        let certificate_id = model.certificate_id.unwrap_or_else(|| generate_id("ASCA"));
        let status = model.status.unwrap_or_else(|| "Active".to_string());

        let cert_view = SigningCertificateView {
            user_name: model.user_name,
            certificate_id: certificate_id.clone(),
            certificate_body: model.certificate_body,
            status,
            upload_date: None,
        };

        let view = IamStateView {
            signing_certificates: HashMap::from([(certificate_id, cert_view)]),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        for cert in view.signing_certificates.values() {
            let attrs = serde_json::json!({
                "id": format!("{}:{}", cert.certificate_id, cert.user_name),
                "certificate_id": cert.certificate_id,
                "user_name": cert.user_name,
                "certificate_body": cert.certificate_body,
                "status": cert.status,
            });
            results.push(ExtractedResource {
                name: cert.certificate_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_service_specific_credential
// ---------------------------------------------------------------------------

/// Converts `aws_iam_service_specific_credential` Terraform resources.
///
/// `service_user_name` and `service_password` are AWS-generated and have
/// no TF input counterpart. The converter leaves them as empty strings.
pub struct AwsIamServiceSpecificCredentialConverter {
    service: Arc<IamService>,
}

impl AwsIamServiceSpecificCredentialConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamServiceSpecificCredentialConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_service_specific_credential"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_iam_user"]
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

impl AwsIamServiceSpecificCredentialConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let model: iam_gen::IamServiceSpecificCredentialTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_iam_service_specific_credential", e)
            })?;

        let credential_id = model
            .service_specific_credential_id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let status = model.status.unwrap_or_else(|| "Active".to_string());

        let cred_view = ServiceSpecificCredentialView {
            service_specific_credential_id: credential_id.clone(),
            user_name: model.user_name,
            service_name: model.service_name,
            service_user_name: String::new(),
            service_password: String::new(),
            status,
            create_date: None,
        };

        let view = IamStateView {
            service_specific_credentials: HashMap::from([(credential_id, cred_view)]),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        for cred in view.service_specific_credentials.values() {
            let attrs = serde_json::json!({
                "id": cred.service_specific_credential_id,
                "service_specific_credential_id": cred.service_specific_credential_id,
                "user_name": cred.user_name,
                "service_name": cred.service_name,
                "service_user_name": cred.service_user_name,
                "service_password": cred.service_password,
                "status": cred.status,
            });
            results.push(ExtractedResource {
                name: cred.service_specific_credential_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_user_ssh_key
// ---------------------------------------------------------------------------

/// Converts `aws_iam_user_ssh_key` Terraform resources.
///
/// `fingerprint` is an AWS-derived field with no TF input counterpart;
/// the converter leaves it empty.
pub struct AwsIamUserSshKeyConverter {
    service: Arc<IamService>,
}

impl AwsIamUserSshKeyConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamUserSshKeyConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_user_ssh_key"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_iam_user"]
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

impl AwsIamUserSshKeyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let model: iam_gen::IamUserSshKeyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iam_user_ssh_key", e))?;

        let ssh_public_key_id = model
            .ssh_public_key_id
            .unwrap_or_else(|| generate_id("APKA"));
        let status = model.status.unwrap_or_else(|| "Active".to_string());
        // `encoding` is a TF-only knob ("SSH" / "PEM") that selects which
        // encoding `extract` should emit; it is not stored on the view.
        let _encoding = model.encoding.unwrap_or_else(|| "SSH".to_string());

        let key_view = SshPublicKeyView {
            user_name: model.username,
            ssh_public_key_id: ssh_public_key_id.clone(),
            fingerprint: String::new(),
            ssh_public_key_body: model.public_key,
            status,
            upload_date: None,
        };

        let view = IamStateView {
            ssh_public_keys: HashMap::from([(ssh_public_key_id, key_view)]),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        for key in view.ssh_public_keys.values() {
            let attrs = serde_json::json!({
                "id": key.ssh_public_key_id,
                "ssh_public_key_id": key.ssh_public_key_id,
                "username": key.user_name,
                "public_key": key.ssh_public_key_body,
                "fingerprint": key.fingerprint,
                "status": key.status,
                "encoding": "SSH",
            });
            results.push(ExtractedResource {
                name: key.ssh_public_key_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_access_key
// ---------------------------------------------------------------------------

/// Converts `aws_iam_access_key` Terraform resources.
///
/// `pgp_key` is a TF-only encryption hint with no view counterpart and
/// is ignored on inject. `secret_access_key` is AWS-generated and not
/// represented in the TF input.
pub struct AwsIamAccessKeyConverter {
    service: Arc<IamService>,
}

impl AwsIamAccessKeyConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamAccessKeyConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_access_key"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_iam_user"]
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

impl AwsIamAccessKeyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let model: iam_gen::IamAccessKeyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iam_access_key", e))?;

        let access_key_id = model.id.unwrap_or_else(|| generate_id("AKIA"));
        let status = model.status.unwrap_or_else(|| "Active".to_string());

        let key_view = AccessKeyView {
            user_name: model.user,
            access_key_id: access_key_id.clone(),
            secret_access_key: String::new(),
            status,
            create_date: None,
        };

        let view = IamStateView {
            access_keys: HashMap::from([(access_key_id, key_view)]),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        for key in view.access_keys.values() {
            let attrs = serde_json::json!({
                "id": key.access_key_id,
                "user": key.user_name,
                "status": key.status,
                "create_date": key.create_date,
            });
            results.push(ExtractedResource {
                name: key.access_key_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_user_login_profile
// ---------------------------------------------------------------------------

/// Converts `aws_iam_user_login_profile` Terraform resources.
///
/// `password_length` is a TF-only knob (controls Terraform's generated
/// password length) with no view counterpart; the converter accepts it
/// but does not store it.
pub struct AwsIamUserLoginProfileConverter {
    service: Arc<IamService>,
}

impl AwsIamUserLoginProfileConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamUserLoginProfileConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_user_login_profile"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_iam_user"]
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

impl AwsIamUserLoginProfileConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let model: iam_gen::IamUserLoginProfileTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iam_user_login_profile", e))?;

        let user = model.user;
        // `password_length` is read but not persisted: winterbaume does
        // not synthesise a password.
        let _password_length = model.password_length;

        let profile_view = LoginProfileView {
            user_name: user.clone(),
            create_date: None,
            password_reset_required: model.password_reset_required,
        };

        let view = IamStateView {
            login_profiles: HashMap::from([(user, profile_view)]),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        for profile in view.login_profiles.values() {
            let attrs = serde_json::json!({
                "id": profile.user_name,
                "user": profile.user_name,
                "password_reset_required": profile.password_reset_required,
                "password_length": 20,
            });
            results.push(ExtractedResource {
                name: profile.user_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_account_password_policy
// ---------------------------------------------------------------------------

/// Converts `aws_iam_account_password_policy` Terraform resources.
///
/// This is a singleton: TF allows only one resource per account, and the
/// underlying state stores it on `IamStateView.account_password_policy`.
/// `max_password_age`, `password_reuse_prevention`, and `hard_expiry`
/// are optional integer/bool fields that the codegen cannot express
/// today, so the converter reads them directly from raw attributes.
pub struct AwsIamAccountPasswordPolicyConverter {
    service: Arc<IamService>,
}

impl AwsIamAccountPasswordPolicyConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamAccountPasswordPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_account_password_policy"
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

impl AwsIamAccountPasswordPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let model: iam_gen::IamAccountPasswordPolicyTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_iam_account_password_policy", e))?;

        let max_password_age = attrs
            .get("max_password_age")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);
        let password_reuse_prevention = attrs
            .get("password_reuse_prevention")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);
        let hard_expiry = attrs.get("hard_expiry").and_then(|v| v.as_bool());

        let policy_view = AccountPasswordPolicyView {
            minimum_password_length: model.minimum_password_length as i32,
            require_symbols: model.require_symbols,
            require_numbers: model.require_numbers,
            require_uppercase_characters: model.require_uppercase_characters,
            require_lowercase_characters: model.require_lowercase_characters,
            allow_users_to_change_password: model.allow_users_to_change_password,
            max_password_age,
            password_reuse_prevention,
            hard_expiry,
            expire_passwords: model.expire_passwords,
        };

        let view = IamStateView {
            account_password_policy: Some(policy_view),
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        if let Some(p) = &view.account_password_policy {
            let attrs = serde_json::json!({
                "id": "iam-account-password-policy",
                "minimum_password_length": p.minimum_password_length,
                "require_symbols": p.require_symbols,
                "require_numbers": p.require_numbers,
                "require_uppercase_characters": p.require_uppercase_characters,
                "require_lowercase_characters": p.require_lowercase_characters,
                "allow_users_to_change_password": p.allow_users_to_change_password,
                "expire_passwords": p.expire_passwords,
                "max_password_age": p.max_password_age,
                "password_reuse_prevention": p.password_reuse_prevention,
                "hard_expiry": p.hard_expiry,
            });
            results.push(ExtractedResource {
                name: "iam-account-password-policy".to_string(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iam_account_alias
// ---------------------------------------------------------------------------

/// Converts `aws_iam_account_alias` Terraform resources.
///
/// IAM stores account aliases as a `Vec<String>` singleton on
/// `IamStateView.account_aliases`. The merge logic (in `views.rs`)
/// deduplicates pushes, so injecting the same alias twice is idempotent.
/// Extract emits one TF resource per stored alias.
pub struct AwsIamAccountAliasConverter {
    service: Arc<IamService>,
}

impl AwsIamAccountAliasConverter {
    /// Create a new converter backed by the given IAM service.
    pub fn new(service: Arc<IamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIamAccountAliasConverter {
    fn resource_type(&self) -> &str {
        "aws_iam_account_alias"
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

impl AwsIamAccountAliasConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let model: iam_gen::IamAccountAliasTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iam_account_alias", e))?;

        let view = IamStateView {
            account_aliases: vec![model.account_alias],
            ..Default::default()
        };
        self.service
            .merge(&ctx.default_account_id, &ctx.default_region, view)
            .await?;

        Ok(ConversionResult {
            region: ctx.default_region.clone(),
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
        for alias in &view.account_aliases {
            let attrs = serde_json::json!({
                "id": alias,
                "account_alias": alias,
            });
            results.push(ExtractedResource {
                name: alias.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
