//! Terraform converters for X-Ray resources.
//!
//! `GroupTfModel` and `SamplingRuleTfModel` are generated from
//! `specs/xray.toml`. The ARN templates, the
//! `insights_configuration` nested-block raw read on the group, the f64
//! `fixed_rate` / `created_at` / `modified_at` raw reads on the
//! sampling rule, and the `*` defaults for resource_arn / service_name
//! / service_type / host / http_method / url_path are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;
use winterbaume_xray::XRayService;
use winterbaume_xray::views::{GroupView, SamplingRuleView, XRayStateView};

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::xray as xray_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_xray_group
// ---------------------------------------------------------------------------

/// Converts `aws_xray_group` Terraform resources to/from X-Ray state.
pub struct AwsXrayGroupConverter {
    service: Arc<XRayService>,
}

impl AwsXrayGroupConverter {
    pub fn new(service: Arc<XRayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsXrayGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_xray_group"
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

impl AwsXrayGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: xray_gen::GroupTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_xray_group", e))?;

        let attrs = &instance.attributes;
        let group_name = model.group_name.clone();
        let filter_expression = model.filter_expression.unwrap_or_default();
        let group_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:xray:{}:{}:group/{}/{}",
                region, ctx.default_account_id, group_name, group_name
            )
        });

        // insights_configuration is a nested-block array — read raw.
        let insights_enabled = attrs
            .get("insights_configuration")
            .and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .and_then(|ic| ic.get("insights_enabled"))
            .and_then(|v| v.as_bool());
        let notifications_enabled = attrs
            .get("insights_configuration")
            .and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .and_then(|ic| ic.get("notifications_enabled"))
            .and_then(|v| v.as_bool());

        let group_view = GroupView {
            group_name: group_name.clone(),
            group_arn,
            filter_expression,
            insights_enabled,
            notifications_enabled,
        };

        let mut state_view = XRayStateView {
            groups: HashMap::new(),
            resource_policies: HashMap::new(),
            sampling_rules: HashMap::new(),
            ..Default::default()
        };
        state_view.groups.insert(group_name, group_view);
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
        for g in view.groups.values() {
            let mut attrs = serde_json::json!({
                "id": g.group_name,
                "group_name": g.group_name,
                "arn": g.group_arn,
                "filter_expression": g.filter_expression,
            });
            if g.insights_enabled.is_some() || g.notifications_enabled.is_some() {
                attrs["insights_configuration"] = serde_json::json!([{
                    "insights_enabled": g.insights_enabled.unwrap_or(false),
                    "notifications_enabled": g.notifications_enabled.unwrap_or(false),
                }]);
            }
            results.push(ExtractedResource {
                name: g.group_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_xray_sampling_rule
// ---------------------------------------------------------------------------

/// Converts `aws_xray_sampling_rule` Terraform resources to/from X-Ray state.
pub struct AwsXraySamplingRuleConverter {
    service: Arc<XRayService>,
}

impl AwsXraySamplingRuleConverter {
    pub fn new(service: Arc<XRayService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsXraySamplingRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_xray_sampling_rule"
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

impl AwsXraySamplingRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: xray_gen::SamplingRuleTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_xray_sampling_rule", e))?;

        let attrs = &instance.attributes;
        let rule_name = model.rule_name.clone();
        let rule_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:xray:{}:{}:sampling-rule/{}",
                region, ctx.default_account_id, rule_name
            )
        });
        let resource_arn = model.resource_arn.unwrap_or_else(|| "*".to_string());
        let service_name = model.service_name.unwrap_or_else(|| "*".to_string());
        let service_type = model.service_type.unwrap_or_else(|| "*".to_string());
        let host = model.host.unwrap_or_else(|| "*".to_string());
        let http_method = model.http_method.unwrap_or_else(|| "*".to_string());
        let url_path = model.url_path.unwrap_or_else(|| "*".to_string());

        // f64 not in spec vocabulary — read raw.
        let fixed_rate = attrs
            .get("fixed_rate")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.05);
        let created_at = attrs
            .get("created_at")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let modified_at = attrs
            .get("modified_at")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let rule_view = SamplingRuleView {
            rule_name: rule_name.clone(),
            rule_arn,
            resource_arn,
            priority: model.priority as i32,
            fixed_rate,
            reservoir_size: model.reservoir_size as i32,
            service_name,
            service_type,
            host,
            http_method,
            url_path,
            version: model.version as i32,
            created_at,
            modified_at,
        };

        let mut state_view = XRayStateView {
            groups: HashMap::new(),
            resource_policies: HashMap::new(),
            sampling_rules: HashMap::new(),
            ..Default::default()
        };
        state_view.sampling_rules.insert(rule_name, rule_view);
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
        for rule in view.sampling_rules.values() {
            let attrs = serde_json::json!({
                "id": rule.rule_name,
                "rule_name": rule.rule_name,
                "arn": rule.rule_arn,
                "resource_arn": rule.resource_arn,
                "priority": rule.priority,
                "fixed_rate": rule.fixed_rate,
                "reservoir_size": rule.reservoir_size,
                "service_name": rule.service_name,
                "service_type": rule.service_type,
                "host": rule.host,
                "http_method": rule.http_method,
                "url_path": rule.url_path,
                "version": rule.version,
                "created_at": rule.created_at,
                "modified_at": rule.modified_at,
            });
            results.push(ExtractedResource {
                name: rule.rule_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
