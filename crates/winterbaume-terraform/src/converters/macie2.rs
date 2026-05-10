//! Terraform converters for Macie2 resources.
//!
//! `AccountTfModel` and `ClassificationJobTfModel` are generated from
//! `specs/macie2.toml`. The synthesised job IDs and ARN templates, the
//! default service-role IAM ARN, the `s3_job_definition` /
//! `schedule_frequency` raw-JSON blobs, the `allow_list_ids` /
//! `custom_data_identifier_ids` / `managed_data_identifier_ids`
//! Vec<String> arrays, and the Option<i32> `sampling_percentage` are
//! wired up here.

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
use crate::generated::macie2 as macie2_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: macie2_gen::AccountTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_macie2_account", e))?;

        let finding_publishing_frequency = model
            .finding_publishing_frequency
            .unwrap_or_else(|| "FIFTEEN_MINUTES".to_string());
        let status = model.status.unwrap_or_else(|| "ENABLED".to_string());
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = model.created_at.unwrap_or_else(|| now.clone());
        let updated_at = model.updated_at.unwrap_or_else(|| now.clone());
        let service_role = model.service_role.unwrap_or_else(|| {
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: macie2_gen::ClassificationJobTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_macie2_classification_job", e))?;

        let attrs = &instance.attributes;

        let job_id = model
            .job_id
            .or(model.id)
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let job_arn = model.job_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:macie2:{}:{}:classification-job/{}",
                region, ctx.default_account_id, job_id
            )
        });
        let name = model.name.unwrap_or_default();
        let job_status = model.job_status.unwrap_or_else(|| "RUNNING".to_string());
        let client_token = model
            .client_token
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        // Raw JSON blob — read raw.
        let s3_job_definition = attrs
            .get("s3_job_definition")
            .cloned()
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
        // Option<i32> not in spec vocabulary — read raw.
        let sampling_percentage = attrs
            .get("sampling_percentage")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        // Raw JSON object — read raw.
        let schedule_frequency = attrs.get("schedule_frequency").cloned();
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = model.created_at.unwrap_or_else(|| now.clone());

        // Vec<String> arrays — read raw.
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

        // Merge `tags_all` into `tags` (tags wins).
        let mut tags = model.tags;
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let job_view = MacieClassificationJobView {
            job_id: job_id.clone(),
            job_arn,
            name,
            description: model.description,
            job_type: model.job_type,
            job_status,
            client_token,
            s3_job_definition,
            allow_list_ids,
            custom_data_identifier_ids,
            managed_data_identifier_ids,
            managed_data_identifier_selector: model.managed_data_identifier_selector,
            sampling_percentage,
            schedule_frequency,
            initial_run: model.initial_run,
            tags,
            created_at,
            last_run_time: model.last_run_time,
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
