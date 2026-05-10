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
use crate::generated::timestreamquery as timestreamquery_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: timestreamquery_gen::ScheduledQueryTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_timestreamquery_scheduled_query", e)
            })?;

        let arn = model.arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:timestream:{}:{}:scheduled-query/{}",
                region, ctx.default_account_id, model.name
            )
        });

        // JSON-blob nested-block fields stay raw.
        let attrs = &instance.attributes;
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

        let sq_view = ScheduledQueryView {
            arn: arn.clone(),
            name: model.name.clone(),
            query_string: model.query_string.unwrap_or_default(),
            state: model.state.unwrap_or_else(|| "ENABLED".to_string()),
            schedule_expression: model.schedule_expression.unwrap_or_default(),
            target_database: model.target_database.unwrap_or_default(),
            target_table: model.target_table.unwrap_or_default(),
            s3_error_report_bucket: model.s3_error_report_bucket,
            creation_time: model
                .creation_time
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            last_run_status: model.last_run_status,
            notification_topic_arn: model.notification_topic_arn.unwrap_or_default(),
            role_arn: model.role_arn.unwrap_or_default(),
            error_report_configuration,
            target_configuration,
            notification_configuration,
            schedule_configuration,
            recently_failed_runs: vec![],
            last_run_summary: None,
        };

        // Resource tags keyed by ARN.
        let mut resource_tags: HashMap<String, HashMap<String, String>> = HashMap::new();
        if !model.tags.is_empty() {
            resource_tags.insert(arn.clone(), model.tags);
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
