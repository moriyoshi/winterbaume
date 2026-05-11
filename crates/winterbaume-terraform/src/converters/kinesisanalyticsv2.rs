//! Terraform converter for Kinesis Analytics V2 resources.
//!
//! `ApplicationTfModel` is generated from `specs/kinesisanalyticsv2.toml`.
//! The runtime_environment default ("FLINK-1_15"), the application ARN
//! template, the constant `application_status = "READY"`, the
//! create_timestamp / last_update_timestamp values, and the opaque
//! `application_configuration` / `cloudwatch_logging_options` JSON
//! pass-through are wired up here.

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
use crate::generated::kinesisanalyticsv2 as kinesisanalyticsv2_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: kinesisanalyticsv2_gen::ApplicationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_kinesisanalyticsv2_application", e))?;

        // `application_configuration` and `cloudwatch_logging_options` flow
        // through as opaque JSON; the spec field-type vocabulary does not
        // include `serde_json::Value`, so they're read directly from the
        // raw attributes here.
        let application_configuration = instance
            .attributes
            .get("application_configuration")
            .cloned();
        let cloudwatch_logging_options = instance
            .attributes
            .get("cloudwatch_logging_options")
            .cloned();

        let runtime_environment = model
            .runtime_environment
            .unwrap_or_else(|| "FLINK-1_15".to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:kinesisanalytics:{}:{}:application/{}",
                region, ctx.default_account_id, model.name
            )
        });

        let now = chrono::Utc::now().to_rfc3339();
        let app_view = ApplicationView {
            application_name: model.name.clone(),
            application_arn: arn,
            application_status: "READY".to_string(),
            application_version_id: model.version_id,
            runtime_environment,
            service_execution_role: model.service_execution_role,
            create_timestamp: now.clone(),
            last_update_timestamp: now,
            application_description: model.description,
            tags: model.tags,
            snapshots: vec![],
            application_configuration,
            cloudwatch_logging_options,
        };

        let mut state_view = KinesisAnalyticsV2StateView {
            applications: HashMap::new(),
        };
        state_view.applications.insert(model.name, app_view);
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

// ---------------------------------------------------------------------------
// aws_kinesis_analytics_application (legacy v1 alias)
// ---------------------------------------------------------------------------

/// Converts the legacy `aws_kinesis_analytics_application` (v1 SQL Analytics)
/// resources by projecting them into the kinesisanalyticsv2 application slot.
/// This keeps state coherence with v2 consumers while accepting v1-shaped
/// Terraform attributes.
pub struct AwsKinesisAnalyticsApplicationConverter {
    service: Arc<KinesisAnalyticsV2Service>,
}

impl AwsKinesisAnalyticsApplicationConverter {
    pub fn new(service: Arc<KinesisAnalyticsV2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKinesisAnalyticsApplicationConverter {
    fn resource_type(&self) -> &str {
        "aws_kinesis_analytics_application"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let model: kinesisanalyticsv2_gen::AnalyticsApplicationV1TfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_kinesis_analytics_application", e)
                })?;

            let arn = model.arn.unwrap_or_else(|| {
                format!(
                    "arn:aws:kinesisanalytics:{}:{}:application/{}",
                    region, ctx.default_account_id, model.name
                )
            });

            let now = chrono::Utc::now().to_rfc3339();
            let app_view = ApplicationView {
                application_name: model.name.clone(),
                application_arn: arn,
                application_status: "READY".to_string(),
                application_version_id: model.version,
                runtime_environment: "SQL-1_0".to_string(),
                service_execution_role: None,
                create_timestamp: now.clone(),
                last_update_timestamp: now,
                application_description: model.description,
                tags: model.tags,
                snapshots: vec![],
                application_configuration: None,
                cloudwatch_logging_options: None,
            };

            let mut state_view = KinesisAnalyticsV2StateView {
                applications: HashMap::new(),
            };
            state_view.applications.insert(model.name, app_view);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Extraction is owned by the v2 converter; the legacy alias is
        // inject-only to avoid duplicate ExtractedResource entries for the
        // same backing state slot.
        Box::pin(async move { Ok(vec![]) })
    }
}
