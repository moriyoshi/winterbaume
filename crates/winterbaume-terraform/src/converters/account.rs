//! Terraform converter for Account resources.

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
use crate::util::{extract_region, require_str};

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
        let attrs = &instance.attributes;
        let contact_type = require_str(
            attrs,
            "alternate_contact_type",
            "aws_account_alternate_contact",
        )?;
        let name = require_str(attrs, "name", "aws_account_alternate_contact")?;
        let email_address = require_str(attrs, "email_address", "aws_account_alternate_contact")?;
        let phone_number = require_str(attrs, "phone_number", "aws_account_alternate_contact")?;
        let title = require_str(attrs, "title", "aws_account_alternate_contact")?;
        let region = extract_region(attrs, &ctx.default_region);

        let contact_view = AlternateContactView {
            alternate_contact_type: contact_type.to_string(),
            name: name.to_string(),
            email_address: email_address.to_string(),
            phone_number: phone_number.to_string(),
            title: title.to_string(),
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
            .insert(contact_type.to_string(), contact_view);
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
