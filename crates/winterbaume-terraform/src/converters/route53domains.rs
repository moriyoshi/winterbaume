//! Terraform converter for Route53 Domains resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_route53domains::Route53DomainsService;
use winterbaume_route53domains::views::{
    ContactDetailView, DomainRegistrationView, NameserverRecord, Route53DomainsStateView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_bool, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_route53domains_registered_domain
// ---------------------------------------------------------------------------

/// Converts `aws_route53domains_registered_domain` Terraform resources to/from Route53 Domains state.
pub struct AwsRoute53DomainsRegisteredDomainConverter {
    service: Arc<Route53DomainsService>,
}

impl AwsRoute53DomainsRegisteredDomainConverter {
    pub fn new(service: Arc<Route53DomainsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRoute53DomainsRegisteredDomainConverter {
    fn resource_type(&self) -> &str {
        "aws_route53domains_registered_domain"
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

fn extract_contact_detail(attrs: &serde_json::Value, prefix: &str) -> ContactDetailView {
    let get = |field: &str| -> Option<String> {
        attrs
            .get(prefix)
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|obj| obj.get(field))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    };
    ContactDetailView {
        first_name: get("first_name"),
        last_name: get("last_name"),
        email: get("email"),
        phone_number: get("phone_number"),
        organization_name: get("organization_name"),
        address_line_1: get("address_line_1"),
        address_line_2: get("address_line_2"),
        city: get("city"),
        state: get("state"),
        country_code: get("country_code"),
        zip_code: get("zip_code"),
        contact_type: get("contact_type"),
    }
}

fn contact_to_json(contact: &ContactDetailView) -> serde_json::Value {
    serde_json::json!({
        "first_name": contact.first_name,
        "last_name": contact.last_name,
        "email": contact.email,
        "phone_number": contact.phone_number,
        "organization_name": contact.organization_name,
        "address_line_1": contact.address_line_1,
        "address_line_2": contact.address_line_2,
        "city": contact.city,
        "state": contact.state,
        "country_code": contact.country_code,
        "zip_code": contact.zip_code,
        "contact_type": contact.contact_type,
    })
}

impl AwsRoute53DomainsRegisteredDomainConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let domain_name =
            require_str(attrs, "domain_name", "aws_route53domains_registered_domain")?;
        let region = extract_region(attrs, &ctx.default_region);
        let auto_renew = optional_bool(attrs, "auto_renew").unwrap_or(true);
        let transfer_lock = optional_bool(attrs, "transfer_lock").unwrap_or(true);
        let admin_privacy = optional_bool(attrs, "admin_privacy").unwrap_or(true);
        let registrant_privacy = optional_bool(attrs, "registrant_privacy").unwrap_or(true);
        let tech_privacy = optional_bool(attrs, "tech_privacy").unwrap_or(true);

        let _tags_all = attrs.get("tags_all");
        let billing_contact = attrs.get("billing_contact").and_then(|v| {
            v.as_array().and_then(|arr| arr.first()).map(|obj| {
                let get = |field: &str| -> Option<String> {
                    obj.get(field)
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                };
                ContactDetailView {
                    first_name: get("first_name"),
                    last_name: get("last_name"),
                    email: get("email"),
                    phone_number: get("phone_number"),
                    organization_name: get("organization_name"),
                    address_line_1: get("address_line_1"),
                    address_line_2: get("address_line_2"),
                    city: get("city"),
                    state: get("state"),
                    country_code: get("country_code"),
                    zip_code: get("zip_code"),
                    contact_type: get("contact_type"),
                }
            })
        });
        let billing_privacy = optional_bool(attrs, "billing_privacy").unwrap_or(true);

        // Parse name_server blocks
        let name_server: Vec<NameserverRecord> = attrs
            .get("name_server")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|ns| {
                        let name = ns.get("name")?.as_str()?.to_string();
                        let glue_ips: Vec<String> = ns
                            .get("glue_ips")
                            .and_then(|v| v.as_array())
                            .map(|a| {
                                a.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default();
                        Some(NameserverRecord { name, glue_ips })
                    })
                    .collect()
            })
            .unwrap_or_default();
        let _abuse_contact_email = optional_str(attrs, "abuse_contact_email");
        let _domain_price = optional_str(attrs, "domain_price");
        let _availability_status = optional_str(attrs, "availability_status");
        let _registrar_name = optional_str(attrs, "registrar_name");
        let _registrar_url = optional_str(attrs, "registrar_url");
        let _reseller = optional_str(attrs, "reseller");
        let _whois_server = optional_str(attrs, "whois_server");
        let _abuse_contact_phone = optional_str(attrs, "abuse_contact_phone");

        let admin_contact = extract_contact_detail(attrs, "admin_contact");
        let registrant_contact = extract_contact_detail(attrs, "registrant_contact");
        let tech_contact = extract_contact_detail(attrs, "tech_contact");

        let now = chrono::Utc::now();
        let domain_view = DomainRegistrationView {
            domain_name: domain_name.to_string(),
            auto_renew,
            admin_contact,
            registrant_contact,
            tech_contact,
            admin_privacy,
            registrant_privacy,
            tech_privacy,
            creation_date: now,
            expiration_date: now,
            updated_date: now,
            transfer_lock,
            status_list: vec![],
            nameservers: vec![],
            name_server,
            billing_contact,
            billing_privacy,
        };

        let mut state_view = Route53DomainsStateView {
            domains: HashMap::new(),
        };
        state_view
            .domains
            .insert(domain_name.to_string(), domain_view);
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
        for domain in view.domains.values() {
            let attrs = serde_json::json!({
                "id": domain.domain_name,
                "domain_name": domain.domain_name,
                "auto_renew": domain.auto_renew,
                "admin_contact": [contact_to_json(&domain.admin_contact)],
                "registrant_contact": [contact_to_json(&domain.registrant_contact)],
                "tech_contact": [contact_to_json(&domain.tech_contact)],
                "admin_privacy": domain.admin_privacy,
                "registrant_privacy": domain.registrant_privacy,
                "tech_privacy": domain.tech_privacy,
                "creation_date": domain.creation_date.to_rfc3339(),
                "expiration_date": domain.expiration_date.to_rfc3339(),
                "updated_date": domain.updated_date.to_rfc3339(),
                "transfer_lock": domain.transfer_lock,
                "status_list": domain.status_list,
                "nameservers": domain.nameservers,
                "name_server": domain.name_server.iter().map(|ns| serde_json::json!({
                    "name": ns.name,
                    "glue_ips": ns.glue_ips,
                })).collect::<Vec<_>>(),
                "billing_contact": domain.billing_contact.as_ref().map(|c| vec![contact_to_json(c)]),
                "billing_privacy": domain.billing_privacy,
            });
            results.push(ExtractedResource {
                name: domain.domain_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
