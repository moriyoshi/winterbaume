//! Terraform converter for RAM resources.
//!
//! `ResourceShareTfModel` is generated from `specs/ram.toml`. The ARN
//! template, the `status = "ACTIVE"` constant, the zero-valued
//! `creation_time`/`last_updated_time` placeholders, and the conversion
//! between the model's `HashMap<String, String>` tag bag and the view's
//! `Vec<TagView>` are wired up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_ram::RamService;
use winterbaume_ram::views::{
    RamStateView, ResourceShareAssociationView, ResourceShareInvitationView, ResourceShareView,
    TagView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::ram as ram_gen;
use crate::util::{classify_deserialize_error, extract_account_id, extract_region};

// ---------------------------------------------------------------------------
// aws_ram_resource_share
// ---------------------------------------------------------------------------

pub struct AwsRamResourceShareConverter {
    service: Arc<RamService>,
}

impl AwsRamResourceShareConverter {
    pub fn new(service: Arc<RamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRamResourceShareConverter {
    fn resource_type(&self) -> &str {
        "aws_ram_resource_share"
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

impl AwsRamResourceShareConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let account_id = extract_account_id(&instance.attributes, &ctx.default_account_id);
        let model: ram_gen::ResourceShareTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ram_resource_share", e))?;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ram:{}:{}:resource-share/{}",
                region, account_id, model.name
            )
        });

        let tags: Vec<TagView> = model
            .tags
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let rs_view = ResourceShareView {
            resource_share_arn: arn.clone(),
            name: model.name,
            owning_account_id: account_id.clone(),
            allow_external_principals: model.allow_external_principals,
            status: "ACTIVE".to_string(),
            creation_time: 0.0,
            last_updated_time: 0.0,
            tags,
        };

        let mut state_view = RamStateView::default();
        state_view.resource_shares.insert(arn, rs_view);
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
        for rs in view.resource_shares.values() {
            let tags_json: serde_json::Map<String, serde_json::Value> = rs
                .tags
                .iter()
                .map(|t| (t.key.clone(), serde_json::Value::String(t.value.clone())))
                .collect();
            let attrs = serde_json::json!({
                "id": rs.resource_share_arn,
                "arn": rs.resource_share_arn,
                "name": rs.name,
                "owning_account_id": rs.owning_account_id,
                "allow_external_principals": rs.allow_external_principals,
                "status": rs.status,
                "creation_time": rs.creation_time,
                "last_updated_time": rs.last_updated_time,
                "tags": tags_json,
            });
            results.push(ExtractedResource {
                name: rs.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ram_principal_association
// ---------------------------------------------------------------------------

pub struct AwsRamPrincipalAssociationConverter {
    service: Arc<RamService>,
}

impl AwsRamPrincipalAssociationConverter {
    pub fn new(service: Arc<RamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRamPrincipalAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_ram_principal_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ram_resource_share"]
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

impl AwsRamPrincipalAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let account_id = extract_account_id(attrs, &ctx.default_account_id);
        let model: ram_gen::RamPrincipalAssociationTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ram_principal_association", e))?;

        let share_name = model
            .resource_share_arn
            .rsplit('/')
            .next()
            .unwrap_or("share")
            .to_string();

        let view = ResourceShareAssociationView {
            resource_share_arn: model.resource_share_arn,
            resource_share_name: share_name,
            associated_entity: model.principal,
            association_type: "PRINCIPAL".to_string(),
            status: "ASSOCIATED".to_string(),
            creation_time: 0.0,
            last_updated_time: 0.0,
            external: false,
        };

        let mut state_view = RamStateView::default();
        state_view.associations.push(view);
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
        for a in view
            .associations
            .iter()
            .filter(|a| a.association_type == "PRINCIPAL")
        {
            let id = format!("{},{}", a.resource_share_arn, a.associated_entity);
            let attrs = serde_json::json!({
                "id": id,
                "resource_share_arn": a.resource_share_arn,
                "principal": a.associated_entity,
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
// aws_ram_resource_association
// ---------------------------------------------------------------------------

pub struct AwsRamResourceAssociationConverter {
    service: Arc<RamService>,
}

impl AwsRamResourceAssociationConverter {
    pub fn new(service: Arc<RamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRamResourceAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_ram_resource_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ram_resource_share"]
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

impl AwsRamResourceAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let account_id = extract_account_id(attrs, &ctx.default_account_id);
        let model: ram_gen::RamResourceAssociationTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_ram_resource_association", e))?;

        let share_name = model
            .resource_share_arn
            .rsplit('/')
            .next()
            .unwrap_or("share")
            .to_string();

        let view = ResourceShareAssociationView {
            resource_share_arn: model.resource_share_arn,
            resource_share_name: share_name,
            associated_entity: model.resource_arn,
            association_type: "RESOURCE".to_string(),
            status: "ASSOCIATED".to_string(),
            creation_time: 0.0,
            last_updated_time: 0.0,
            external: false,
        };

        let mut state_view = RamStateView::default();
        state_view.associations.push(view);
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
        for a in view
            .associations
            .iter()
            .filter(|a| a.association_type == "RESOURCE")
        {
            let id = format!("{},{}", a.resource_share_arn, a.associated_entity);
            let attrs = serde_json::json!({
                "id": id,
                "resource_share_arn": a.resource_share_arn,
                "resource_arn": a.associated_entity,
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
// aws_ram_resource_share_accepter
// ---------------------------------------------------------------------------

pub struct AwsRamResourceShareAccepterConverter {
    service: Arc<RamService>,
}

impl AwsRamResourceShareAccepterConverter {
    pub fn new(service: Arc<RamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRamResourceShareAccepterConverter {
    fn resource_type(&self) -> &str {
        "aws_ram_resource_share_accepter"
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

impl AwsRamResourceShareAccepterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let account_id = extract_account_id(attrs, &ctx.default_account_id);
        let model: ram_gen::RamResourceShareAccepterTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_ram_resource_share_accepter", e))?;

        let share_arn = model.share_arn;
        let share_name = share_arn.rsplit('/').next().unwrap_or("share").to_string();
        let invitation_arn = model.invitation_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ram:{}:{}:resource-share-invitation/{}",
                region,
                account_id,
                uuid::Uuid::new_v4()
            )
        });

        let view = ResourceShareInvitationView {
            invitation_arn,
            resource_share_arn: share_arn,
            resource_share_name: share_name,
            sender_account_id: model.sender_account_id.unwrap_or_default(),
            receiver_account_id: model
                .receiver_account_id
                .unwrap_or_else(|| account_id.clone()),
            status: model.status.unwrap_or_else(|| "ACCEPTED".to_string()),
            invitation_timestamp: 0.0,
        };

        let mut state_view = RamStateView::default();
        state_view.invitations.push(view);
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
        for inv in &view.invitations {
            let attrs = serde_json::json!({
                "id": inv.invitation_arn,
                "share_arn": inv.resource_share_arn,
                "invitation_arn": inv.invitation_arn,
                "status": inv.status,
                "sender_account_id": inv.sender_account_id,
                "receiver_account_id": inv.receiver_account_id,
            });
            results.push(ExtractedResource {
                name: inv.invitation_arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ram_sharing_with_organization
// ---------------------------------------------------------------------------

pub struct AwsRamSharingWithOrganizationConverter {
    service: Arc<RamService>,
}

impl AwsRamSharingWithOrganizationConverter {
    pub fn new(service: Arc<RamService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRamSharingWithOrganizationConverter {
    fn resource_type(&self) -> &str {
        "aws_ram_sharing_with_organization"
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

impl AwsRamSharingWithOrganizationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let account_id = extract_account_id(attrs, &ctx.default_account_id);
        let _model: ram_gen::RamSharingWithOrganizationTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_ram_sharing_with_organization", e))?;

        let state_view = RamStateView {
            sharing_with_org_enabled: true,
            ..Default::default()
        };
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
        if !view.sharing_with_org_enabled {
            return Ok(vec![]);
        }
        let id = "sharing-with-organization".to_string();
        let attrs = serde_json::json!({ "id": id });
        Ok(vec![ExtractedResource {
            name: id,
            account_id: ctx.default_account_id.clone(),
            region: ctx.default_region.clone(),
            attributes: attrs,
        }])
    }
}
