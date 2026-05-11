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
use winterbaume_macie2::views::{
    Macie2StateView, MacieAdminAccountView, MacieClassificationExportConfigView,
    MacieClassificationJobView, MacieCustomDataIdentifierView, MacieFindingsFilterView,
    MacieMemberView, MacieOrgConfigView, MacieSessionView,
};
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

// ---------------------------------------------------------------------------
// aws_macie2_classification_export_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_macie2_classification_export_configuration` Terraform resources
/// to/from Macie2 state. The `s3_destination` block is preserved as a raw JSON
/// blob in the state view because its shape varies (bucket name, KMS key ARN,
/// optional key prefix) and is otherwise opaque to Winterbaume.
pub struct AwsMacie2ClassificationExportConfigurationConverter {
    service: Arc<Macie2Service>,
}

impl AwsMacie2ClassificationExportConfigurationConverter {
    pub fn new(service: Arc<Macie2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMacie2ClassificationExportConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_macie2_classification_export_configuration"
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

impl AwsMacie2ClassificationExportConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: macie2_gen::ClassificationExportConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_macie2_classification_export_configuration", e)
            })?;

        // The `s3_destination` block is a structured TF list — preserve as raw
        // JSON so extract can round-trip it without lossy reprojection.
        let raw = instance
            .attributes
            .get("s3_destination")
            .cloned()
            .unwrap_or(serde_json::Value::Null);

        let state_view = Macie2StateView {
            classification_export_config: Some(MacieClassificationExportConfigView { raw }),
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
        let Some(cfg) = &view.classification_export_config else {
            return Ok(vec![]);
        };
        let attrs = serde_json::json!({
            "id": ctx.default_account_id,
            "s3_destination": cfg.raw,
        });
        Ok(vec![ExtractedResource {
            name: ctx.default_account_id.clone(),
            account_id: ctx.default_account_id.clone(),
            region: ctx.default_region.clone(),
            attributes: attrs,
        }])
    }
}

// ---------------------------------------------------------------------------
// aws_macie2_custom_data_identifier
// ---------------------------------------------------------------------------

/// Converts `aws_macie2_custom_data_identifier` Terraform resources to/from
/// Macie2 state. The `keywords`, `ignore_words`, and `severity_levels` lists
/// are read raw from `instance.attributes`.
pub struct AwsMacie2CustomDataIdentifierConverter {
    service: Arc<Macie2Service>,
}

impl AwsMacie2CustomDataIdentifierConverter {
    pub fn new(service: Arc<Macie2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMacie2CustomDataIdentifierConverter {
    fn resource_type(&self) -> &str {
        "aws_macie2_custom_data_identifier"
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

impl AwsMacie2CustomDataIdentifierConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: macie2_gen::CustomDataIdentifierTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_macie2_custom_data_identifier", e))?;

        let attrs = &instance.attributes;
        let id = model
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = model.arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:macie2:{}:{}:custom-data-identifier/{}",
                region, ctx.default_account_id, id
            )
        });
        // Either `name` or a synthesized `<name_prefix><suffix>` is required
        // by Terraform; fall back to the id so the view always has something.
        let name = model
            .name
            .clone()
            .or_else(|| {
                model
                    .name_prefix
                    .as_ref()
                    .map(|p| format!("{p}{}", &id[..id.len().min(8)]))
            })
            .unwrap_or_else(|| id.clone());
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = model.created_at.clone().unwrap_or(now);

        // Vec<String> arrays — read raw.
        let keywords = attrs
            .get("keywords")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let ignore_words = attrs
            .get("ignore_words")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        // `severity_levels` is a list of { occurrences_threshold, severity }
        // blocks — project each entry into MacieSeverityLevelView.
        let severity_levels = attrs
            .get("severity_levels")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        let obj = item.as_object()?;
                        Some(winterbaume_macie2::views::MacieSeverityLevelView {
                            occurrences_threshold: obj
                                .get("occurrences_threshold")
                                .and_then(|v| v.as_i64())
                                .unwrap_or(0),
                            severity: obj
                                .get("severity")
                                .and_then(|v| v.as_str())
                                .unwrap_or("LOW")
                                .to_string(),
                        })
                    })
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

        let cdi_view = MacieCustomDataIdentifierView {
            id: id.clone(),
            arn,
            name,
            description: model.description,
            regex: model.regex,
            keywords,
            ignore_words,
            maximum_match_distance: model.maximum_match_distance as i32,
            severity_levels,
            tags,
            created_at,
            deleted: false,
        };

        let mut state_view = Macie2StateView::default();
        state_view.custom_data_identifiers.insert(id, cdi_view);
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
        for cdi in view.custom_data_identifiers.values() {
            if cdi.deleted {
                continue;
            }
            let severity_levels: Vec<serde_json::Value> = cdi
                .severity_levels
                .iter()
                .map(|sl| {
                    serde_json::json!({
                        "occurrences_threshold": sl.occurrences_threshold,
                        "severity": sl.severity,
                    })
                })
                .collect();
            let attrs = serde_json::json!({
                "id": cdi.id,
                "arn": cdi.arn,
                "name": cdi.name,
                "description": cdi.description,
                "regex": cdi.regex,
                "keywords": cdi.keywords,
                "ignore_words": cdi.ignore_words,
                "maximum_match_distance": cdi.maximum_match_distance,
                "severity_levels": severity_levels,
                "created_at": cdi.created_at,
                "tags": cdi.tags,
            });
            results.push(ExtractedResource {
                name: cdi.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_macie2_findings_filter
// ---------------------------------------------------------------------------

/// Converts `aws_macie2_findings_filter` Terraform resources to/from Macie2
/// state. The `finding_criteria` block is preserved as raw JSON because its
/// nested `criterion` map has variable shape.
pub struct AwsMacie2FindingsFilterConverter {
    service: Arc<Macie2Service>,
}

impl AwsMacie2FindingsFilterConverter {
    pub fn new(service: Arc<Macie2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMacie2FindingsFilterConverter {
    fn resource_type(&self) -> &str {
        "aws_macie2_findings_filter"
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

impl AwsMacie2FindingsFilterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: macie2_gen::FindingsFilterTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_macie2_findings_filter", e))?;

        let attrs = &instance.attributes;
        let id = model
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = model.arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:macie2:{}:{}:findings-filter/{}",
                region, ctx.default_account_id, id
            )
        });
        let name = model
            .name
            .clone()
            .or_else(|| {
                model
                    .name_prefix
                    .as_ref()
                    .map(|p| format!("{p}{}", &id[..id.len().min(8)]))
            })
            .unwrap_or_else(|| id.clone());
        // Raw JSON block — read raw.
        let finding_criteria = attrs
            .get("finding_criteria")
            .cloned()
            .unwrap_or(serde_json::Value::Null);

        // Merge `tags_all` into `tags` (tags wins).
        let mut tags = model.tags;
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let ff_view = MacieFindingsFilterView {
            id: id.clone(),
            arn,
            name,
            description: model.description,
            action: model.action,
            position: model.position as i32,
            finding_criteria,
            tags,
        };

        let mut state_view = Macie2StateView::default();
        state_view.findings_filters.insert(id, ff_view);
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
        for ff in view.findings_filters.values() {
            let attrs = serde_json::json!({
                "id": ff.id,
                "arn": ff.arn,
                "name": ff.name,
                "description": ff.description,
                "action": ff.action,
                "position": ff.position,
                "finding_criteria": ff.finding_criteria,
                "tags": ff.tags,
            });
            results.push(ExtractedResource {
                name: ff.id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_macie2_invitation_accepter
// ---------------------------------------------------------------------------

/// Converts `aws_macie2_invitation_accepter` Terraform resources to/from
/// Macie2 state. Acceptance is a peer-account operation: the recipient account
/// records its administrator. We populate the `administrator` view slot on
/// the current account so subsequent GetAdministratorAccount calls observe
/// the relationship.
pub struct AwsMacie2InvitationAccepterConverter {
    service: Arc<Macie2Service>,
}

impl AwsMacie2InvitationAccepterConverter {
    pub fn new(service: Arc<Macie2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMacie2InvitationAccepterConverter {
    fn resource_type(&self) -> &str {
        "aws_macie2_invitation_accepter"
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

impl AwsMacie2InvitationAccepterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: macie2_gen::InvitationAccepterTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_macie2_invitation_accepter", e))?;

        let invitation_id = model
            .invitation_id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();

        let state_view = Macie2StateView {
            administrator: Some(winterbaume_macie2::views::MacieAdministratorView {
                account_id: model.administrator_account_id,
                invitation_id,
                invited_at: now,
                relationship_status: "Enabled".to_string(),
            }),
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
        let Some(admin) = &view.administrator else {
            return Ok(vec![]);
        };
        let attrs = serde_json::json!({
            "id": ctx.default_account_id,
            "administrator_account_id": admin.account_id,
            "invitation_id": admin.invitation_id,
        });
        Ok(vec![ExtractedResource {
            name: ctx.default_account_id.clone(),
            account_id: ctx.default_account_id.clone(),
            region: ctx.default_region.clone(),
            attributes: attrs,
        }])
    }
}

// ---------------------------------------------------------------------------
// aws_macie2_member
// ---------------------------------------------------------------------------

/// Converts `aws_macie2_member` Terraform resources to/from Macie2 state.
pub struct AwsMacie2MemberConverter {
    service: Arc<Macie2Service>,
}

impl AwsMacie2MemberConverter {
    pub fn new(service: Arc<Macie2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMacie2MemberConverter {
    fn resource_type(&self) -> &str {
        "aws_macie2_member"
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

impl AwsMacie2MemberConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: macie2_gen::MemberTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_macie2_member", e))?;

        let now = chrono::Utc::now().to_rfc3339();
        let relationship_status = model.relationship_status.unwrap_or_else(|| {
            if model.invite {
                "Invited".to_string()
            } else {
                "Created".to_string()
            }
        });
        let invited_at = if model.invite {
            Some(model.invited_at.unwrap_or_else(|| now.clone()))
        } else {
            model.invited_at
        };
        let updated_at = model.updated_at.unwrap_or(now);

        let member_view = MacieMemberView {
            account_id: model.account_id.clone(),
            email: model.email,
            relationship_status,
            invited_at,
            updated_at,
        };

        let mut state_view = Macie2StateView::default();
        state_view.members.insert(model.account_id, member_view);
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
        for m in view.members.values() {
            let arn = format!(
                "arn:aws:macie2:{}:{}:member/{}",
                ctx.default_region, ctx.default_account_id, m.account_id
            );
            let invite = matches!(
                m.relationship_status.as_str(),
                "Invited" | "Enabled" | "Paused"
            );
            let attrs = serde_json::json!({
                "id": m.account_id,
                "account_id": m.account_id,
                "arn": arn,
                "email": m.email,
                "relationship_status": m.relationship_status,
                "invited_at": m.invited_at,
                "updated_at": m.updated_at,
                "administrator_account_id": ctx.default_account_id,
                "master_account_id": ctx.default_account_id,
                "invite": invite,
            });
            results.push(ExtractedResource {
                name: m.account_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_macie2_organization_admin_account
// ---------------------------------------------------------------------------

/// Converts `aws_macie2_organization_admin_account` Terraform resources to/from
/// Macie2 state.
pub struct AwsMacie2OrganizationAdminAccountConverter {
    service: Arc<Macie2Service>,
}

impl AwsMacie2OrganizationAdminAccountConverter {
    pub fn new(service: Arc<Macie2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMacie2OrganizationAdminAccountConverter {
    fn resource_type(&self) -> &str {
        "aws_macie2_organization_admin_account"
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

impl AwsMacie2OrganizationAdminAccountConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: macie2_gen::OrganizationAdminAccountTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_macie2_organization_admin_account", e)
            })?;

        // Read existing snapshot so we accumulate admin accounts rather than
        // clobber the list. `Macie2StateView::admin_accounts` merge appends.
        let state_view = Macie2StateView {
            admin_accounts: vec![MacieAdminAccountView {
                account_id: model.admin_account_id,
                status: "ENABLED".to_string(),
            }],
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
        for a in view.admin_accounts.iter() {
            let attrs = serde_json::json!({
                "id": a.account_id,
                "admin_account_id": a.account_id,
            });
            results.push(ExtractedResource {
                name: a.account_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_macie2_organization_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_macie2_organization_configuration` Terraform resources to/from
/// Macie2 state.
pub struct AwsMacie2OrganizationConfigurationConverter {
    service: Arc<Macie2Service>,
}

impl AwsMacie2OrganizationConfigurationConverter {
    pub fn new(service: Arc<Macie2Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMacie2OrganizationConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_macie2_organization_configuration"
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

impl AwsMacie2OrganizationConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: macie2_gen::OrganizationConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_macie2_organization_configuration", e)
            })?;

        let state_view = Macie2StateView {
            org_config: Some(MacieOrgConfigView {
                auto_enable: model.auto_enable,
                max_account_limit_reached: false,
            }),
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
        let Some(cfg) = &view.org_config else {
            return Ok(vec![]);
        };
        let attrs = serde_json::json!({
            "id": ctx.default_account_id,
            "auto_enable": cfg.auto_enable,
        });
        Ok(vec![ExtractedResource {
            name: ctx.default_account_id.clone(),
            account_id: ctx.default_account_id.clone(),
            region: ctx.default_region.clone(),
            attributes: attrs,
        }])
    }
}
