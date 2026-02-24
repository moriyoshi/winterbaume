//! Terraform converters for IoT resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_iot::IotService;
use winterbaume_iot::views::{
    CertificateView, IotPolicyView, IotStateView, ThingTypeView, ThingView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{
    extract_region, extract_tags, optional_bool, optional_i64, optional_str, require_str,
};

// ---------------------------------------------------------------------------
// aws_iot_thing
// ---------------------------------------------------------------------------

/// Converts `aws_iot_thing` Terraform resources to/from IoT state.
pub struct AwsIotThingConverter {
    service: Arc<IotService>,
}

impl AwsIotThingConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotThingConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_thing"
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

impl AwsIotThingConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_iot_thing")?;
        let region = extract_region(attrs, &ctx.default_region);

        let thing_type_name = optional_str(attrs, "thing_type_name");
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:iot:{}:{}:thing/{}",
                region, ctx.default_account_id, name
            )
        });
        let version = optional_i64(attrs, "version").unwrap_or(1);

        let mut thing_attributes = HashMap::new();
        if let Some(obj) = attrs.get("attributes").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    thing_attributes.insert(k.clone(), s.to_string());
                }
            }
        }

        let thing_id = optional_str(attrs, "default_client_id").unwrap_or_else(|| name.to_string());

        let thing_view = ThingView {
            thing_name: name.to_string(),
            thing_id,
            thing_type_name,
            attributes: thing_attributes,
            version,
            thing_arn: arn,
            billing_group_name: None,
            principals: vec![],
            thing_groups: vec![],
        };

        let mut state_view = IotStateView {
            things: HashMap::new(),
            thing_types: HashMap::new(),
            thing_groups: HashMap::new(),
            billing_groups: HashMap::new(),
            certificates: HashMap::new(),
            ca_certificates: HashMap::new(),
            policies: HashMap::new(),
            role_aliases: HashMap::new(),
            domain_configurations: HashMap::new(),
            jobs: HashMap::new(),
            job_templates: HashMap::new(),
            topic_rules: HashMap::new(),
            registration_code: String::new(),
            indexing_config_thing: None,
            indexing_config_thing_group: None,
        };
        state_view.things.insert(name.to_string(), thing_view);
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
        for t in view.things.values() {
            let attrs = serde_json::json!({
                "id": t.thing_name,
                "name": t.thing_name,
                "arn": t.thing_arn,
                "default_client_id": t.thing_id,
                "thing_type_name": t.thing_type_name,
                "attributes": t.attributes,
                "version": t.version,
            });
            results.push(ExtractedResource {
                name: t.thing_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iot_thing_type
// ---------------------------------------------------------------------------

/// Converts `aws_iot_thing_type` Terraform resources to/from IoT state.
pub struct AwsIotThingTypeConverter {
    service: Arc<IotService>,
}

impl AwsIotThingTypeConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotThingTypeConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_thing_type"
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

impl AwsIotThingTypeConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_iot_thing_type")?;
        let region = extract_region(attrs, &ctx.default_region);

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:iot:{}:{}:thingtype/{}",
                region, ctx.default_account_id, name
            )
        });
        let thing_type_id = optional_str(attrs, "id").unwrap_or_else(|| name.to_string());
        let deprecated = optional_bool(attrs, "deprecated").unwrap_or(false);

        let mut searchable_attributes = None;
        let mut description = None;
        if let Some(props) = attrs.get("properties").and_then(|v| v.as_array()) {
            if let Some(prop_obj) = props.first().and_then(|v| v.as_object()) {
                description = prop_obj
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                if let Some(sa) = prop_obj
                    .get("searchable_attributes")
                    .and_then(|v| v.as_array())
                {
                    searchable_attributes = Some(
                        sa.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect(),
                    );
                }
            }
        }

        let tt_view = ThingTypeView {
            thing_type_name: name.to_string(),
            thing_type_id,
            thing_type_arn: arn,
            thing_type_description: description,
            searchable_attributes,
            creation_date: 0.0,
            deprecated,
            deprecation_date: None,
        };

        let mut state_view = IotStateView {
            things: HashMap::new(),
            thing_types: HashMap::new(),
            thing_groups: HashMap::new(),
            billing_groups: HashMap::new(),
            certificates: HashMap::new(),
            ca_certificates: HashMap::new(),
            policies: HashMap::new(),
            role_aliases: HashMap::new(),
            domain_configurations: HashMap::new(),
            jobs: HashMap::new(),
            job_templates: HashMap::new(),
            topic_rules: HashMap::new(),
            registration_code: String::new(),
            indexing_config_thing: None,
            indexing_config_thing_group: None,
        };
        state_view.thing_types.insert(name.to_string(), tt_view);
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
        for tt in view.thing_types.values() {
            let attrs = serde_json::json!({
                "id": tt.thing_type_id,
                "name": tt.thing_type_name,
                "arn": tt.thing_type_arn,
                "deprecated": tt.deprecated,
                "properties": [{
                    "description": tt.thing_type_description,
                    "searchable_attributes": tt.searchable_attributes,
                }],
            });
            results.push(ExtractedResource {
                name: tt.thing_type_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iot_policy
// ---------------------------------------------------------------------------

/// Converts `aws_iot_policy` Terraform resources to/from IoT state.
pub struct AwsIotPolicyConverter {
    service: Arc<IotService>,
}

impl AwsIotPolicyConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_policy"
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

impl AwsIotPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_iot_policy")?;
        let region = extract_region(attrs, &ctx.default_region);

        let policy_document = optional_str(attrs, "policy").unwrap_or_default();
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:iot:{}:{}:policy/{}",
                region, ctx.default_account_id, name
            )
        });
        let default_version_id =
            optional_str(attrs, "default_version_id").unwrap_or_else(|| "1".to_string());

        let _tags = extract_tags(attrs);

        let policy_view = IotPolicyView {
            policy_name: name.to_string(),
            policy_arn: arn,
            policy_document,
            creation_date: 0.0,
            last_modified_date: 0.0,
            generation_id: String::new(),
            versions: vec![],
            default_version_id,
            targets: vec![],
            principals: vec![],
        };

        let mut state_view = IotStateView {
            things: HashMap::new(),
            thing_types: HashMap::new(),
            thing_groups: HashMap::new(),
            billing_groups: HashMap::new(),
            certificates: HashMap::new(),
            ca_certificates: HashMap::new(),
            policies: HashMap::new(),
            role_aliases: HashMap::new(),
            domain_configurations: HashMap::new(),
            jobs: HashMap::new(),
            job_templates: HashMap::new(),
            topic_rules: HashMap::new(),
            registration_code: String::new(),
            indexing_config_thing: None,
            indexing_config_thing_group: None,
        };
        state_view.policies.insert(name.to_string(), policy_view);
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
        for p in view.policies.values() {
            let attrs = serde_json::json!({
                "id": p.policy_name,
                "name": p.policy_name,
                "arn": p.policy_arn,
                "policy": p.policy_document,
                "default_version_id": p.default_version_id,
            });
            results.push(ExtractedResource {
                name: p.policy_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iot_certificate
// ---------------------------------------------------------------------------

/// Converts `aws_iot_certificate` Terraform resources to/from IoT state.
pub struct AwsIotCertificateConverter {
    service: Arc<IotService>,
}

impl AwsIotCertificateConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotCertificateConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_certificate"
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

impl AwsIotCertificateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let certificate_id = optional_str(attrs, "id").unwrap_or_default();
        let certificate_pem = optional_str(attrs, "certificate_pem").unwrap_or_default();
        let _ca_pem = optional_str(attrs, "ca_pem");
        let active = optional_bool(attrs, "active").unwrap_or(true);
        let status = if active {
            "ACTIVE".to_string()
        } else {
            "INACTIVE".to_string()
        };
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:iot:{}:{}:cert/{}",
                region, ctx.default_account_id, certificate_id
            )
        });

        let cert_view = CertificateView {
            certificate_id: certificate_id.clone(),
            certificate_arn: arn,
            certificate_pem,
            status,
            creation_date: 0.0,
            ca_certificate_id: optional_str(attrs, "ca_certificate_id"),
            owned_by: ctx.default_account_id.clone(),
            mode: "DEFAULT".to_string(),
        };

        let mut state_view = IotStateView {
            things: HashMap::new(),
            thing_types: HashMap::new(),
            thing_groups: HashMap::new(),
            billing_groups: HashMap::new(),
            certificates: HashMap::new(),
            ca_certificates: HashMap::new(),
            policies: HashMap::new(),
            role_aliases: HashMap::new(),
            domain_configurations: HashMap::new(),
            jobs: HashMap::new(),
            job_templates: HashMap::new(),
            topic_rules: HashMap::new(),
            registration_code: String::new(),
            indexing_config_thing: None,
            indexing_config_thing_group: None,
        };
        state_view.certificates.insert(certificate_id, cert_view);
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
        for c in view.certificates.values() {
            let active = c.status == "ACTIVE";
            let attrs = serde_json::json!({
                "id": c.certificate_id,
                "arn": c.certificate_arn,
                "certificate_pem": c.certificate_pem,
                "active": active,
                "ca_pem": null,
            });
            results.push(ExtractedResource {
                name: c.certificate_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
