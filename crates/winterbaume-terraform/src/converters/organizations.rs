//! Terraform converters for AWS Organizations resources.
//!
//! `AccountTfModel`, `OrganizationalUnitTfModel`, and
//! `OrganizationsPolicyTfModel` are generated from
//! `specs/organizations.toml`. The ARN templates, the random/UUID-derived
//! IDs, the parent_id / status / joined_method / aws_managed constants,
//! the `create_account_status_id`, and the tags side-channel for accounts
//! are wired up here. The management account ID is read straight from
//! `instance.attributes` because the model does not surface `account_id`.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_organizations::OrganizationsService;
use winterbaume_organizations::views::{
    AccountView, DelegatedAdministratorView, OrgPolicyView, OrgTagView, OrganizationView,
    OrganizationalUnitView, OrganizationsStateView, PolicyAttachmentView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::organizations as organizations_gen;
use crate::util::{classify_deserialize_error, extract_account_id, extract_region};

// ---------------------------------------------------------------------------
// aws_organizations_account
// ---------------------------------------------------------------------------

pub struct AwsOrganizationsAccountConverter {
    service: Arc<OrganizationsService>,
}

impl AwsOrganizationsAccountConverter {
    pub fn new(service: Arc<OrganizationsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOrganizationsAccountConverter {
    fn resource_type(&self) -> &str {
        "aws_organizations_account"
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

impl AwsOrganizationsAccountConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let mgmt_account_id = extract_account_id(attrs, &ctx.default_account_id);

        let model: organizations_gen::AccountTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_organizations_account", e))?;

        let member_account_id = model
            .id
            .unwrap_or_else(|| format!("{:012}", rand_account_id()));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:organizations::{}:account/o-example/{}",
                mgmt_account_id, member_account_id
            )
        });
        let parent_id = model.parent_id.unwrap_or_else(|| "r-root".to_string());
        let name = model.name.clone();

        let view = AccountView {
            id: member_account_id.clone(),
            arn,
            name: name.clone(),
            email: model.email,
            status: "ACTIVE".to_string(),
            joined_method: "CREATED".to_string(),
            joined_timestamp: chrono::Utc::now().to_rfc3339(),
            create_account_status_id: format!("car-{}", &uuid::Uuid::new_v4().to_string()[..8]),
            parent_id,
        };

        let mut state_view = minimal_org_state_view();
        state_view.accounts.insert(member_account_id, view);

        // Handle tags
        let tags = extract_org_tags(attrs);
        if !tags.is_empty() {
            state_view.tags.insert(name, tags);
        }

        self.service
            .merge(&mgmt_account_id, &region, state_view)
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
        for acct in view.accounts.values() {
            let attrs = serde_json::json!({
                "id": acct.id,
                "name": acct.name,
                "email": acct.email,
                "arn": acct.arn,
                "status": acct.status,
                "parent_id": acct.parent_id,
            });
            results.push(ExtractedResource {
                name: acct.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_organizations_organizational_unit
// ---------------------------------------------------------------------------

pub struct AwsOrganizationsOuConverter {
    service: Arc<OrganizationsService>,
}

impl AwsOrganizationsOuConverter {
    pub fn new(service: Arc<OrganizationsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOrganizationsOuConverter {
    fn resource_type(&self) -> &str {
        "aws_organizations_organizational_unit"
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

impl AwsOrganizationsOuConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let account_id = extract_account_id(attrs, &ctx.default_account_id);

        let model: organizations_gen::OrganizationalUnitTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_organizations_organizational_unit", e)
            })?;

        let ou_id = model
            .id
            .unwrap_or_else(|| format!("ou-{}", &uuid::Uuid::new_v4().to_string()[..12]));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:organizations::{}:ou/o-example/{}",
                account_id, ou_id
            )
        });

        let view = OrganizationalUnitView {
            id: ou_id.clone(),
            arn,
            name: model.name,
            parent_id: model.parent_id,
        };

        let mut state_view = minimal_org_state_view();
        state_view.ous.insert(ou_id, view);
        self.service.merge(&account_id, &region, state_view).await?;

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
        for ou in view.ous.values() {
            let attrs = serde_json::json!({
                "id": ou.id,
                "name": ou.name,
                "arn": ou.arn,
                "parent_id": ou.parent_id,
            });
            results.push(ExtractedResource {
                name: ou.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_organizations_policy
// ---------------------------------------------------------------------------

pub struct AwsOrganizationsPolicyConverter {
    service: Arc<OrganizationsService>,
}

impl AwsOrganizationsPolicyConverter {
    pub fn new(service: Arc<OrganizationsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOrganizationsPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_organizations_policy"
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

impl AwsOrganizationsPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let account_id = extract_account_id(attrs, &ctx.default_account_id);

        let model: organizations_gen::OrganizationsPolicyTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_organizations_policy", e))?;

        let policy_type = model
            .policy_type
            .unwrap_or_else(|| "SERVICE_CONTROL_POLICY".to_string());
        let description = model.description.unwrap_or_default();
        let policy_id = model
            .id
            .unwrap_or_else(|| format!("p-{}", &uuid::Uuid::new_v4().to_string()[..12]));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:organizations::{}:policy/o-example/{}/{}",
                account_id,
                policy_type.to_lowercase().replace('_', "-"),
                policy_id
            )
        });

        let view = OrgPolicyView {
            id: policy_id.clone(),
            arn,
            name: model.name,
            description,
            policy_type,
            content: model.content,
            aws_managed: false,
        };

        let mut state_view = minimal_org_state_view();
        state_view.policies.insert(policy_id, view);
        self.service.merge(&account_id, &region, state_view).await?;

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
            if pol.aws_managed {
                continue;
            }
            let attrs = serde_json::json!({
                "id": pol.id,
                "name": pol.name,
                "arn": pol.arn,
                "description": pol.description,
                "type": pol.policy_type,
                "content": pol.content,
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
// aws_organizations_organization
// ---------------------------------------------------------------------------

pub struct AwsOrganizationsOrganizationConverter {
    service: Arc<OrganizationsService>,
}

impl AwsOrganizationsOrganizationConverter {
    pub fn new(service: Arc<OrganizationsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOrganizationsOrganizationConverter {
    fn resource_type(&self) -> &str {
        "aws_organizations_organization"
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

impl AwsOrganizationsOrganizationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let account_id = extract_account_id(attrs, &ctx.default_account_id);

        let model: organizations_gen::OrganizationsOrganizationTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_organizations_organization", e))?;

        let id = model.id.unwrap_or_else(|| "o-example".to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!("arn:aws:organizations::{}:organization/{}", account_id, id)
        });
        let master_account_id = model
            .master_account_id
            .unwrap_or_else(|| account_id.clone());
        let master_account_email = model
            .master_account_email
            .unwrap_or_else(|| format!("master+{}@example.com", master_account_id));

        let mut state_view = minimal_org_state_view();
        state_view.organization = Some(OrganizationView {
            id,
            arn,
            master_account_id,
            master_account_email,
        });

        self.service.merge(&account_id, &region, state_view).await?;

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
        if let Some(org) = &view.organization {
            let attrs = serde_json::json!({
                "id": org.id,
                "arn": org.arn,
                "master_account_id": org.master_account_id,
                "master_account_email": org.master_account_email,
                "feature_set": "ALL",
            });
            results.push(ExtractedResource {
                name: org.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_organizations_delegated_administrator
// ---------------------------------------------------------------------------

pub struct AwsOrganizationsDelegatedAdministratorConverter {
    service: Arc<OrganizationsService>,
}

impl AwsOrganizationsDelegatedAdministratorConverter {
    pub fn new(service: Arc<OrganizationsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOrganizationsDelegatedAdministratorConverter {
    fn resource_type(&self) -> &str {
        "aws_organizations_delegated_administrator"
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

impl AwsOrganizationsDelegatedAdministratorConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let mgmt_account_id = extract_account_id(attrs, &ctx.default_account_id);

        let model: organizations_gen::OrganizationsDelegatedAdministratorTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_organizations_delegated_administrator", e)
            })?;

        let delegation_enabled_date = chrono::Utc::now().to_rfc3339();
        let delegated_account = AccountView {
            id: model.account_id.clone(),
            arn: format!(
                "arn:aws:organizations::{}:account/o-example/{}",
                mgmt_account_id, model.account_id
            ),
            name: format!("delegated-{}", model.account_id),
            email: format!("delegated+{}@example.com", model.account_id),
            status: "ACTIVE".to_string(),
            joined_method: "INVITED".to_string(),
            joined_timestamp: delegation_enabled_date.clone(),
            create_account_status_id: String::new(),
            parent_id: "r-root".to_string(),
        };

        let mut state_view = minimal_org_state_view();
        state_view
            .delegated_admins
            .push(DelegatedAdministratorView {
                account: delegated_account,
                delegation_enabled_date,
                services: vec![],
            });

        self.service
            .merge(&mgmt_account_id, &region, state_view)
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
        for da in &view.delegated_admins {
            let service_principal = da
                .services
                .first()
                .map(|s| s.service_principal.clone())
                .unwrap_or_default();
            let id = format!("{}/{}", da.account.id, service_principal);
            let attrs = serde_json::json!({
                "id": id,
                "account_id": da.account.id,
                "service_principal": service_principal,
                "delegation_enabled_date": da.delegation_enabled_date,
                "arn": da.account.arn,
                "email": da.account.email,
                "name": da.account.name,
                "status": da.account.status,
                "joined_method": da.account.joined_method,
                "joined_timestamp": da.account.joined_timestamp,
            });
            results.push(ExtractedResource {
                name: da.account.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_organizations_policy_attachment
// ---------------------------------------------------------------------------

pub struct AwsOrganizationsPolicyAttachmentConverter {
    service: Arc<OrganizationsService>,
}

impl AwsOrganizationsPolicyAttachmentConverter {
    pub fn new(service: Arc<OrganizationsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOrganizationsPolicyAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_organizations_policy_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_organizations_policy"]
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

impl AwsOrganizationsPolicyAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let account_id = extract_account_id(attrs, &ctx.default_account_id);

        let model: organizations_gen::OrganizationsPolicyAttachmentTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_organizations_policy_attachment", e)
            })?;

        let mut state_view = minimal_org_state_view();
        state_view.policy_attachments.push(PolicyAttachmentView {
            policy_id: model.policy_id,
            target_id: model.target_id,
        });
        self.service.merge(&account_id, &region, state_view).await?;

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
        for att in &view.policy_attachments {
            let id = format!("{}/{}", att.target_id, att.policy_id);
            let attrs = serde_json::json!({
                "id": id,
                "policy_id": att.policy_id,
                "target_id": att.target_id,
            });
            results.push(ExtractedResource {
                name: id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_organizations_resource_policy
// ---------------------------------------------------------------------------

pub struct AwsOrganizationsResourcePolicyConverter {
    service: Arc<OrganizationsService>,
}

impl AwsOrganizationsResourcePolicyConverter {
    pub fn new(service: Arc<OrganizationsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsOrganizationsResourcePolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_organizations_resource_policy"
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

impl AwsOrganizationsResourcePolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let account_id = extract_account_id(attrs, &ctx.default_account_id);

        let model: organizations_gen::OrganizationsResourcePolicyTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_organizations_resource_policy", e))?;

        let mut state_view = minimal_org_state_view();
        state_view.resource_policy = Some(model.content);
        self.service.merge(&account_id, &region, state_view).await?;

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
        if let Some(content) = &view.resource_policy {
            let id = "resource-policy".to_string();
            let attrs = serde_json::json!({
                "id": id,
                "content": content,
                "arn": format!("arn:aws:organizations::{}:resourcepolicy/o-example", ctx.default_account_id),
            });
            results.push(ExtractedResource {
                name: id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn minimal_org_state_view() -> OrganizationsStateView {
    OrganizationsStateView::default()
}

fn extract_org_tags(attrs: &serde_json::Value) -> Vec<OrgTagView> {
    let mut tags = Vec::new();
    if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
        for (k, v) in obj {
            if let Some(s) = v.as_str() {
                tags.push(OrgTagView {
                    key: k.clone(),
                    value: s.to_string(),
                });
            }
        }
    }
    tags
}

fn rand_account_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    100_000_000_000 + (seed % 900_000_000_000)
}
