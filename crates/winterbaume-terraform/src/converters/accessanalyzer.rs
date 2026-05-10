//! Terraform converters for IAM Access Analyzer resources.
//!
//! `AnalyzerTfModel` is generated from `specs/accessanalyzer.toml`. The
//! `analyzer_type` default ("ACCOUNT"), the ARN template, and the
//! constant `status` ("ACTIVE") and `created_at`
//! ("2024-01-01T00:00:00.000Z") view fields are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_accessanalyzer::AccessAnalyzerService;
use winterbaume_accessanalyzer::views::{AccessAnalyzerStateView, AnalyzerView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::accessanalyzer as accessanalyzer_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_accessanalyzer_analyzer
// ---------------------------------------------------------------------------

pub struct AwsAccessAnalyzerAnalyzerConverter {
    service: Arc<AccessAnalyzerService>,
}

impl AwsAccessAnalyzerAnalyzerConverter {
    pub fn new(service: Arc<AccessAnalyzerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAccessAnalyzerAnalyzerConverter {
    fn resource_type(&self) -> &str {
        "aws_accessanalyzer_analyzer"
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

impl AwsAccessAnalyzerAnalyzerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: accessanalyzer_gen::AnalyzerTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_accessanalyzer_analyzer", e))?;

        let analyzer_name = model.analyzer_name.clone();
        let analyzer_type = model.analyzer_type.unwrap_or_else(|| "ACCOUNT".to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:access-analyzer:{region}:{account_id}:analyzer/{analyzer_name}",
                region = region,
                account_id = ctx.default_account_id,
                analyzer_name = analyzer_name,
            )
        });

        let analyzer_view = AnalyzerView {
            arn,
            name: analyzer_name.clone(),
            analyzer_type,
            status: "ACTIVE".to_string(),
            created_at: "2024-01-01T00:00:00.000Z".to_string(),
            tags: model.tags,
            archive_rules: HashMap::new(),
        };

        let mut state_view = AccessAnalyzerStateView::default();
        state_view.analyzers.insert(analyzer_name, analyzer_view);
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

        let mut resources = Vec::new();
        for (name, analyzer) in &view.analyzers {
            let mut attrs = serde_json::Map::new();
            attrs.insert("id".to_string(), serde_json::json!(name));
            attrs.insert(
                "analyzer_name".to_string(),
                serde_json::json!(analyzer.name),
            );
            attrs.insert("arn".to_string(), serde_json::json!(analyzer.arn));
            attrs.insert(
                "type".to_string(),
                serde_json::json!(analyzer.analyzer_type),
            );
            if !analyzer.tags.is_empty() {
                attrs.insert("tags".to_string(), serde_json::json!(analyzer.tags));
            }
            resources.push(ExtractedResource {
                name: name.clone(),
                attributes: serde_json::Value::Object(attrs),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
            });
        }

        Ok(resources)
    }
}
