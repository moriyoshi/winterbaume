//! Terraform converter for AMP (Amazon Managed Service for Prometheus) resources.
//!
//! `WorkspaceTfModel` is generated from `specs/amp.toml`. The constant
//! view fields (`status_code = "ACTIVE"`) and the two URL templates
//! (`arn`, `prometheus_endpoint`) are wired up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_amp::AmpService;
use winterbaume_amp::views::{AmpStateView, WorkspaceView};
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
