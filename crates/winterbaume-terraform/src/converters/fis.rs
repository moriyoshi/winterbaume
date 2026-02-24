//! Terraform converters for AWS FIS resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_fis::FisService;
use winterbaume_fis::views::{
    ExperimentTemplateActionView, ExperimentTemplateStopConditionView,
    ExperimentTemplateTargetView, ExperimentTemplateView, FisStateView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_fis_experiment_template
// ---------------------------------------------------------------------------

pub struct AwsFisExperimentTemplateConverter {
    service: Arc<FisService>,
}

impl AwsFisExperimentTemplateConverter {
    pub fn new(service: Arc<FisService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsFisExperimentTemplateConverter {
    fn resource_type(&self) -> &str {
        "aws_fis_experiment_template"
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

impl AwsFisExperimentTemplateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let description = require_str(attrs, "description", "aws_fis_experiment_template")?;
        let role_arn = require_str(attrs, "role_arn", "aws_fis_experiment_template")?;
        let id = optional_str(attrs, "id").unwrap_or_else(|| format!("EXT{:016x}", rand_id()));
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:fis:{}:{}:experiment-template/{}",
                region, ctx.default_account_id, id
            )
        });
        let tags = extract_tags(attrs);

        // Parse stop conditions from Terraform nested blocks
        let stop_conditions = attrs
            .get("stop_condition")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|sc| {
                        let source = sc.get("source").and_then(|v| v.as_str())?;
                        let value = sc
                            .get("value")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        Some(ExperimentTemplateStopConditionView {
                            source: source.to_string(),
                            value,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Parse actions
        let actions = attrs
            .get("action")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|a| {
                        let name = a.get("name").and_then(|v| v.as_str())?;
                        let action_id = a.get("action_id").and_then(|v| v.as_str()).unwrap_or("");
                        let desc = a
                            .get("description")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        Some((
                            name.to_string(),
                            ExperimentTemplateActionView {
                                action_id: action_id.to_string(),
                                description: desc,
                                parameters: Default::default(),
                                targets: Default::default(),
                                start_after: Default::default(),
                            },
                        ))
                    })
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        // Parse targets
        let targets = attrs
            .get("target")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| {
                        let name = t.get("name").and_then(|v| v.as_str())?;
                        let resource_type = t
                            .get("resource_type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        let selection_mode = t
                            .get("selection_mode")
                            .and_then(|v| v.as_str())
                            .unwrap_or("ALL");
                        let resource_arns = t
                            .get("resource_arns")
                            .and_then(|v| v.as_array())
                            .map(|a| {
                                a.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default();
                        Some((
                            name.to_string(),
                            ExperimentTemplateTargetView {
                                resource_type: resource_type.to_string(),
                                resource_arns,
                                resource_tags: Default::default(),
                                filters: Default::default(),
                                selection_mode: selection_mode.to_string(),
                                parameters: Default::default(),
                            },
                        ))
                    })
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        let now = chrono::Utc::now().to_rfc3339();
        let template_view = ExperimentTemplateView {
            id: id.clone(),
            arn,
            description: description.to_string(),
            role_arn: role_arn.to_string(),
            targets,
            actions,
            stop_conditions,
            tags,
            creation_time: now.clone(),
            last_update_time: now,
        };

        let mut state_view = FisStateView::default();
        state_view.experiment_templates.insert(id, template_view);
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
        for template in view.experiment_templates.values() {
            let stop_conditions: Vec<serde_json::Value> = template
                .stop_conditions
                .iter()
                .map(|sc| {
                    serde_json::json!({
                        "source": sc.source,
                        "value": sc.value,
                    })
                })
                .collect();
            let actions: Vec<serde_json::Value> = template
                .actions
                .iter()
                .map(|(name, a)| {
                    serde_json::json!({
                        "name": name,
                        "action_id": a.action_id,
                        "description": a.description,
                    })
                })
                .collect();
            let targets: Vec<serde_json::Value> = template
                .targets
                .iter()
                .map(|(name, t)| {
                    serde_json::json!({
                        "name": name,
                        "resource_type": t.resource_type,
                        "selection_mode": t.selection_mode,
                        "resource_arns": t.resource_arns,
                    })
                })
                .collect();

            let attrs = serde_json::json!({
                "id": template.id,
                "arn": template.arn,
                "description": template.description,
                "role_arn": template.role_arn,
                "stop_condition": stop_conditions,
                "action": actions,
                "target": targets,
                "tags": template.tags,
            });
            results.push(ExtractedResource {
                name: template.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

fn rand_id() -> u128 {
    uuid::Uuid::new_v4().as_u128()
}
