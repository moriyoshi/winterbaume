//! Terraform converter for Account resources.
//!
//! `AlternateContactTfModel` is generated from `specs/account.toml`. The
//! StateView assembly (which uses the contact type as the map key) is
//! wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_account::AccountService;
use winterbaume_account::views::{AccountStateView, AlternateContactView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::account as account_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_account_alternate_contact
// ---------------------------------------------------------------------------

/// Converts `aws_account_alternate_contact` Terraform resources to/from Account state.
pub struct AwsAccountAlternateContactConverter {
    service: Arc<AccountService>,
}

impl AwsAccountAlternateContactConverter {
    pub fn new(service: Arc<AccountService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAccountAlternateContactConverter {
    fn resource_type(&self) -> &str {
        "aws_account_alternate_contact"
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

impl AwsAccountAlternateContactConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: account_gen::AlternateContactTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_account_alternate_contact", e))?;

        let contact_view = AlternateContactView {
            alternate_contact_type: model.alternate_contact_type.clone(),
            name: model.name,
            email_address: model.email_address,
            phone_number: model.phone_number,
            title: model.title,
        };

        let mut state_view = AccountStateView {
            alternate_contacts: HashMap::new(),
            contact_information: None,
            account_name: None,
            primary_email: None,
            region_overrides: HashMap::new(),
        };
        state_view
            .alternate_contacts
            .insert(model.alternate_contact_type, contact_view);
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
        for (contact_type, contact) in &view.alternate_contacts {
            let attrs = serde_json::json!({
                "id": contact_type,
                "alternate_contact_type": contact.alternate_contact_type,
                "name": contact.name,
                "email_address": contact.email_address,
                "phone_number": contact.phone_number,
                "title": contact.title,
            });
            results.push(ExtractedResource {
                name: contact_type.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
