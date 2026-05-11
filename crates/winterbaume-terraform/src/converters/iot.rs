//! Terraform converters for IoT resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_iot::IotService;
use winterbaume_iot::views::{
    BillingGroupView, CACertificateView, CertificateView, DomainConfigurationView, IotPolicyView,
    IotStateView, RoleAliasView, ThingGroupView, ThingTypeView, ThingView, TopicRuleView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::iot as iot_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::ThingTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_iot_thing", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iot:{}:{}:thing/{}",
                region, ctx.default_account_id, name
            )
        });
        let version = model.version;

        let mut thing_attributes = HashMap::new();
        if let Some(obj) = attrs.get("attributes").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    thing_attributes.insert(k.clone(), s.to_string());
                }
            }
        }

        let thing_id = model.default_client_id.unwrap_or_else(|| name.clone());

        let thing_view = ThingView {
            thing_name: name.clone(),
            thing_id,
            thing_type_name: model.thing_type_name,
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
        state_view.things.insert(name, thing_view);
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::ThingTypeTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_iot_thing_type", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iot:{}:{}:thingtype/{}",
                region, ctx.default_account_id, name
            )
        });
        let thing_type_id = model.id.unwrap_or_else(|| name.clone());
        let deprecated = model.deprecated;

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
            thing_type_name: name.clone(),
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
        state_view.thing_types.insert(name, tt_view);
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::IotPolicyTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_iot_policy", e))?;

        let name = model.name.clone();
        let policy_document = model.policy.unwrap_or_default();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iot:{}:{}:policy/{}",
                region, ctx.default_account_id, name
            )
        });
        let default_version_id = model.default_version_id.unwrap_or_else(|| "1".to_string());

        let policy_view = IotPolicyView {
            policy_name: name.clone(),
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
        state_view.policies.insert(name, policy_view);
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::CertificateTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_certificate", e))?;

        let certificate_id = model.id.unwrap_or_default();
        let certificate_pem = model.certificate_pem.unwrap_or_default();
        let active = model.active;
        let status = if active {
            "ACTIVE".to_string()
        } else {
            "INACTIVE".to_string()
        };
        let arn = model.arn.unwrap_or_else(|| {
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
            ca_certificate_id: model.ca_certificate_id,
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

// ---------------------------------------------------------------------------
// aws_iot_authorizer (warning-only — no state slot in IoT view)
// ---------------------------------------------------------------------------

/// Converts `aws_iot_authorizer` resources. The IoT state view does not model
/// authorizers, so inject is best-effort (warning) and extract returns empty.
pub struct AwsIotAuthorizerConverter {
    #[allow(dead_code)]
    service: Arc<IotService>,
}

impl AwsIotAuthorizerConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotAuthorizerConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_authorizer"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsIotAuthorizerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::IotAuthorizerTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_authorizer", e))?;
        let msg = format!(
            "aws_iot_authorizer '{}' accepted but not stored; IoT view has no authorizer slot",
            model.name
        );
        eprintln!("warning: {msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![msg],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_iot_billing_group
// ---------------------------------------------------------------------------

/// Converts `aws_iot_billing_group` Terraform resources to/from IoT state.
pub struct AwsIotBillingGroupConverter {
    service: Arc<IotService>,
}

impl AwsIotBillingGroupConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotBillingGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_billing_group"
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

impl AwsIotBillingGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::IotBillingGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_billing_group", e))?;

        let name = model.name.clone();
        let billing_group_id = model.id.unwrap_or_else(|| name.clone());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iot:{}:{}:billinggroup/{}",
                region, ctx.default_account_id, name
            )
        });

        let bg_view = BillingGroupView {
            billing_group_name: name.clone(),
            billing_group_id,
            billing_group_arn: arn,
            billing_group_description: None,
            version: 1,
            creation_date: 0.0,
            things: vec![],
        };

        let mut state_view = IotStateView::default();
        state_view.billing_groups.insert(name, bg_view);
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
        for bg in view.billing_groups.values() {
            let attrs = serde_json::json!({
                "id": bg.billing_group_id,
                "name": bg.billing_group_name,
                "arn": bg.billing_group_arn,
            });
            results.push(ExtractedResource {
                name: bg.billing_group_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iot_ca_certificate
// ---------------------------------------------------------------------------

/// Converts `aws_iot_ca_certificate` Terraform resources to/from IoT state.
pub struct AwsIotCaCertificateConverter {
    service: Arc<IotService>,
}

impl AwsIotCaCertificateConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotCaCertificateConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_ca_certificate"
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

impl AwsIotCaCertificateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::IotCaCertificateTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_ca_certificate", e))?;

        let certificate_id = model.id.unwrap_or_default();
        let certificate_pem = model.ca_certificate_pem.unwrap_or_default();
        let status = if model.active {
            "ACTIVE".to_string()
        } else {
            "INACTIVE".to_string()
        };
        let auto_registration_status = if model.allow_auto_registration {
            "ENABLE".to_string()
        } else {
            "DISABLE".to_string()
        };
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iot:{}:{}:cacert/{}",
                region, ctx.default_account_id, certificate_id
            )
        });
        let mode = model
            .certificate_mode
            .unwrap_or_else(|| "DEFAULT".to_string());

        let cert_view = CACertificateView {
            certificate_id: certificate_id.clone(),
            certificate_arn: arn,
            certificate_pem,
            status,
            auto_registration_status,
            creation_date: 0.0,
            owned_by: ctx.default_account_id.clone(),
            mode,
        };

        let mut state_view = IotStateView::default();
        state_view.ca_certificates.insert(certificate_id, cert_view);
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
        for c in view.ca_certificates.values() {
            let active = c.status == "ACTIVE";
            let allow_auto_registration = c.auto_registration_status == "ENABLE";
            let attrs = serde_json::json!({
                "id": c.certificate_id,
                "arn": c.certificate_arn,
                "ca_certificate_pem": c.certificate_pem,
                "active": active,
                "allow_auto_registration": allow_auto_registration,
                "certificate_mode": c.mode,
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

// ---------------------------------------------------------------------------
// aws_iot_domain_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_iot_domain_configuration` Terraform resources.
pub struct AwsIotDomainConfigurationConverter {
    service: Arc<IotService>,
}

impl AwsIotDomainConfigurationConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotDomainConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_domain_configuration"
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

impl AwsIotDomainConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::IotDomainConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_domain_configuration", e))?;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iot:{}:{}:domainconfiguration/{}",
                region, ctx.default_account_id, name
            )
        });
        let status = model.status.unwrap_or_else(|| "ENABLED".to_string());

        let dc_view = DomainConfigurationView {
            domain_configuration_name: name.clone(),
            domain_configuration_arn: arn,
            domain_name: model.domain_name,
            domain_configuration_status: status,
            service_type: model.service_type,
            creation_date: 0.0,
        };

        let mut state_view = IotStateView::default();
        state_view.domain_configurations.insert(name, dc_view);
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
        for dc in view.domain_configurations.values() {
            let attrs = serde_json::json!({
                "id": dc.domain_configuration_name,
                "name": dc.domain_configuration_name,
                "arn": dc.domain_configuration_arn,
                "domain_name": dc.domain_name,
                "service_type": dc.service_type,
                "status": dc.domain_configuration_status,
            });
            results.push(ExtractedResource {
                name: dc.domain_configuration_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iot_event_configurations (warning-only — no state slot)
// ---------------------------------------------------------------------------

/// Converts `aws_iot_event_configurations`. IoT view has no event
/// configuration slot, so inject is warning-only.
pub struct AwsIotEventConfigurationsConverter {
    #[allow(dead_code)]
    service: Arc<IotService>,
}

impl AwsIotEventConfigurationsConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotEventConfigurationsConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_event_configurations"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsIotEventConfigurationsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: iot_gen::IotEventConfigurationsTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_event_configurations", e))?;
        let msg =
            "aws_iot_event_configurations accepted but not stored; IoT view has no event-configurations slot"
                .to_string();
        eprintln!("warning: {msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![msg],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_iot_indexing_configuration (snapshot+mutate — sets the global indexing
// config value on IotStateView).
// ---------------------------------------------------------------------------

/// Converts `aws_iot_indexing_configuration`. Writes the raw attributes JSON
/// into the global `indexing_config_thing` / `indexing_config_thing_group`
/// slots on the IoT state view. The TF spec is configuration-only so we
/// preserve the entire attribute object verbatim.
pub struct AwsIotIndexingConfigurationConverter {
    service: Arc<IotService>,
}

impl AwsIotIndexingConfigurationConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotIndexingConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_indexing_configuration"
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

impl AwsIotIndexingConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: iot_gen::IotIndexingConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_indexing_configuration", e))?;

        let attrs = &instance.attributes;
        let thing_indexing = attrs.get("thing_indexing_configuration").cloned();
        let thing_group_indexing = attrs.get("thing_group_indexing_configuration").cloned();

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if thing_indexing.is_some() {
            view.indexing_config_thing = thing_indexing;
        }
        if thing_group_indexing.is_some() {
            view.indexing_config_thing_group = thing_group_indexing;
        }
        self.service
            .restore(&ctx.default_account_id, &region, view)
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
        if view.indexing_config_thing.is_none() && view.indexing_config_thing_group.is_none() {
            return Ok(vec![]);
        }
        let attrs = serde_json::json!({
            "id": "default",
            "thing_indexing_configuration": view.indexing_config_thing,
            "thing_group_indexing_configuration": view.indexing_config_thing_group,
        });
        Ok(vec![ExtractedResource {
            name: "default".to_string(),
            account_id: ctx.default_account_id.clone(),
            region: ctx.default_region.clone(),
            attributes: attrs,
        }])
    }
}

// ---------------------------------------------------------------------------
// aws_iot_logging_options (warning-only — no state slot)
// ---------------------------------------------------------------------------

/// Converts `aws_iot_logging_options`. IoT view does not model logging
/// options, so inject is warning-only.
pub struct AwsIotLoggingOptionsConverter {
    #[allow(dead_code)]
    service: Arc<IotService>,
}

impl AwsIotLoggingOptionsConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotLoggingOptionsConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_logging_options"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsIotLoggingOptionsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: iot_gen::IotLoggingOptionsTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_logging_options", e))?;
        let msg =
            "aws_iot_logging_options accepted but not stored; IoT view has no logging-options slot"
                .to_string();
        eprintln!("warning: {msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![msg],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_iot_policy_attachment (snapshot+mutate — attach policy to target)
// ---------------------------------------------------------------------------

/// Converts `aws_iot_policy_attachment`. Modifies an existing policy by
/// appending the target identity to `IotPolicyView::targets`. Depends on
/// `aws_iot_policy` having been processed first.
pub struct AwsIotPolicyAttachmentConverter {
    service: Arc<IotService>,
}

impl AwsIotPolicyAttachmentConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotPolicyAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_policy_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_iot_policy"]
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

impl AwsIotPolicyAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::IotPolicyAttachmentTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_policy_attachment", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(policy) = view.policies.get_mut(&model.policy) {
            if !policy.targets.iter().any(|t| t == &model.target) {
                policy.targets.push(model.target.clone());
            }
        } else {
            warnings.push(format!(
                "policy '{}' not found in state; attachment to '{}' skipped",
                model.policy, model.target
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
            for target in &policy.targets {
                let id = format!("{}|{}", policy.policy_name, target);
                let attrs = serde_json::json!({
                    "id": id,
                    "policy": policy.policy_name,
                    "target": target,
                });
                results.push(ExtractedResource {
                    name: id,
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
// aws_iot_provisioning_template (warning-only — no state slot)
// ---------------------------------------------------------------------------

/// Converts `aws_iot_provisioning_template`. IoT view has no provisioning
/// template slot, so inject is warning-only.
pub struct AwsIotProvisioningTemplateConverter {
    #[allow(dead_code)]
    service: Arc<IotService>,
}

impl AwsIotProvisioningTemplateConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotProvisioningTemplateConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_provisioning_template"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsIotProvisioningTemplateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::IotProvisioningTemplateTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_provisioning_template", e))?;
        let msg = format!(
            "aws_iot_provisioning_template '{}' accepted but not stored; IoT view has no provisioning-template slot",
            model.name
        );
        eprintln!("warning: {msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![msg],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_iot_role_alias
// ---------------------------------------------------------------------------

/// Converts `aws_iot_role_alias` Terraform resources.
pub struct AwsIotRoleAliasConverter {
    service: Arc<IotService>,
}

impl AwsIotRoleAliasConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotRoleAliasConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_role_alias"
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

impl AwsIotRoleAliasConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::IotRoleAliasTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_role_alias", e))?;

        let alias = model.alias.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iot:{}:{}:rolealias/{}",
                region, ctx.default_account_id, alias
            )
        });

        let credential_duration = i32::try_from(model.credential_duration).unwrap_or(3600);
        let ra_view = RoleAliasView {
            role_alias: alias.clone(),
            role_alias_arn: arn,
            role_arn: model.role_arn,
            credential_duration_seconds: credential_duration,
            creation_date: 0.0,
            last_modified_date: 0.0,
            owner: ctx.default_account_id.clone(),
        };

        let mut state_view = IotStateView::default();
        state_view.role_aliases.insert(alias, ra_view);
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
        for ra in view.role_aliases.values() {
            let attrs = serde_json::json!({
                "id": ra.role_alias,
                "alias": ra.role_alias,
                "arn": ra.role_alias_arn,
                "role_arn": ra.role_arn,
                "credential_duration": ra.credential_duration_seconds,
            });
            results.push(ExtractedResource {
                name: ra.role_alias.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iot_thing_group
// ---------------------------------------------------------------------------

/// Converts `aws_iot_thing_group` Terraform resources.
pub struct AwsIotThingGroupConverter {
    service: Arc<IotService>,
}

impl AwsIotThingGroupConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotThingGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_thing_group"
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

impl AwsIotThingGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::IotThingGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_thing_group", e))?;

        let name = model.name.clone();
        let thing_group_id = model.id.unwrap_or_else(|| name.clone());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iot:{}:{}:thinggroup/{}",
                region, ctx.default_account_id, name
            )
        });

        let attrs = &instance.attributes;
        let mut group_attributes = HashMap::new();
        let mut description = None;
        if let Some(props) = attrs.get("properties").and_then(|v| v.as_array()) {
            if let Some(prop_obj) = props.first().and_then(|v| v.as_object()) {
                description = prop_obj
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                if let Some(payload) = prop_obj.get("attribute_payload").and_then(|v| v.as_array())
                {
                    if let Some(payload_obj) = payload.first().and_then(|v| v.as_object()) {
                        if let Some(obj) = payload_obj.get("attributes").and_then(|v| v.as_object())
                        {
                            for (k, v) in obj {
                                if let Some(s) = v.as_str() {
                                    group_attributes.insert(k.clone(), s.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        let tg_view = ThingGroupView {
            thing_group_name: name.clone(),
            thing_group_id,
            thing_group_arn: arn,
            parent_group_name: model.parent_group_name,
            thing_group_description: description,
            attributes: group_attributes,
            version: model.version,
            creation_date: 0.0,
            things: vec![],
        };

        let mut state_view = IotStateView::default();
        state_view.thing_groups.insert(name, tg_view);
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
        for tg in view.thing_groups.values() {
            let attrs = serde_json::json!({
                "id": tg.thing_group_id,
                "name": tg.thing_group_name,
                "arn": tg.thing_group_arn,
                "parent_group_name": tg.parent_group_name,
                "version": tg.version,
            });
            results.push(ExtractedResource {
                name: tg.thing_group_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iot_thing_group_membership (snapshot+mutate)
// ---------------------------------------------------------------------------

/// Converts `aws_iot_thing_group_membership`. Modifies existing thing and
/// thing group state to record the membership link. Depends on
/// `aws_iot_thing` and `aws_iot_thing_group` having been processed first.
pub struct AwsIotThingGroupMembershipConverter {
    service: Arc<IotService>,
}

impl AwsIotThingGroupMembershipConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotThingGroupMembershipConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_thing_group_membership"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_iot_thing", "aws_iot_thing_group"]
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

impl AwsIotThingGroupMembershipConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::IotThingGroupMembershipTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_thing_group_membership", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];

        if let Some(group) = view.thing_groups.get_mut(&model.thing_group_name) {
            if !group.things.iter().any(|t| t == &model.thing_name) {
                group.things.push(model.thing_name.clone());
            }
        } else {
            warnings.push(format!(
                "thing group '{}' not found in state; membership skipped",
                model.thing_group_name
            ));
        }
        if let Some(thing) = view.things.get_mut(&model.thing_name) {
            if !thing
                .thing_groups
                .iter()
                .any(|g| g == &model.thing_group_name)
            {
                thing.thing_groups.push(model.thing_group_name.clone());
            }
        } else {
            warnings.push(format!(
                "thing '{}' not found in state; membership skipped",
                model.thing_name
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
        for tg in view.thing_groups.values() {
            for thing_name in &tg.things {
                let id = format!("{}|{}", thing_name, tg.thing_group_name);
                let attrs = serde_json::json!({
                    "id": id,
                    "thing_name": thing_name,
                    "thing_group_name": tg.thing_group_name,
                });
                results.push(ExtractedResource {
                    name: id,
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
// aws_iot_thing_principal_attachment (snapshot+mutate)
// ---------------------------------------------------------------------------

/// Converts `aws_iot_thing_principal_attachment`. Modifies an existing thing
/// to append the principal. Depends on `aws_iot_thing` having been
/// processed first.
pub struct AwsIotThingPrincipalAttachmentConverter {
    service: Arc<IotService>,
}

impl AwsIotThingPrincipalAttachmentConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotThingPrincipalAttachmentConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_thing_principal_attachment"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_iot_thing"]
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

impl AwsIotThingPrincipalAttachmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::IotThingPrincipalAttachmentTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_thing_principal_attachment", e))?;

        let mut view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(thing) = view.things.get_mut(&model.thing) {
            if !thing.principals.iter().any(|p| p == &model.principal) {
                thing.principals.push(model.principal.clone());
            }
        } else {
            warnings.push(format!(
                "thing '{}' not found in state; principal attachment skipped",
                model.thing
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
            for p in &t.principals {
                let id = format!("{}|{}", t.thing_name, p);
                let attrs = serde_json::json!({
                    "id": id,
                    "thing": t.thing_name,
                    "principal": p,
                });
                results.push(ExtractedResource {
                    name: id,
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
// aws_iot_topic_rule
// ---------------------------------------------------------------------------

/// Converts `aws_iot_topic_rule` Terraform resources. Nested action blocks
/// are preserved verbatim into `actions_json`/`error_action_json` from the
/// raw attribute payload.
pub struct AwsIotTopicRuleConverter {
    service: Arc<IotService>,
}

impl AwsIotTopicRuleConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotTopicRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_topic_rule"
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

impl AwsIotTopicRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: iot_gen::IotTopicRuleTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_topic_rule", e))?;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:iot:{}:{}:rule/{}",
                region, ctx.default_account_id, name
            )
        });
        let attrs = &instance.attributes;
        let actions_json = serde_json::json!({});
        let actions_value = attrs
            .get("actions")
            .or(attrs.get("action"))
            .cloned()
            .unwrap_or(actions_json);
        let error_action_value = attrs.get("error_action").cloned();

        let tr_view = TopicRuleView {
            rule_name: name.clone(),
            rule_arn: arn,
            sql: model.sql,
            description: model.description,
            rule_disabled: !model.enabled,
            creation_date: 0.0,
            actions_json: actions_value,
            error_action_json: error_action_value,
            aws_iot_sql_version: model.sql_version,
        };

        let mut state_view = IotStateView::default();
        state_view.topic_rules.insert(name, tr_view);
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
        for tr in view.topic_rules.values() {
            let attrs = serde_json::json!({
                "id": tr.rule_name,
                "name": tr.rule_name,
                "arn": tr.rule_arn,
                "description": tr.description,
                "enabled": !tr.rule_disabled,
                "sql": tr.sql,
                "sql_version": tr.aws_iot_sql_version,
            });
            results.push(ExtractedResource {
                name: tr.rule_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_iot_topic_rule_destination (warning-only — no state slot)
// ---------------------------------------------------------------------------

/// Converts `aws_iot_topic_rule_destination`. IoT view has no destination
/// slot, so inject is warning-only.
pub struct AwsIotTopicRuleDestinationConverter {
    #[allow(dead_code)]
    service: Arc<IotService>,
}

impl AwsIotTopicRuleDestinationConverter {
    pub fn new(service: Arc<IotService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsIotTopicRuleDestinationConverter {
    fn resource_type(&self) -> &str {
        "aws_iot_topic_rule_destination"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsIotTopicRuleDestinationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: iot_gen::IotTopicRuleDestinationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_iot_topic_rule_destination", e))?;
        let msg =
            "aws_iot_topic_rule_destination accepted but not stored; IoT view has no destination slot"
                .to_string();
        eprintln!("warning: {msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![msg],
        })
    }
}
