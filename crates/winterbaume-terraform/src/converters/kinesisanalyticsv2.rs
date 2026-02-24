//! Terraform converter for Kinesis Analytics V2 resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_kinesisanalyticsv2::KinesisAnalyticsV2Service;
use winterbaume_kinesisanalyticsv2::views::{ApplicationView, KinesisAnalyticsV2StateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_i64, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_kinesisanalyticsv2_application
// ---------------------------------------------------------------------------

/// Converts `aws_kinesisanalyticsv2_application` Terraform resources to/from Kinesis Analytics V2 state.
pub struct AwsKinesisanalyticsv2ApplicationConverter {
    service: Arc<KinesisAnalyticsV2Service>,
}

impl AwsKinesisanalyticsv2ApplicationConverter {
    pub fn new(service: Arc<KinesisAnalyticsV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKinesisanalyticsv2ApplicationConverter {
    fn resource_type(&self) -> &str {
        "aws_kinesisanalyticsv2_application"
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

impl AwsKinesisanalyticsv2ApplicationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_kinesisanalyticsv2_application")?;
        let region = extract_region(attrs, &ctx.default_region);

        let runtime_environment =
            optional_str(attrs, "runtime_environment").unwrap_or_else(|| "FLINK-1_15".to_string());
        let service_execution_role = optional_str(attrs, "service_execution_role");
        let description = optional_str(attrs, "description");
        let version_id = optional_i64(attrs, "version_id").unwrap_or(1);

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:kinesisanalytics:{}:{}:application/{}",
                region, ctx.default_account_id, name
            )
        });
        let _start_application = attrs
            .get("start_application")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let _create_timestamp = optional_str(attrs, "create_timestamp");
        let _last_update_timestamp = optional_str(attrs, "last_update_timestamp");

        let now = chrono::Utc::now().to_rfc3339();
        let application_configuration = attrs.get("application_configuration").cloned();
        let cloudwatch_logging_options = attrs.get("cloudwatch_logging_options").cloned();
        let app_view = ApplicationView {
            application_name: name.to_string(),
            application_arn: arn,
            application_status: "READY".to_string(),
            application_version_id: version_id,
            runtime_environment,
            service_execution_role,
            create_timestamp: now.clone(),
            last_update_timestamp: now,
            application_description: description,
            tags: extract_tags(attrs),
            snapshots: vec![],
            application_configuration,
            cloudwatch_logging_options,
        };

        let mut state_view = KinesisAnalyticsV2StateView {
            applications: HashMap::new(),
        };
        state_view.applications.insert(name.to_string(), app_view);
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
        for app in view.applications.values() {
            let attrs = serde_json::json!({
                "id": app.application_name,
                "name": app.application_name,
                "arn": app.application_arn,
                "status": app.application_status,
                "version_id": app.application_version_id,
                "runtime_environment": app.runtime_environment,
                "service_execution_role": app.service_execution_role,
                "create_timestamp": app.create_timestamp,
                "last_update_timestamp": app.last_update_timestamp,
                "description": app.application_description,
                "tags": app.tags,
                "tags_all": app.tags,
                "start_application": false,
                "application_configuration": app.application_configuration,
                "cloudwatch_logging_options": app.cloudwatch_logging_options,
            });
            results.push(ExtractedResource {
                name: app.application_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
