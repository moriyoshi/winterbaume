//! Terraform converter for Macie2 resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_macie2::Macie2Service;
use winterbaume_macie2::views::{Macie2StateView, MacieClassificationJobView, MacieSessionView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_bool, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_macie2_account
// ---------------------------------------------------------------------------

/// Converts `aws_macie2_account` Terraform resources to/from Macie2 state.
pub struct AwsMacie2AccountConverter {
    service: Arc<Macie2Service>,
}

impl AwsMacie2AccountConverter {
    pub fn new(service: Arc<Macie2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMacie2AccountConverter {
    fn resource_type(&self) -> &str {
        "aws_macie2_account"
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

impl AwsMacie2AccountConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let finding_publishing_frequency = optional_str(attrs, "finding_publishing_frequency")
            .unwrap_or_else(|| "FIFTEEN_MINUTES".to_string());
        let status = optional_str(attrs, "status").unwrap_or_else(|| "ENABLED".to_string());
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = optional_str(attrs, "created_at").unwrap_or_else(|| now.clone());
        let updated_at = optional_str(attrs, "updated_at").unwrap_or_else(|| now.clone());
        let service_role = optional_str(attrs, "service_role").unwrap_or_else(|| {
            format!(
                "arn:aws:iam::{}:role/aws-service-role/macie.amazonaws.com/AWSServiceRoleForAmazonMacie",
                ctx.default_account_id
            )
        });

        let session_view = MacieSessionView {
            status,
            finding_publishing_frequency,
            created_at,
            updated_at,
            service_role,
        };

        let state_view = Macie2StateView {
            session: Some(session_view),
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
        if let Some(session) = &view.session {
            let attrs = serde_json::json!({
                "id": ctx.default_account_id,
                "status": session.status,
                "finding_publishing_frequency": session.finding_publishing_frequency,
                "created_at": session.created_at,
                "updated_at": session.updated_at,
                "service_role": session.service_role,
            });
            results.push(ExtractedResource {
                name: ctx.default_account_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_macie2_classification_job
// ---------------------------------------------------------------------------

/// Converts `aws_macie2_classification_job` Terraform resources to/from Macie2 state.
pub struct AwsMacie2ClassificationJobConverter {
    service: Arc<Macie2Service>,
}

impl AwsMacie2ClassificationJobConverter {
    pub fn new(service: Arc<Macie2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMacie2ClassificationJobConverter {
    fn resource_type(&self) -> &str {
        "aws_macie2_classification_job"
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

impl AwsMacie2ClassificationJobConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let job_type = require_str(attrs, "job_type", "aws_macie2_classification_job")?;
        let region = extract_region(attrs, &ctx.default_region);
        let job_id = optional_str(attrs, "job_id")
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let job_arn = optional_str(attrs, "job_arn").unwrap_or_else(|| {
            format!(
                "arn:aws:macie2:{}:{}:classification-job/{}",
                region, ctx.default_account_id, job_id
            )
        });
        let name = optional_str(attrs, "name").unwrap_or_default();
        let description = optional_str(attrs, "description");
        let job_status = optional_str(attrs, "job_status").unwrap_or_else(|| "RUNNING".to_string());
        let client_token =
            optional_str(attrs, "client_token").unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let s3_job_definition = attrs
            .get("s3_job_definition")
            .cloned()
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
        let initial_run = optional_bool(attrs, "initial_run").unwrap_or(false);
        let sampling_percentage = attrs
            .get("sampling_percentage")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let schedule_frequency = attrs.get("schedule_frequency").cloned();
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = optional_str(attrs, "created_at").unwrap_or_else(|| now.clone());
        let last_run_time = optional_str(attrs, "last_run_time");
        let tags = extract_tags(attrs);

        let allow_list_ids = attrs
            .get("allow_list_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let custom_data_identifier_ids = attrs
            .get("custom_data_identifier_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let managed_data_identifier_ids = attrs
            .get("managed_data_identifier_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let managed_data_identifier_selector =
            optional_str(attrs, "managed_data_identifier_selector");

        let job_view = MacieClassificationJobView {
            job_id: job_id.clone(),
            job_arn,
            name,
            description,
            job_type: job_type.to_string(),
            job_status,
            client_token,
            s3_job_definition,
            allow_list_ids,
            custom_data_identifier_ids,
            managed_data_identifier_ids,
            managed_data_identifier_selector,
            sampling_percentage,
            schedule_frequency,
            initial_run,
            tags,
            created_at,
            last_run_time,
        };

        let mut state_view = Macie2StateView::default();
        state_view.classification_jobs.insert(job_id, job_view);
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
        for job in view.classification_jobs.values() {
            let attrs = serde_json::json!({
                "id": job.job_id,
                "job_id": job.job_id,
                "job_arn": job.job_arn,
                "name": job.name,
                "description": job.description,
                "job_type": job.job_type,
                "job_status": job.job_status,
                "client_token": job.client_token,
                "s3_job_definition": job.s3_job_definition,
                "allow_list_ids": job.allow_list_ids,
                "custom_data_identifier_ids": job.custom_data_identifier_ids,
                "managed_data_identifier_ids": job.managed_data_identifier_ids,
                "managed_data_identifier_selector": job.managed_data_identifier_selector,
                "sampling_percentage": job.sampling_percentage,
                "schedule_frequency": job.schedule_frequency,
                "initial_run": job.initial_run,
                "tags": job.tags,
                "created_at": job.created_at,
                "last_run_time": job.last_run_time,
            });
            results.push(ExtractedResource {
                name: job.job_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
