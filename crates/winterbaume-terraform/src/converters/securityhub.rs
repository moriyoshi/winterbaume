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
    ActionTargetView, AutomationRuleView, ConfigPolicyAssociationView, ConfigPolicyView,
    FindingAggregatorView, HubInfoView, InsightView, MemberView, OrgAdminAccountView,
    OrgConfigView, ProductSubscriptionView, SecurityHubStateView, StandardsControlAssociationView,
    StandardsControlView, StandardsSubscriptionView,
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

// ---------------------------------------------------------------------------
// aws_securityhub_action_target
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_action_target` Terraform resources to/from SecurityHub state.
pub struct AwsSecurityhubActionTargetConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubActionTargetConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubActionTargetConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_action_target"
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

impl AwsSecurityhubActionTargetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::ActionTargetTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_securityhub_action_target", e))?;

        let arn = optional_str(&instance.attributes, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:securityhub:{}:{}:action/custom/{}",
                region, ctx.default_account_id, model.identifier
            )
        });

        let mut state_view = SecurityHubStateView::default();
        state_view.action_targets.insert(
            arn.clone(),
            ActionTargetView {
                arn,
                name: model.name,
                description: model.description,
            },
        );
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
        for t in view.action_targets.values() {
            let identifier = t.arn.rsplit_once('/').map(|(_, s)| s).unwrap_or(&t.arn);
            let attrs = serde_json::json!({
                "id": t.arn,
                "arn": t.arn,
                "name": t.name,
                "identifier": identifier,
                "description": t.description,
            });
            results.push(ExtractedResource {
                name: identifier.to_string(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_securityhub_automation_rule
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_automation_rule` Terraform resources to/from SecurityHub state.
pub struct AwsSecurityhubAutomationRuleConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubAutomationRuleConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubAutomationRuleConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_automation_rule"
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

impl AwsSecurityhubAutomationRuleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::AutomationRuleTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_securityhub_automation_rule", e))?;

        let rule_id = optional_str(&instance.attributes, "id")
            .or_else(|| optional_str(&instance.attributes, "arn"))
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let rule_arn = optional_str(&instance.attributes, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:securityhub:{}:{}:automation-rule/{}",
                region, ctx.default_account_id, rule_id
            )
        });
        let now = chrono::Utc::now().to_rfc3339();
        let rule_status = model.rule_status.unwrap_or_else(|| "ENABLED".to_string());

        let view = AutomationRuleView {
            rule_arn: rule_arn.clone(),
            rule_id: Some(rule_id),
            rule_name: model.rule_name,
            rule_order: model.rule_order as i32,
            rule_status,
            description: model.description,
            is_terminal: model.is_terminal,
            created_at: now.clone(),
            updated_at: now,
            raw: instance.attributes.clone(),
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.automation_rules.insert(rule_arn, view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
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
        for r in view.automation_rules.values() {
            let attrs = serde_json::json!({
                "id": r.rule_arn,
                "arn": r.rule_arn,
                "rule_name": r.rule_name,
                "rule_order": r.rule_order,
                "rule_status": r.rule_status,
                "description": r.description,
                "is_terminal": r.is_terminal,
            });
            results.push(ExtractedResource {
                name: r.rule_arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_securityhub_configuration_policy
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_configuration_policy` Terraform resources to/from SecurityHub state.
pub struct AwsSecurityhubConfigurationPolicyConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubConfigurationPolicyConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubConfigurationPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_configuration_policy"
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

impl AwsSecurityhubConfigurationPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::ConfigurationPolicyTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_securityhub_configuration_policy", e)
            })?;

        let id = optional_str(&instance.attributes, "id")
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = optional_str(&instance.attributes, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:securityhub:{}:{}:configuration-policy/{}",
                region, ctx.default_account_id, id
            )
        });
        let configuration_policy = instance.attributes.get("configuration_policy").cloned();
        let now = chrono::Utc::now().to_rfc3339();

        let view = ConfigPolicyView {
            id: id.clone(),
            arn,
            name: model.name,
            description: model.description,
            created_at: now.clone(),
            updated_at: now,
            configuration_policy,
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.config_policies.insert(id, view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
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
        for p in view.config_policies.values() {
            let attrs = serde_json::json!({
                "id": p.id,
                "arn": p.arn,
                "name": p.name,
                "description": p.description,
                "configuration_policy": p.configuration_policy,
            });
            results.push(ExtractedResource {
                name: p.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_securityhub_configuration_policy_association
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_configuration_policy_association` Terraform resources
/// to/from SecurityHub state.
pub struct AwsSecurityhubConfigurationPolicyAssociationConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubConfigurationPolicyAssociationConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubConfigurationPolicyAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_configuration_policy_association"
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

impl AwsSecurityhubConfigurationPolicyAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::ConfigurationPolicyAssociationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_securityhub_configuration_policy_association", e)
            })?;

        let target_type = if model.target_id.starts_with("ou-") {
            "ORGANIZATIONAL_UNIT".to_string()
        } else if model.target_id == "r-" || model.target_id.starts_with("r-") {
            "ROOT".to_string()
        } else {
            "ACCOUNT".to_string()
        };
        let now = chrono::Utc::now().to_rfc3339();

        let view = ConfigPolicyAssociationView {
            configuration_policy_id: model.policy_id,
            target_id: model.target_id.clone(),
            target_type,
            association_type: "APPLIED".to_string(),
            association_status: "SUCCESS".to_string(),
            association_status_message: None,
            updated_at: now,
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.policy_associations.insert(model.target_id, view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
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
        for a in view.policy_associations.values() {
            let attrs = serde_json::json!({
                "id": a.target_id,
                "policy_id": a.configuration_policy_id,
                "target_id": a.target_id,
            });
            results.push(ExtractedResource {
                name: a.target_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_securityhub_finding_aggregator
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_finding_aggregator` Terraform resources to/from SecurityHub state.
pub struct AwsSecurityhubFindingAggregatorConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubFindingAggregatorConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubFindingAggregatorConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_finding_aggregator"
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

impl AwsSecurityhubFindingAggregatorConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::FindingAggregatorTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_securityhub_finding_aggregator", e))?;

        let specified_regions = instance
            .attributes
            .get("specified_regions")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let arn = optional_str(&instance.attributes, "arn")
            .or_else(|| optional_str(&instance.attributes, "id"))
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:securityhub:{}:{}:finding-aggregator/{}",
                    region,
                    ctx.default_account_id,
                    uuid::Uuid::new_v4()
                )
            });

        let view = FindingAggregatorView {
            arn: arn.clone(),
            region_linking_mode: model.linking_mode,
            regions: specified_regions,
            finding_aggregation_region: region.clone(),
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.finding_aggregators.insert(arn, view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
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
        for a in view.finding_aggregators.values() {
            let attrs = serde_json::json!({
                "id": a.arn,
                "arn": a.arn,
                "linking_mode": a.region_linking_mode,
                "specified_regions": a.regions,
            });
            results.push(ExtractedResource {
                name: a.arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_securityhub_insight
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_insight` Terraform resources to/from SecurityHub state.
pub struct AwsSecurityhubInsightConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubInsightConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubInsightConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_insight"
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

impl AwsSecurityhubInsightConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::InsightTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_securityhub_insight", e))?;

        let arn = optional_str(&instance.attributes, "arn")
            .or_else(|| optional_str(&instance.attributes, "id"))
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:securityhub:{}:{}:insight/{}/custom/{}",
                    region,
                    ctx.default_account_id,
                    ctx.default_account_id,
                    uuid::Uuid::new_v4()
                )
            });
        let filters = instance
            .attributes
            .get("filters")
            .cloned()
            .unwrap_or(serde_json::Value::Null);

        let view = InsightView {
            arn: arn.clone(),
            name: model.name,
            group_by_attribute: model.group_by_attribute,
            filters,
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.insights.insert(arn, view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
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
        for i in view.insights.values() {
            let attrs = serde_json::json!({
                "id": i.arn,
                "arn": i.arn,
                "name": i.name,
                "group_by_attribute": i.group_by_attribute,
                "filters": i.filters,
            });
            results.push(ExtractedResource {
                name: i.arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_securityhub_invite_accepter
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_invite_accepter` Terraform resources to/from
/// SecurityHub state. Acceptance is a peer-account operation that mutates
/// the inviting account's member record; this converter is warning-only on
/// inject because Winterbaume's state is keyed per-account and lacks a
/// state slot for invitation acceptance from the recipient's perspective.
pub struct AwsSecurityhubInviteAccepterConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubInviteAccepterConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubInviteAccepterConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_invite_accepter"
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

impl AwsSecurityhubInviteAccepterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::InviteAccepterTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_securityhub_invite_accepter", e))?;

        Ok(ConversionResult {
            region,
            warnings: vec![format!(
                "aws_securityhub_invite_accepter: no state slot for master_id={} acceptance; ignored",
                model.master_id
            )],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_securityhub_member
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_member` Terraform resources to/from SecurityHub state.
pub struct AwsSecurityhubMemberConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubMemberConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubMemberConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_member"
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

impl AwsSecurityhubMemberConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::MemberTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_securityhub_member", e))?;

        let now = chrono::Utc::now().to_rfc3339();
        let member_status = if model.invite {
            "INVITED".to_string()
        } else {
            "CREATED".to_string()
        };
        let email = model.email.unwrap_or_default();

        let mut state_view = SecurityHubStateView::default();
        state_view.members.insert(
            model.account_id.clone(),
            MemberView {
                account_id: model.account_id,
                email,
                member_status,
                invited_at: now.clone(),
                updated_at: now,
            },
        );
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
        for m in view.members.values() {
            let attrs = serde_json::json!({
                "id": m.account_id,
                "account_id": m.account_id,
                "email": m.email,
                "master_id": ctx.default_account_id,
                "member_status": m.member_status,
                "invite": m.member_status == "INVITED" || m.member_status == "ASSOCIATED",
            });
            results.push(ExtractedResource {
                name: m.account_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_securityhub_organization_admin_account
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_organization_admin_account` Terraform resources
/// to/from SecurityHub state.
pub struct AwsSecurityhubOrganizationAdminAccountConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubOrganizationAdminAccountConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubOrganizationAdminAccountConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_organization_admin_account"
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

impl AwsSecurityhubOrganizationAdminAccountConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::OrganizationAdminAccountTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_securityhub_organization_admin_account", e)
            })?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.org_admin_account = Some(OrgAdminAccountView {
            account_id: model.admin_account_id,
            status: "ENABLED".to_string(),
        });
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
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
        if let Some(admin) = &view.org_admin_account {
            let attrs = serde_json::json!({
                "id": admin.account_id,
                "admin_account_id": admin.account_id,
            });
            results.push(ExtractedResource {
                name: admin.account_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_securityhub_organization_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_organization_configuration` Terraform resources
/// to/from SecurityHub state.
pub struct AwsSecurityhubOrganizationConfigurationConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubOrganizationConfigurationConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubOrganizationConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_organization_configuration"
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

impl AwsSecurityhubOrganizationConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::OrganizationConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_securityhub_organization_configuration", e)
            })?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.org_config = OrgConfigView {
            auto_enable: model.auto_enable,
            member_account_limit_reached: snapshot.org_config.member_account_limit_reached,
            auto_enable_standards: model
                .auto_enable_standards
                .unwrap_or_else(|| "DEFAULT".to_string()),
        };
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
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
        // Only emit if we have evidence the resource was set (non-default).
        if !view.org_config.auto_enable
            && view.org_config.auto_enable_standards.is_empty()
            && view.org_admin_account.is_none()
        {
            return Ok(vec![]);
        }
        let auto_enable_standards = if view.org_config.auto_enable_standards.is_empty() {
            "DEFAULT".to_string()
        } else {
            view.org_config.auto_enable_standards.clone()
        };
        let attrs = serde_json::json!({
            "id": ctx.default_account_id,
            "auto_enable": view.org_config.auto_enable,
            "auto_enable_standards": auto_enable_standards,
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
// aws_securityhub_product_subscription
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_product_subscription` Terraform resources to/from
/// SecurityHub state.
pub struct AwsSecurityhubProductSubscriptionConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubProductSubscriptionConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubProductSubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_product_subscription"
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

impl AwsSecurityhubProductSubscriptionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::ProductSubscriptionTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_securityhub_product_subscription", e)
            })?;

        // Derive a subscription ARN by replacing "product/" with "product-subscription/"
        // in the source product_arn (the same scheme Terraform/AWS uses).
        let subscription_arn = optional_str(&instance.attributes, "arn")
            .or_else(|| optional_str(&instance.attributes, "id"))
            .unwrap_or_else(|| {
                let suffix = model
                    .product_arn
                    .rsplit_once("product/")
                    .map(|(_, s)| s)
                    .unwrap_or(&model.product_arn);
                format!(
                    "arn:aws:securityhub:{}:{}:product-subscription/{}",
                    region, ctx.default_account_id, suffix
                )
            });

        let view = ProductSubscriptionView {
            product_subscription_arn: subscription_arn.clone(),
            product_arn: model.product_arn,
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot
            .product_subscriptions
            .insert(subscription_arn, view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
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
        for s in view.product_subscriptions.values() {
            let attrs = serde_json::json!({
                "id": s.product_subscription_arn,
                "arn": s.product_subscription_arn,
                "product_arn": s.product_arn,
            });
            results.push(ExtractedResource {
                name: s.product_subscription_arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_securityhub_standards_control
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_standards_control` Terraform resources to/from
/// SecurityHub state.
pub struct AwsSecurityhubStandardsControlConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubStandardsControlConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubStandardsControlConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_standards_control"
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

impl AwsSecurityhubStandardsControlConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::StandardsControlTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_securityhub_standards_control", e))?;

        let now = chrono::Utc::now().to_rfc3339();

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let entry = snapshot
            .standards_controls
            .entry(model.standards_control_arn.clone())
            .or_insert_with(|| StandardsControlView {
                standards_control_arn: model.standards_control_arn.clone(),
                control_status: model.control_status.clone(),
                disabled_reason: None,
                control_status_updated_at: None,
                control_id: None,
                title: None,
                description: None,
                remediation_url: None,
                severity_rating: None,
                related_requirements: vec![],
            });
        entry.control_status = model.control_status;
        entry.disabled_reason = model.disabled_reason;
        entry.control_status_updated_at = Some(now);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
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
        for c in view.standards_controls.values() {
            let attrs = serde_json::json!({
                "id": c.standards_control_arn,
                "standards_control_arn": c.standards_control_arn,
                "control_status": c.control_status,
                "disabled_reason": c.disabled_reason,
                "control_status_updated_at": c.control_status_updated_at,
                "control_id": c.control_id,
                "title": c.title,
                "description": c.description,
                "remediation_url": c.remediation_url,
                "severity_rating": c.severity_rating,
                "related_requirements": c.related_requirements,
            });
            results.push(ExtractedResource {
                name: c.standards_control_arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_securityhub_standards_control_association
// ---------------------------------------------------------------------------

/// Converts `aws_securityhub_standards_control_association` Terraform resources
/// to/from SecurityHub state.
pub struct AwsSecurityhubStandardsControlAssociationConverter {
    service: Arc<SecurityHubService>,
}

impl AwsSecurityhubStandardsControlAssociationConverter {
    pub fn new(service: Arc<SecurityHubService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecurityhubStandardsControlAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_securityhub_standards_control_association"
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

impl AwsSecurityhubStandardsControlAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: securityhub_gen::StandardsControlAssociationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_securityhub_standards_control_association", e)
            })?;

        let key = format!("{}::{}", model.security_control_id, model.standards_arn);
        let view = StandardsControlAssociationView {
            security_control_id: model.security_control_id,
            standards_arn: model.standards_arn,
            association_status: model.association_status,
            updated_reason: model.updated_reason,
            security_control_arn: None,
            standards_control_title: None,
            standards_control_description: None,
            standards_control_arns: vec![],
            related_requirements: vec![],
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        snapshot.standards_control_associations.insert(key, view);
        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
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
        for (key, a) in view.standards_control_associations.iter() {
            let attrs = serde_json::json!({
                "id": key,
                "security_control_id": a.security_control_id,
                "standards_arn": a.standards_arn,
                "association_status": a.association_status,
                "updated_reason": a.updated_reason,
            });
            results.push(ExtractedResource {
                name: key.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
