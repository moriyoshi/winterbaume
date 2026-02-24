//! Terraform converters for AWS Application Cost Profiler resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_applicationcostprofiler::ApplicationCostProfilerService;
use winterbaume_applicationcostprofiler::views::{
    ApplicationCostProfilerStateView, ReportDefinitionView, S3LocationView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_applicationcostprofiler_report_definition
// ---------------------------------------------------------------------------

pub struct AwsApplicationCostProfilerReportDefinitionConverter {
    service: Arc<ApplicationCostProfilerService>,
}

impl AwsApplicationCostProfilerReportDefinitionConverter {
    pub fn new(service: Arc<ApplicationCostProfilerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsApplicationCostProfilerReportDefinitionConverter {
    fn resource_type(&self) -> &str {
        "aws_applicationcostprofiler_report_definition"
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

impl AwsApplicationCostProfilerReportDefinitionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let report_id = require_str(
            attrs,
            "report_id",
            "aws_applicationcostprofiler_report_definition",
        )?;
        let report_description = optional_str(attrs, "report_description").unwrap_or_default();
        let report_frequency =
            optional_str(attrs, "report_frequency").unwrap_or_else(|| "DAILY".to_string());
        let format = optional_str(attrs, "format").unwrap_or_else(|| "CSV".to_string());

        // destination_s3_location is a single-element block in terraform
        let dest_block = attrs
            .get("destination_s3_location")
            .and_then(|v| {
                v.as_array()
                    .and_then(|a| a.first().cloned())
                    .or_else(|| Some(v.clone()))
            })
            .ok_or_else(|| ConversionError::MissingAttribute {
                resource_type: "aws_applicationcostprofiler_report_definition".to_string(),
                attribute: "destination_s3_location".to_string(),
            })?;
        let bucket = dest_block
            .get("bucket")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ConversionError::MissingAttribute {
                resource_type: "aws_applicationcostprofiler_report_definition".to_string(),
                attribute: "destination_s3_location.bucket".to_string(),
            })?
            .to_string();
        let prefix = dest_block
            .get("prefix")
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_default();

        let now = chrono::Utc::now().timestamp();
        let mut state_view = ApplicationCostProfilerStateView::default();
        state_view.reports.insert(
            report_id.to_string(),
            ReportDefinitionView {
                report_id: report_id.to_string(),
                report_description,
                report_frequency,
                format,
                destination_s3_location: S3LocationView { bucket, prefix },
                created_at: now,
                last_updated_at: now,
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

        let mut resources = Vec::new();
        for (id, report) in &view.reports {
            let mut attrs = serde_json::Map::new();
            attrs.insert("id".to_string(), serde_json::json!(id));
            attrs.insert("report_id".to_string(), serde_json::json!(report.report_id));
            attrs.insert(
                "report_description".to_string(),
                serde_json::json!(report.report_description),
            );
            attrs.insert(
                "report_frequency".to_string(),
                serde_json::json!(report.report_frequency),
            );
            attrs.insert("format".to_string(), serde_json::json!(report.format));
            attrs.insert(
                "destination_s3_location".to_string(),
                serde_json::json!([{
                    "bucket": report.destination_s3_location.bucket,
                    "prefix": report.destination_s3_location.prefix,
                }]),
            );
            resources.push(ExtractedResource {
                name: id.clone(),
                attributes: serde_json::Value::Object(attrs),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
            });
        }

        Ok(resources)
    }
}
