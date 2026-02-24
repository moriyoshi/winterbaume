//! Terraform converters for IAM resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_iam::IamService;
use winterbaume_iam::views::{
    AttachedPolicyView, GroupView, IamStateView, InstanceProfileView, ManagedPolicyView,
    PolicyVersionView, RoleView, UserView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_tags, optional_str, require_str};

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
        let name = require_str(attrs, "name", "aws_iam_user")?;
        let path = optional_str(attrs, "path").unwrap_or_else(|| "/".to_string());
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:iam::{}:user{}{}",
                ctx.default_account_id, path, name
            )
        });
        let user_id = optional_str(attrs, "unique_id").unwrap_or_else(|| generate_id("AIDA"));
        let tags = extract_tags(attrs);

        let user_view = UserView {
            name: name.to_string(),
            user_id,
            account_id: ctx.default_account_id.clone(),
            path,
            arn,
            create_date: None,
            tags,
            attached_policies: vec![],
            inline_policies: vec![],
            permissions_boundary: None,
        };

        let view = IamStateView {
            users: HashMap::from([(name.to_string(), user_view)]),
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
        let name = require_str(attrs, "name", "aws_iam_role")?;
        let path = optional_str(attrs, "path").unwrap_or_else(|| "/".to_string());
        let assume_role_policy = optional_str(attrs, "assume_role_policy").unwrap_or_default();
        let description = optional_str(attrs, "description").unwrap_or_default();
        let max_session_duration = attrs
            .get("max_session_duration")
            .and_then(|v| v.as_i64())
            .unwrap_or(3600) as i32;
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:iam::{}:role{}{}",
                ctx.default_account_id, path, name
            )
        });
        let role_id = optional_str(attrs, "unique_id").unwrap_or_else(|| generate_id("AROA"));
        let tags = extract_tags(attrs);
        let permissions_boundary = optional_str(attrs, "permissions_boundary");

        let role_view = RoleView {
            name: name.to_string(),
            role_id,
            account_id: ctx.default_account_id.clone(),
            path,
            arn,
            assume_role_policy_document: assume_role_policy,
            description,
            create_date: None,
            max_session_duration,
            tags,
            attached_policies: vec![],
            inline_policies: vec![],
            permissions_boundary,
        };

        let view = IamStateView {
            roles: HashMap::from([(name.to_string(), role_view)]),
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
        let name = require_str(attrs, "name", "aws_iam_policy")
            .or_else(|_| require_str(attrs, "policy_name", "aws_iam_policy"))?;
        let path = optional_str(attrs, "path").unwrap_or_else(|| "/".to_string());
        let description = optional_str(attrs, "description").unwrap_or_default();
        let document = optional_str(attrs, "policy").unwrap_or_default();
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:iam::{}:policy{}{}",
                ctx.default_account_id, path, name
            )
        });
        let policy_id = optional_str(attrs, "policy_id").unwrap_or_else(|| generate_id("ANPA"));
        let tags = extract_tags(attrs);

        let policy_view = ManagedPolicyView {
            policy_name: name.to_string(),
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
            tags,
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
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_iam_group")?;
        let path = optional_str(attrs, "path").unwrap_or_else(|| "/".to_string());
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:iam::{}:group{}{}",
                ctx.default_account_id, path, name
            )
        });
        let group_id = optional_str(attrs, "unique_id").unwrap_or_else(|| generate_id("AGPA"));

        let group_view = GroupView {
            name: name.to_string(),
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
            groups: HashMap::from([(name.to_string(), group_view)]),
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
        let name = require_str(attrs, "name", "aws_iam_instance_profile")?;
        let path = optional_str(attrs, "path").unwrap_or_else(|| "/".to_string());
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:iam::{}:instance-profile{}{}",
                ctx.default_account_id, path, name
            )
        });
        let instance_profile_id =
            optional_str(attrs, "unique_id").unwrap_or_else(|| generate_id("AIPA"));
        let tags = extract_tags(attrs);

        let mut roles = vec![];
        if let Some(role) = optional_str(attrs, "role") {
            roles.push(role);
        }

        let ip_view = InstanceProfileView {
            name: name.to_string(),
            instance_profile_id,
            account_id: ctx.default_account_id.clone(),
            path,
            arn,
            create_date: None,
            roles,
            tags,
        };

        let view = IamStateView {
            instance_profiles: HashMap::from([(name.to_string(), ip_view)]),
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
