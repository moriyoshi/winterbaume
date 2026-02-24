//! Terraform converter for AMP (Amazon Managed Service for Prometheus) resources.

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
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_prometheus_workspace
// ---------------------------------------------------------------------------

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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let workspace_id = require_str(attrs, "id", "aws_prometheus_workspace")?;
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:aps:{}:{}:workspace/{}",
                region, ctx.default_account_id, workspace_id
            )
        });
        let alias = optional_str(attrs, "alias");
        let prometheus_endpoint = optional_str(attrs, "prometheus_endpoint").unwrap_or_else(|| {
            format!(
                "https://aps-workspaces.{}.amazonaws.com/workspaces/{}/",
                region, workspace_id
            )
        });
        let tags = extract_tags(attrs);

        let ws_view = WorkspaceView {
            workspace_id: workspace_id.to_string(),
            arn,
            alias,
            status_code: "ACTIVE".to_string(),
            prometheus_endpoint,
            created_at: optional_str(attrs, "created_at"),
            tags,
        };

        let mut state_view = AmpStateView {
            ..Default::default()
        };
        state_view
            .workspaces
            .insert(ws_view.workspace_id.clone(), ws_view);
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
