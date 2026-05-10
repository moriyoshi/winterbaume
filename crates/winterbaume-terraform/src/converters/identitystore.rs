//! Terraform converter for Identity Store resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_identitystore::IdentityStoreService;
use winterbaume_identitystore::types::{Address, Email, Name, PhoneNumber};
use winterbaume_identitystore::views::{GroupView, IdentityStoreStateView, UserView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::identitystore as identitystore_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_identitystore_group
// ---------------------------------------------------------------------------

/// Converts `aws_identitystore_group` Terraform resources to/from Identity Store state.
pub struct AwsIdentitystoreGroupConverter {
    service: Arc<IdentityStoreService>,
}

impl AwsIdentitystoreGroupConverter {
    pub fn new(service: Arc<IdentityStoreService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIdentitystoreGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_identitystore_group"
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

impl AwsIdentitystoreGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: identitystore_gen::GroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_identitystore_group", e))?;

        let identity_store_id = model.identity_store_id;
        let group_id = model.group_id;

        let group_view = GroupView {
            identity_store_id: identity_store_id.clone(),
            group_id: group_id.clone(),
            display_name: model.display_name,
            description: model.description,
            external_ids: vec![],
        };

        let key = format!("{identity_store_id}/{group_id}");
        let mut state_view = IdentityStoreStateView {
            users: HashMap::new(),
            groups: HashMap::new(),
            memberships: HashMap::new(),
        };
        state_view.groups.insert(key, group_view);
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
        for group in view.groups.values() {
            let id = format!("{}/{}", group.identity_store_id, group.group_id);
            let attrs = serde_json::json!({
                "id": id,
                "identity_store_id": group.identity_store_id,
                "group_id": group.group_id,
                "display_name": group.display_name,
                "description": group.description,
                "external_ids": group.external_ids,
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
// aws_identitystore_user
// ---------------------------------------------------------------------------

/// Converts `aws_identitystore_user` Terraform resources to/from Identity Store state.
pub struct AwsIdentitystoreUserConverter {
    service: Arc<IdentityStoreService>,
}

impl AwsIdentitystoreUserConverter {
    pub fn new(service: Arc<IdentityStoreService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIdentitystoreUserConverter {
    fn resource_type(&self) -> &str {
        "aws_identitystore_user"
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

impl AwsIdentitystoreUserConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: identitystore_gen::UserTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_identitystore_user", e))?;

        let identity_store_id = model.identity_store_id;
        let user_id = model.user_id;

        // Parse nested name block (read raw — not part of strongly-typed model).
        let name_parsed: Option<Name> = attrs
            .get("name")
            .and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .map(|n| Name {
                formatted: n
                    .get("formatted")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                family_name: n
                    .get("family_name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                given_name: n
                    .get("given_name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                middle_name: n
                    .get("middle_name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                honorific_prefix: n
                    .get("honorific_prefix")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                honorific_suffix: n
                    .get("honorific_suffix")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
            });

        // Parse nested emails blocks.
        let emails_parsed: Option<Vec<Email>> =
            attrs.get("emails").and_then(|v| v.as_array()).map(|arr| {
                arr.iter()
                    .map(|e| Email {
                        value: e
                            .get("value")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        email_type: e
                            .get("type")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        primary: e.get("primary").and_then(|v| v.as_bool()),
                    })
                    .collect()
            });

        // Parse nested addresses blocks.
        let addresses_parsed: Option<Vec<Address>> = attrs
            .get("addresses")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|a| Address {
                        street_address: a
                            .get("street_address")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        locality: a
                            .get("locality")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        region: a
                            .get("region")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        postal_code: a
                            .get("postal_code")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        country: a
                            .get("country")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        formatted: a
                            .get("formatted")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        address_type: a
                            .get("type")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        primary: a.get("primary").and_then(|v| v.as_bool()),
                    })
                    .collect()
            });

        // Parse nested phone_numbers blocks.
        let phone_numbers_parsed: Option<Vec<PhoneNumber>> = attrs
            .get("phone_numbers")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|p| PhoneNumber {
                        value: p
                            .get("value")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        phone_type: p
                            .get("type")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        primary: p.get("primary").and_then(|v| v.as_bool()),
                    })
                    .collect()
            });

        let user_view = UserView {
            identity_store_id: identity_store_id.clone(),
            user_id: user_id.clone(),
            user_name: model.user_name,
            name: name_parsed,
            display_name: model.display_name,
            nick_name: model.nick_name,
            profile_url: model.profile_url,
            emails: emails_parsed,
            addresses: addresses_parsed,
            phone_numbers: phone_numbers_parsed,
            user_type: model.user_type,
            title: model.title,
            preferred_language: model.preferred_language,
            locale: model.locale,
            timezone: model.timezone,
            external_ids: vec![],
            photos: None,
            website: model.website,
            birthdate: model.birthdate,
            roles: None,
        };

        let key = format!("{identity_store_id}/{user_id}");
        let mut state_view = IdentityStoreStateView {
            users: HashMap::new(),
            groups: HashMap::new(),
            memberships: HashMap::new(),
        };
        state_view.users.insert(key, user_view);
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
        for user in view.users.values() {
            let id = format!("{}/{}", user.identity_store_id, user.user_id);
            let attrs = serde_json::json!({
                "id": id,
                "identity_store_id": user.identity_store_id,
                "user_id": user.user_id,
                "user_name": user.user_name,
                "name": user.name,
                "display_name": user.display_name,
                "nick_name": user.nick_name,
                "profile_url": user.profile_url,
                "emails": user.emails,
                "addresses": user.addresses,
                "phone_numbers": user.phone_numbers,
                "user_type": user.user_type,
                "title": user.title,
                "preferred_language": user.preferred_language,
                "locale": user.locale,
                "timezone": user.timezone,
                "external_ids": user.external_ids,
                "photos": user.photos,
                "website": user.website,
                "birthdate": user.birthdate,
                "roles": user.roles,
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
