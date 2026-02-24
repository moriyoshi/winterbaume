//! Terraform converter for Synthetics resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_synthetics::SyntheticsService;
use winterbaume_synthetics::views::{CanaryView, SyntheticsStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_i64, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_synthetics_canary
// ---------------------------------------------------------------------------

/// Converts `aws_synthetics_canary` Terraform resources to/from Synthetics state.
pub struct AwsSyntheticsCanaryConverter {
    service: Arc<SyntheticsService>,
}

impl AwsSyntheticsCanaryConverter {
    pub fn new(service: Arc<SyntheticsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSyntheticsCanaryConverter {
    fn resource_type(&self) -> &str {
        "aws_synthetics_canary"
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

impl AwsSyntheticsCanaryConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_synthetics_canary")?;
        let _tags_all = attrs.get("tags_all");
        let _start_canary = optional_str(attrs, "start_canary");
        let _delete_lambda = optional_str(attrs, "delete_lambda");
        let artifact_config = attrs.get("artifact_config").cloned();
        let run_config = attrs.get("run_config").cloned();
        let vpc_config = attrs.get("vpc_config").cloned();
        let id = optional_str(attrs, "id").unwrap_or_else(|| name.to_string());
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:synthetics:{}:{}:canary:{}",
                region, ctx.default_account_id, name
            )
        });
        let artifact_s3_location = optional_str(attrs, "artifact_s3_location").unwrap_or_default();
        let runtime_version = optional_str(attrs, "runtime_version")
            .unwrap_or_else(|| "syn-nodejs-puppeteer-6.2".to_string());
        let handler = require_str(attrs, "handler", "aws_synthetics_canary")?;
        let schedule_expression = optional_str(attrs, "schedule_expression")
            .or_else(|| {
                attrs
                    .get("schedule")
                    .and_then(|v| v.as_array())
                    .and_then(|a| a.first())
                    .and_then(|s| s.get("expression"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "rate(5 minutes)".to_string());
        let schedule_duration_in_seconds = optional_i64(attrs, "schedule_duration_in_seconds")
            .or_else(|| {
                attrs
                    .get("schedule")
                    .and_then(|v| v.as_array())
                    .and_then(|a| a.first())
                    .and_then(|s| s.get("duration_in_seconds"))
                    .and_then(|v| v.as_i64())
            });
        let success_retention_period_in_days = attrs
            .get("success_retention_period")
            .and_then(|v| v.as_i64())
            .unwrap_or(31) as i32;
        let failure_retention_period_in_days = attrs
            .get("failure_retention_period")
            .and_then(|v| v.as_i64())
            .unwrap_or(31) as i32;
        let status_state = optional_str(attrs, "status").unwrap_or_else(|| "READY".to_string());
        let status_state_reason = optional_str(attrs, "status_state_reason");
        let status_state_reason_code = optional_str(attrs, "status_state_reason_code");
        let created_at = optional_str(attrs, "created_at").unwrap_or_default();
        let last_modified = optional_str(attrs, "last_modified").unwrap_or_default();
        let execution_role_arn = optional_str(attrs, "execution_role_arn").unwrap_or_default();
        let s3_encryption_mode = optional_str(attrs, "s3_encryption_mode");
        let tags = extract_tags(attrs);

        let canary_view = CanaryView {
            name: name.to_string(),
            id: id.clone(),
            arn,
            artifact_s3_location,
            runtime_version,
            handler: handler.to_string(),
            schedule_expression,
            schedule_duration_in_seconds,
            success_retention_period_in_days,
            failure_retention_period_in_days,
            status_state,
            status_state_reason,
            status_state_reason_code,
            created_at,
            last_modified,
            execution_role_arn,
            s3_encryption_mode,
            tags,
            artifact_config,
            run_config,
            vpc_config,
        };

        let mut state_view = SyntheticsStateView {
            canaries: HashMap::new(),
            groups: HashMap::new(),
        };
        state_view.canaries.insert(name.to_string(), canary_view);
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
        for canary in view.canaries.values() {
            let attrs = serde_json::json!({
                "id": canary.id,
                "name": canary.name,
                "arn": canary.arn,
                "artifact_s3_location": canary.artifact_s3_location,
                "runtime_version": canary.runtime_version,
                "handler": canary.handler,
                "schedule_expression": canary.schedule_expression,
                "schedule_duration_in_seconds": canary.schedule_duration_in_seconds,
                "success_retention_period": canary.success_retention_period_in_days,
                "failure_retention_period": canary.failure_retention_period_in_days,
                "status": canary.status_state,
                "status_state_reason": canary.status_state_reason,
                "status_state_reason_code": canary.status_state_reason_code,
                "created_at": canary.created_at,
                "last_modified": canary.last_modified,
                "execution_role_arn": canary.execution_role_arn,
                "s3_encryption_mode": canary.s3_encryption_mode,
                "source_location_arn": format!("arn:aws:lambda:{}:{}:layer:cwsyn-{}", ctx.default_region, ctx.default_account_id, canary.name),
                "tags": canary.tags,
                "tags_all": canary.tags,
                "artifact_config": canary.artifact_config,
                "run_config": canary.run_config,
                "vpc_config": canary.vpc_config,
                "schedule": [{"expression": canary.schedule_expression, "duration_in_seconds": canary.schedule_duration_in_seconds}],
            });
            results.push(ExtractedResource {
                name: canary.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
