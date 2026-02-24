//! Terraform converter for Timestream Query resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;
use winterbaume_timestreamquery::TimestreamQueryService;
use winterbaume_timestreamquery::views::{ScheduledQueryView, TimestreamQueryStateView};

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_timestreamquery_scheduled_query
// ---------------------------------------------------------------------------

/// Converts `aws_timestreamquery_scheduled_query` Terraform resources to/from Timestream Query state.
pub struct AwsTimestreamQueryScheduledQueryConverter {
    service: Arc<TimestreamQueryService>,
}

impl AwsTimestreamQueryScheduledQueryConverter {
    pub fn new(service: Arc<TimestreamQueryService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsTimestreamQueryScheduledQueryConverter {
    fn resource_type(&self) -> &str {
        "aws_timestreamquery_scheduled_query"
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

impl AwsTimestreamQueryScheduledQueryConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_timestreamquery_scheduled_query")?;
        let region = extract_region(attrs, &ctx.default_region);
        let query_string = optional_str(attrs, "query_string").unwrap_or_default();
        let schedule_expression = optional_str(attrs, "schedule_expression").unwrap_or_default();
        let target_database = optional_str(attrs, "target_database").unwrap_or_default();
        let target_table = optional_str(attrs, "target_table").unwrap_or_default();
        let s3_error_report_bucket = optional_str(attrs, "s3_error_report_bucket");
        let notification_topic_arn =
            optional_str(attrs, "notification_topic_arn").unwrap_or_default();
        let role_arn = optional_str(attrs, "role_arn").unwrap_or_default();
        let state_str = optional_str(attrs, "state").unwrap_or_else(|| "ENABLED".to_string());
        let last_run_status = optional_str(attrs, "last_run_status");

        let _tags_all = attrs.get("tags_all");
        let _kms_key_id = optional_str(attrs, "kms_key_id");
        let error_report_configuration = attrs
            .get("error_report_configuration")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let target_configuration = attrs
            .get("target_configuration")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let notification_configuration = attrs
            .get("notification_configuration")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let schedule_configuration = attrs
            .get("schedule_configuration")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:timestream:{}:{}:scheduled-query/{}",
                region, ctx.default_account_id, name
            )
        });

        let sq_view = ScheduledQueryView {
            arn: arn.clone(),
            name: name.to_string(),
            query_string,
            state: state_str,
            schedule_expression,
            target_database,
            target_table,
            s3_error_report_bucket,
            creation_time: optional_str(attrs, "creation_time")
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            last_run_status,
            notification_topic_arn,
            role_arn,
            error_report_configuration,
            target_configuration,
            notification_configuration,
            schedule_configuration,
            recently_failed_runs: vec![],
            last_run_summary: None,
        };

        // Extract tags for resource_tags
        let mut resource_tags: HashMap<String, HashMap<String, String>> = HashMap::new();
        if let Some(tags_obj) = attrs.get("tags").and_then(|v| v.as_object()) {
            let mut tags = HashMap::new();
            for (k, v) in tags_obj {
                if let Some(s) = v.as_str() {
                    tags.insert(k.clone(), s.to_string());
                }
            }
            if !tags.is_empty() {
                resource_tags.insert(arn.clone(), tags);
            }
        }

        let mut state_view = TimestreamQueryStateView {
            scheduled_queries: HashMap::new(),
            account_settings: Default::default(),
            resource_tags,
        };
        state_view.scheduled_queries.insert(arn, sq_view);
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
        for sq in view.scheduled_queries.values() {
            let tags = view.resource_tags.get(&sq.arn).cloned().unwrap_or_default();

            let attrs = serde_json::json!({
                "id": sq.arn,
                "arn": sq.arn,
                "name": sq.name,
                "query_string": sq.query_string,
                "state": sq.state,
                "schedule_expression": sq.schedule_expression,
                "target_database": sq.target_database,
                "target_table": sq.target_table,
                "s3_error_report_bucket": sq.s3_error_report_bucket,
                "creation_time": sq.creation_time,
                "last_run_status": sq.last_run_status,
                "notification_topic_arn": sq.notification_topic_arn,
                "role_arn": sq.role_arn,
                "last_run_summary": sq.last_run_summary,
                "next_invocation_time": serde_json::Value::Null,
                "tags": tags,
                "tags_all": tags,
                "error_report_configuration": sq.error_report_configuration,
                "target_configuration": sq.target_configuration,
                "notification_configuration": sq.notification_configuration,
                "schedule_configuration": sq.schedule_configuration,
                "recently_failed_runs": sq.recently_failed_runs,
            });
            results.push(ExtractedResource {
                name: sq.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
