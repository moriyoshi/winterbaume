//! Terraform converter for AMP (Amazon Managed Service for Prometheus) resources.
//!
//! `WorkspaceTfModel` is generated from `specs/amp.toml`. The constant
//! view fields (`status_code = "ACTIVE"`) and the two URL templates
//! (`arn`, `prometheus_endpoint`) are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_amp::AmpService;
use winterbaume_amp::views::{AmpStateView, RuleGroupsNamespaceView, WorkspaceView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::amp as amp_gen;
use crate::util::{classify_deserialize_error, extract_region};

pub struct AwsPrometheusWorkspaceConverter {
    service: Arc<AmpService>,
}

impl AwsPrometheusWorkspaceConverter {
    pub fn new(service: Arc<AmpService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsPrometheusWorkspaceConverter {
    fn resource_type(&self) -> &str {
        "aws_prometheus_workspace"
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

impl AwsPrometheusWorkspaceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: amp_gen::WorkspaceTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_prometheus_workspace", e))?;

        let workspace_id = model.id.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:aps:{}:{}:workspace/{}",
                region, ctx.default_account_id, workspace_id
            )
        });
        let prometheus_endpoint = model.prometheus_endpoint.unwrap_or_else(|| {
            format!(
                "https://aps-workspaces.{}.amazonaws.com/workspaces/{}/",
                region, workspace_id
            )
        });

        let ws_view = WorkspaceView {
            workspace_id: workspace_id.clone(),
            arn,
            alias: model.alias,
            status_code: "ACTIVE".to_string(),
            prometheus_endpoint,
            created_at: model.created_at,
            tags: model.tags,
        };

        let mut state_view = AmpStateView {
            ..Default::default()
        };
        state_view.workspaces.insert(workspace_id, ws_view);
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
        for ws in view.workspaces.values() {
            let attrs = serde_json::json!({
                "id": ws.workspace_id,
                "arn": ws.arn,
                "alias": ws.alias,
                "prometheus_endpoint": ws.prometheus_endpoint,
                "created_at": ws.created_at,
                "tags": ws.tags,
            });
            results.push(ExtractedResource {
                name: ws.workspace_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_prometheus_rule_group_namespace
// ---------------------------------------------------------------------------

pub struct AwsPrometheusRuleGroupNamespaceConverter {
    service: Arc<AmpService>,
}

impl AwsPrometheusRuleGroupNamespaceConverter {
    pub fn new(service: Arc<AmpService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsPrometheusRuleGroupNamespaceConverter {
    fn resource_type(&self) -> &str {
        "aws_prometheus_rule_group_namespace"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_prometheus_workspace"]
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

impl AwsPrometheusRuleGroupNamespaceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: amp_gen::RuleGroupsNamespaceTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_prometheus_rule_group_namespace", e)
            })?;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:aps:{}:{}:rulegroupsnamespace/{}/{}",
                region, ctx.default_account_id, model.workspace_id, model.name
            )
        });

        let ns_view = RuleGroupsNamespaceView {
            name: model.name.clone(),
            arn,
            workspace_id: model.workspace_id.clone(),
            data: model.data.unwrap_or_default(),
            status_code: "ACTIVE".to_string(),
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            modified_at: Some(chrono::Utc::now().to_rfc3339()),
            tags: HashMap::new(),
        };

        let mut state_view = AmpStateView::default();
        let mut inner = HashMap::new();
        inner.insert(model.name, ns_view);
        state_view
            .rule_groups_namespaces
            .insert(model.workspace_id, inner);
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
        for (workspace_id, namespaces) in &view.rule_groups_namespaces {
            for ns in namespaces.values() {
                let attrs = serde_json::json!({
                    "id": ns.arn,
                    "name": ns.name,
                    "workspace_id": workspace_id,
                    "arn": ns.arn,
                    "data": ns.data,
                    "tags": ns.tags,
                });
                results.push(ExtractedResource {
                    name: format!("{workspace_id}_{}", ns.name),
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
// aws_prometheus_alert_manager_definition — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_prometheus_alert_manager_definition` resources (validation-only;
/// no backing state slot in `winterbaume_amp`).
pub struct AwsPrometheusAlertManagerDefinitionConverter {
    #[allow(dead_code)]
    service: Arc<AmpService>,
}

impl AwsPrometheusAlertManagerDefinitionConverter {
    pub fn new(service: Arc<AmpService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsPrometheusAlertManagerDefinitionConverter {
    fn resource_type(&self) -> &str {
        "aws_prometheus_alert_manager_definition"
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

impl AwsPrometheusAlertManagerDefinitionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: amp_gen::AlertManagerDefinitionTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_prometheus_alert_manager_definition", e)
            })?;
        let warn_msg = "no state slot in winterbaume_amp for alert manager definitions; \
             inject is a no-op"
            .to_string();
        eprintln!("warning: aws_prometheus_alert_manager_definition: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!(
                "aws_prometheus_alert_manager_definition: {warn_msg}"
            )],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_prometheus_scraper — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_prometheus_scraper` resources (validation-only; no backing
/// state slot in `winterbaume_amp`).
pub struct AwsPrometheusScraperConverter {
    #[allow(dead_code)]
    service: Arc<AmpService>,
}

impl AwsPrometheusScraperConverter {
    pub fn new(service: Arc<AmpService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsPrometheusScraperConverter {
    fn resource_type(&self) -> &str {
        "aws_prometheus_scraper"
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

impl AwsPrometheusScraperConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: amp_gen::ScraperTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_prometheus_scraper", e))?;
        let warn_msg =
            "no state slot in winterbaume_amp for scrapers; inject is a no-op".to_string();
        eprintln!("warning: aws_prometheus_scraper: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!("aws_prometheus_scraper: {warn_msg}")],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_prometheus_workspace_configuration — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_prometheus_workspace_configuration` resources (validation-only;
/// no backing state slot in `winterbaume_amp`).
pub struct AwsPrometheusWorkspaceConfigurationConverter {
    #[allow(dead_code)]
    service: Arc<AmpService>,
}

impl AwsPrometheusWorkspaceConfigurationConverter {
    pub fn new(service: Arc<AmpService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsPrometheusWorkspaceConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_prometheus_workspace_configuration"
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

impl AwsPrometheusWorkspaceConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: amp_gen::WorkspaceConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_prometheus_workspace_configuration", e)
            })?;
        let warn_msg = "no state slot in winterbaume_amp for workspace configurations; \
             inject is a no-op"
            .to_string();
        eprintln!("warning: aws_prometheus_workspace_configuration: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!(
                "aws_prometheus_workspace_configuration: {warn_msg}"
            )],
        })
    }
}
