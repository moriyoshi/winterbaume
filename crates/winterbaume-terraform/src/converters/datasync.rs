//! Terraform converters for DataSync resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_datasync::DataSyncService;
use winterbaume_datasync::views::{DataSyncLocationView, DataSyncStateView, DataSyncTaskView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_datasync_task
// ---------------------------------------------------------------------------

/// Converts `aws_datasync_task` Terraform resources to/from DataSync state.
pub struct AwsDatasyncTaskConverter {
    service: Arc<DataSyncService>,
}

impl AwsDatasyncTaskConverter {
    pub fn new(service: Arc<DataSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDatasyncTaskConverter {
    fn resource_type(&self) -> &str {
        "aws_datasync_task"
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

impl AwsDatasyncTaskConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let task_arn = optional_str(attrs, "arn")
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:datasync:{}:{}:task/task-{}",
                    region,
                    ctx.default_account_id,
                    uuid::Uuid::new_v4().simple()
                )
            });
        let name = optional_str(attrs, "name");
        let status = optional_str(attrs, "status").unwrap_or_else(|| "AVAILABLE".to_string());
        let source_location_arn = require_str(attrs, "source_location_arn", "aws_datasync_task")?;
        let destination_location_arn =
            require_str(attrs, "destination_location_arn", "aws_datasync_task")?;
        let cloud_watch_log_group_arn = optional_str(attrs, "cloudwatch_log_group_arn");
        let _tags_all = attrs.get("tags_all");
        let _options = attrs.get("options");
        let excludes = attrs.get("excludes").cloned();
        let includes = attrs.get("includes").cloned();
        let schedule = attrs.get("schedule").cloned();
        let task_report_config = attrs.get("task_report_config").cloned();
        let creation_time = chrono::Utc::now();

        let task_view = DataSyncTaskView {
            task_arn: task_arn.clone(),
            name,
            status,
            source_location_arn: source_location_arn.to_string(),
            destination_location_arn: destination_location_arn.to_string(),
            cloud_watch_log_group_arn,
            creation_time,
            excludes,
            includes,
            schedule,
            task_report_config,
        };

        let mut state_view = DataSyncStateView {
            tasks: HashMap::new(),
            locations: HashMap::new(),
            task_executions: HashMap::new(),
        };
        state_view.tasks.insert(task_arn, task_view);
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
        for task in view.tasks.values() {
            let attrs = serde_json::json!({
                "id": task.task_arn,
                "arn": task.task_arn,
                "name": task.name,
                "status": task.status,
                "source_location_arn": task.source_location_arn,
                "destination_location_arn": task.destination_location_arn,
                "cloudwatch_log_group_arn": task.cloud_watch_log_group_arn,
                "creation_time": task.creation_time.to_rfc3339(),
                "tags_all": {},
                "options": [],
                "excludes": task.excludes,
                "includes": task.includes,
                "schedule": task.schedule,
                "task_report_config": task.task_report_config,
            });
            results.push(ExtractedResource {
                name: task.task_arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_datasync_location_s3
// ---------------------------------------------------------------------------

/// Converts `aws_datasync_location_s3` Terraform resources to/from DataSync state.
pub struct AwsDatasyncLocationS3Converter {
    service: Arc<DataSyncService>,
}

impl AwsDatasyncLocationS3Converter {
    pub fn new(service: Arc<DataSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDatasyncLocationS3Converter {
    fn resource_type(&self) -> &str {
        "aws_datasync_location_s3"
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

impl AwsDatasyncLocationS3Converter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let _tags_all = attrs.get("tags_all");
        let _agent_arns = attrs.get("agent_arns");
        let s3_config = attrs.get("s3_config").cloned();
        let location_arn = optional_str(attrs, "arn")
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| {
                format!(
                    "arn:aws:datasync:{}:{}:location/loc-{}",
                    region,
                    ctx.default_account_id,
                    uuid::Uuid::new_v4().simple()
                )
            });
        let s3_bucket_arn = optional_str(attrs, "s3_bucket_arn").unwrap_or_default();
        let subdirectory = optional_str(attrs, "subdirectory").unwrap_or_else(|| "/".to_string());
        let location_uri = optional_str(attrs, "uri").unwrap_or_else(|| {
            // Derive a URI from the bucket ARN if possible.
            let bucket = s3_bucket_arn
                .strip_prefix("arn:aws:s3:::")
                .unwrap_or("bucket");
            format!("s3://{}{}", bucket, subdirectory)
        });
        let creation_time = chrono::Utc::now();

        let location_view = DataSyncLocationView {
            location_arn: location_arn.clone(),
            location_uri,
            creation_time,
            s3_config,
        };

        let mut state_view = DataSyncStateView {
            tasks: HashMap::new(),
            locations: HashMap::new(),
            task_executions: HashMap::new(),
        };
        state_view.locations.insert(location_arn, location_view);
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
        for loc in view.locations.values() {
            let attrs = serde_json::json!({
                "id": loc.location_arn,
                "arn": loc.location_arn,
                "uri": loc.location_uri,
                "creation_time": loc.creation_time.to_rfc3339(),
                "s3_config": loc.s3_config,
                "agent_arns": [],
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: loc.location_arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
