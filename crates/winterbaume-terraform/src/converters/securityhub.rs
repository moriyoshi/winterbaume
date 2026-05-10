//! Terraform converter for SecurityHub resources.
//!
//! `AccountTfModel` and `StandardsSubscriptionTfModel` are generated from
//! `specs/securityhub.toml`. The default-standards subscription ARN
//! template, the `id`-as-fallback override, and the `subscribed_at`
//! timestamp constant are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_securityhub::SecurityHubService;
use winterbaume_securityhub::views::{
    HubInfoView, SecurityHubStateView, StandardsSubscriptionView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::securityhub as securityhub_gen;
use crate::util::{classify_deserialize_error, extract_region, optional_str};

// ---------------------------------------------------------------------------
// aws_securityhub_account
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_account` Terraform resources to/from SecurityHub state.
pub struct AwsSecurityhubAccountConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubAccountConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubAccountConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_account"
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

impl AwsSecurityhubAccountConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::AccountTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_securityhub_account", e))?;

        let control_finding_generator = model
            .control_finding_generator
            .unwrap_or_else(|| "SECURITY_CONTROL".to_string());

        let hub_view = HubInfoView {
            enabled: true,
            subscribed_at: Some(chrono::Utc::now().to_rfc3339()),
            tags: model.tags,
        };

        let state_view = SecurityHubStateView {
            hub: hub_view,
            auto_enable_controls: model.auto_enable_controls,
            control_finding_generator,
            enabled_standards: if model.enable_default_standards {
                vec![StandardsSubscriptionView {
                    standards_subscription_arn: format!(
                        "arn:aws:securityhub:{}:{}:subscription/aws-foundational-security-best-practices/v/1.0.0",
                        region, ctx.default_account_id
                    ),
                    standards_arn: "arn:aws:securityhub:::ruleset/aws-foundational-security-best-practices/v/1.0.0".to_string(),
                    standards_status: "READY".to_string(),
                    standards_input: HashMap::new(),
                }]
            } else {
                vec![]
            },
            ..Default::default()
        };
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
        if !view.hub.enabled {
            return Ok(vec![]);
        }
        let attrs = serde_json::json!({
            "id": ctx.default_account_id,
            "arn": format!(
                "arn:aws:securityhub:{}:{}:hub/default",
                ctx.default_region, ctx.default_account_id
            ),
            "enable_default_standards": !view.enabled_standards.is_empty(),
            "auto_enable_controls": view.auto_enable_controls,
            "control_finding_generator": view.control_finding_generator,
            "tags": view.hub.tags,
        });
        Ok(vec![ExtractedResource {
            name: ctx.default_account_id.clone(),
            account_id: ctx.default_account_id.clone(),
            region: ctx.default_region.clone(),
            attributes: attrs,
        }])
    }
}

// ---------------------------------------------------------------------------
// aws_securityhub_standards_subscription
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_standards_subscription` Terraform resources to/from
/// SecurityHub state.
pub struct AwsSecurityhubStandardsSubscriptionConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubStandardsSubscriptionConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubStandardsSubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_standards_subscription"
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

impl AwsSecurityhubStandardsSubscriptionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::StandardsSubscriptionTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_securityhub_standards_subscription", e)
            })?;

        let standards_arn = model.standards_arn;
        let subscription_arn = optional_str(&instance.attributes, "id").unwrap_or_else(|| {
            format!(
                "arn:aws:securityhub:{}:{}:subscription/{}",
                region,
                ctx.default_account_id,
                standards_arn
                    .rsplit_once(":::")
                    .map(|(_, s)| s)
                    .unwrap_or(&standards_arn)
            )
        });

        let sub_view = StandardsSubscriptionView {
            standards_subscription_arn: subscription_arn,
            standards_arn,
            standards_status: "READY".to_string(),
            standards_input: HashMap::new(),
        };

        let state_view = SecurityHubStateView {
            enabled_standards: vec![sub_view],
            ..Default::default()
        };
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
        for sub in &view.enabled_standards {
            let attrs = serde_json::json!({
                "id": sub.standards_subscription_arn,
                "standards_arn": sub.standards_arn,
                "standards_subscription_arn": sub.standards_subscription_arn,
                "standards_status": sub.standards_status,
                "standards_input": sub.standards_input,
            });
            results.push(ExtractedResource {
                name: sub.standards_subscription_arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
