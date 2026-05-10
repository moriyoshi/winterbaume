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
    AccountView, OrgPolicyView, OrgTagView, OrganizationalUnitView, OrganizationsStateView,
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
