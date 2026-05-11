//! Terraform converters for DataSync resources.
//!
//! `DataSyncTaskTfModel` and `DataSyncLocationS3TfModel` are generated
//! from `specs/datasync.toml`. The synthesised task/location ARNs
//! (`task-<uuid>` / `loc-<uuid>`), the URI fallback derived from the
//! S3 bucket ARN, the constant `status` defaults, the
//! `creation_time` (set to `Utc::now()`), and the raw-Value
//! `excludes` / `includes` / `schedule` / `task_report_config` /
//! `s3_config` reads are wired up here.

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
use crate::generated::datasync as datasync_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: datasync_gen::DataSyncTaskTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_datasync_task", e))?;

        let attrs = &instance.attributes;

        let task_arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:datasync:{}:{}:task/task-{}",
                region,
                ctx.default_account_id,
                uuid::Uuid::new_v4().simple()
            )
        });
        let status = model.status.unwrap_or_else(|| "AVAILABLE".to_string());
        let excludes = attrs.get("excludes").cloned();
        let includes = attrs.get("includes").cloned();
        let schedule = attrs.get("schedule").cloned();
        let task_report_config = attrs.get("task_report_config").cloned();
        let creation_time = chrono::Utc::now();

        let task_view = DataSyncTaskView {
            task_arn: task_arn.clone(),
            name: model.name,
            status,
            source_location_arn: model.source_location_arn,
            destination_location_arn: model.destination_location_arn,
            cloud_watch_log_group_arn: model.cloudwatch_log_group_arn,
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: datasync_gen::DataSyncLocationS3TfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_datasync_location_s3", e))?;

        let attrs = &instance.attributes;

        let s3_config = attrs.get("s3_config").cloned();
        let location_arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:datasync:{}:{}:location/loc-{}",
                region,
                ctx.default_account_id,
                uuid::Uuid::new_v4().simple()
            )
        });
        let s3_bucket_arn = model.s3_bucket_arn.unwrap_or_default();
        let subdirectory = model.subdirectory.unwrap_or_else(|| "/".to_string());
        let location_uri = model.uri.unwrap_or_else(|| {
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

// ---------------------------------------------------------------------------
// Shared helpers for the additional location converters
//
// All `aws_datasync_location_*` resource types map to the generic
// `DataSyncLocationView` slot (arn + uri + creation_time). The S3 extractor
// already enumerates `view.locations.values()` exhaustively, so each
// per-protocol extractor below returns an empty list to avoid duplicate
// output. Inject derives a protocol-prefixed `location_uri` when the TF
// state does not carry one explicitly.
// ---------------------------------------------------------------------------

fn datasync_synth_location_arn(ctx: &ConversionContext, region: &str) -> String {
    format!(
        "arn:aws:datasync:{}:{}:location/loc-{}",
        region,
        ctx.default_account_id,
        uuid::Uuid::new_v4().simple()
    )
}

fn datasync_normalised_subdir(value: Option<&str>) -> String {
    match value {
        Some(s) if !s.is_empty() => {
            if s.starts_with('/') {
                s.to_string()
            } else {
                format!("/{}", s)
            }
        }
        _ => "/".to_string(),
    }
}

async fn datasync_merge_location(
    service: &DataSyncService,
    ctx: &ConversionContext,
    region: &str,
    location_arn: String,
    location_uri: String,
) -> Result<(), ConversionError> {
    let location_view = DataSyncLocationView {
        location_arn: location_arn.clone(),
        location_uri,
        creation_time: chrono::Utc::now(),
        s3_config: None,
    };
    let mut state_view = DataSyncStateView {
        tasks: HashMap::new(),
        locations: HashMap::new(),
        task_executions: HashMap::new(),
    };
    state_view.locations.insert(location_arn, location_view);
    service
        .merge(&ctx.default_account_id, region, state_view)
        .await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// aws_datasync_agent
//
// `winterbaume_datasync` does not track DataSync agents as a distinct
// resource. Inject validates the TF attributes via the generated TfModel
// (so malformed input still fails fast), emits a warning, and otherwise
// no-ops. Extract returns an empty list.
// ---------------------------------------------------------------------------

/// Converts `aws_datasync_agent` Terraform resources (validation-only; no state slot).
pub struct AwsDatasyncAgentConverter {
    #[allow(dead_code)]
    service: Arc<DataSyncService>,
}

impl AwsDatasyncAgentConverter {
    pub fn new(service: Arc<DataSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDatasyncAgentConverter {
    fn resource_type(&self) -> &str {
        "aws_datasync_agent"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDatasyncAgentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: datasync_gen::DataSyncAgentTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_datasync_agent", e))?;
        let warn_msg =
            "no state slot in winterbaume_datasync for agents; inject is a no-op".to_string();
        eprintln!("warning: aws_datasync_agent: {}", warn_msg);
        Ok(ConversionResult {
            region,
            warnings: vec![format!("aws_datasync_agent: {}", warn_msg)],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_datasync_location_azure_blob
// ---------------------------------------------------------------------------

/// Converts `aws_datasync_location_azure_blob` Terraform resources.
pub struct AwsDatasyncLocationAzureBlobConverter {
    service: Arc<DataSyncService>,
}

impl AwsDatasyncLocationAzureBlobConverter {
    pub fn new(service: Arc<DataSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDatasyncLocationAzureBlobConverter {
    fn resource_type(&self) -> &str {
        "aws_datasync_location_azure_blob"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDatasyncLocationAzureBlobConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: datasync_gen::DataSyncLocationAzureBlobTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_datasync_location_azure_blob", e))?;

        let location_arn = model
            .arn
            .or(model.id)
            .unwrap_or_else(|| datasync_synth_location_arn(ctx, &region));
        let subdirectory = datasync_normalised_subdir(model.subdirectory.as_deref());
        let location_uri = model.uri.unwrap_or_else(|| {
            let host = model
                .container_url
                .as_deref()
                .and_then(|u| {
                    u.strip_prefix("https://")
                        .or_else(|| u.strip_prefix("http://"))
                })
                .unwrap_or("container");
            format!("azure-blob://{}{}", host, subdirectory)
        });
        datasync_merge_location(&self.service, ctx, &region, location_arn, location_uri).await?;
        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_datasync_location_efs
// ---------------------------------------------------------------------------

/// Converts `aws_datasync_location_efs` Terraform resources.
pub struct AwsDatasyncLocationEfsConverter {
    service: Arc<DataSyncService>,
}

impl AwsDatasyncLocationEfsConverter {
    pub fn new(service: Arc<DataSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDatasyncLocationEfsConverter {
    fn resource_type(&self) -> &str {
        "aws_datasync_location_efs"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDatasyncLocationEfsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: datasync_gen::DataSyncLocationEfsTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_datasync_location_efs", e))?;

        let location_arn = model
            .arn
            .or(model.id)
            .unwrap_or_else(|| datasync_synth_location_arn(ctx, &region));
        let subdirectory = datasync_normalised_subdir(model.subdirectory.as_deref());
        let location_uri = model.uri.unwrap_or_else(|| {
            let fs_id = model
                .efs_file_system_arn
                .as_deref()
                .and_then(|arn| arn.split('/').next_back())
                .unwrap_or("fs");
            format!(
                "efs://{}.efs.{}.amazonaws.com{}",
                fs_id, region, subdirectory
            )
        });
        datasync_merge_location(&self.service, ctx, &region, location_arn, location_uri).await?;
        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_datasync_location_fsx_lustre_file_system
// ---------------------------------------------------------------------------

/// Converts `aws_datasync_location_fsx_lustre_file_system` Terraform resources.
pub struct AwsDatasyncLocationFsxLustreFileSystemConverter {
    service: Arc<DataSyncService>,
}

impl AwsDatasyncLocationFsxLustreFileSystemConverter {
    pub fn new(service: Arc<DataSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDatasyncLocationFsxLustreFileSystemConverter {
    fn resource_type(&self) -> &str {
        "aws_datasync_location_fsx_lustre_file_system"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDatasyncLocationFsxLustreFileSystemConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: datasync_gen::DataSyncLocationFsxLustreFileSystemTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_datasync_location_fsx_lustre_file_system", e)
            })?;

        let location_arn = model
            .arn
            .or(model.id)
            .unwrap_or_else(|| datasync_synth_location_arn(ctx, &region));
        let subdirectory = datasync_normalised_subdir(model.subdirectory.as_deref());
        let location_uri = model.uri.unwrap_or_else(|| {
            let host = model
                .fsx_filesystem_arn
                .as_deref()
                .and_then(|arn| arn.split('/').next_back())
                .unwrap_or("fs");
            format!(
                "fsxl://{}.fsx.{}.amazonaws.com{}",
                host, region, subdirectory
            )
        });
        datasync_merge_location(&self.service, ctx, &region, location_arn, location_uri).await?;
        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_datasync_location_fsx_ontap_file_system
// ---------------------------------------------------------------------------

/// Converts `aws_datasync_location_fsx_ontap_file_system` Terraform resources.
pub struct AwsDatasyncLocationFsxOntapFileSystemConverter {
    service: Arc<DataSyncService>,
}

impl AwsDatasyncLocationFsxOntapFileSystemConverter {
    pub fn new(service: Arc<DataSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDatasyncLocationFsxOntapFileSystemConverter {
    fn resource_type(&self) -> &str {
        "aws_datasync_location_fsx_ontap_file_system"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDatasyncLocationFsxOntapFileSystemConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: datasync_gen::DataSyncLocationFsxOntapFileSystemTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_datasync_location_fsx_ontap_file_system", e)
            })?;

        let location_arn = model
            .arn
            .or(model.id)
            .unwrap_or_else(|| datasync_synth_location_arn(ctx, &region));
        let subdirectory = datasync_normalised_subdir(model.subdirectory.as_deref());
        let location_uri = model.uri.unwrap_or_else(|| {
            let host = model
                .storage_virtual_machine_arn
                .as_deref()
                .and_then(|arn| arn.split('/').next_back())
                .unwrap_or("svm");
            format!(
                "fsxn://{}.fsx.{}.amazonaws.com{}",
                host, region, subdirectory
            )
        });
        datasync_merge_location(&self.service, ctx, &region, location_arn, location_uri).await?;
        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_datasync_location_fsx_openzfs_file_system
// ---------------------------------------------------------------------------

/// Converts `aws_datasync_location_fsx_openzfs_file_system` Terraform resources.
pub struct AwsDatasyncLocationFsxOpenZfsFileSystemConverter {
    service: Arc<DataSyncService>,
}

impl AwsDatasyncLocationFsxOpenZfsFileSystemConverter {
    pub fn new(service: Arc<DataSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDatasyncLocationFsxOpenZfsFileSystemConverter {
    fn resource_type(&self) -> &str {
        "aws_datasync_location_fsx_openzfs_file_system"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDatasyncLocationFsxOpenZfsFileSystemConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: datasync_gen::DataSyncLocationFsxOpenZfsFileSystemTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_datasync_location_fsx_openzfs_file_system", e)
            })?;

        let location_arn = model
            .arn
            .or(model.id)
            .unwrap_or_else(|| datasync_synth_location_arn(ctx, &region));
        let subdirectory = datasync_normalised_subdir(model.subdirectory.as_deref());
        let location_uri = model.uri.unwrap_or_else(|| {
            let host = model
                .fsx_filesystem_arn
                .as_deref()
                .and_then(|arn| arn.split('/').next_back())
                .unwrap_or("fs");
            format!(
                "fsxz://{}.fsx.{}.amazonaws.com{}",
                host, region, subdirectory
            )
        });
        datasync_merge_location(&self.service, ctx, &region, location_arn, location_uri).await?;
        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_datasync_location_fsx_windows_file_system
// ---------------------------------------------------------------------------

/// Converts `aws_datasync_location_fsx_windows_file_system` Terraform resources.
pub struct AwsDatasyncLocationFsxWindowsFileSystemConverter {
    service: Arc<DataSyncService>,
}

impl AwsDatasyncLocationFsxWindowsFileSystemConverter {
    pub fn new(service: Arc<DataSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDatasyncLocationFsxWindowsFileSystemConverter {
    fn resource_type(&self) -> &str {
        "aws_datasync_location_fsx_windows_file_system"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDatasyncLocationFsxWindowsFileSystemConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: datasync_gen::DataSyncLocationFsxWindowsFileSystemTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_datasync_location_fsx_windows_file_system", e)
            })?;

        let location_arn = model
            .arn
            .or(model.id)
            .unwrap_or_else(|| datasync_synth_location_arn(ctx, &region));
        let subdirectory = datasync_normalised_subdir(model.subdirectory.as_deref());
        let location_uri = model.uri.unwrap_or_else(|| {
            let host = model
                .fsx_filesystem_arn
                .as_deref()
                .and_then(|arn| arn.split('/').next_back())
                .unwrap_or("fs");
            format!(
                "fsxw://{}.fsx.{}.amazonaws.com{}",
                host, region, subdirectory
            )
        });
        datasync_merge_location(&self.service, ctx, &region, location_arn, location_uri).await?;
        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_datasync_location_hdfs
// ---------------------------------------------------------------------------

/// Converts `aws_datasync_location_hdfs` Terraform resources.
pub struct AwsDatasyncLocationHdfsConverter {
    service: Arc<DataSyncService>,
}

impl AwsDatasyncLocationHdfsConverter {
    pub fn new(service: Arc<DataSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDatasyncLocationHdfsConverter {
    fn resource_type(&self) -> &str {
        "aws_datasync_location_hdfs"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDatasyncLocationHdfsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: datasync_gen::DataSyncLocationHdfsTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_datasync_location_hdfs", e))?;

        let location_arn = model
            .arn
            .or(model.id)
            .unwrap_or_else(|| datasync_synth_location_arn(ctx, &region));
        let subdirectory = datasync_normalised_subdir(model.subdirectory.as_deref());
        let location_uri = model
            .uri
            .unwrap_or_else(|| format!("hdfs://namenode{}", subdirectory));
        datasync_merge_location(&self.service, ctx, &region, location_arn, location_uri).await?;
        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_datasync_location_nfs
// ---------------------------------------------------------------------------

/// Converts `aws_datasync_location_nfs` Terraform resources.
pub struct AwsDatasyncLocationNfsConverter {
    service: Arc<DataSyncService>,
}

impl AwsDatasyncLocationNfsConverter {
    pub fn new(service: Arc<DataSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDatasyncLocationNfsConverter {
    fn resource_type(&self) -> &str {
        "aws_datasync_location_nfs"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDatasyncLocationNfsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: datasync_gen::DataSyncLocationNfsTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_datasync_location_nfs", e))?;

        let location_arn = model
            .arn
            .or(model.id)
            .unwrap_or_else(|| datasync_synth_location_arn(ctx, &region));
        let subdirectory = datasync_normalised_subdir(model.subdirectory.as_deref());
        let host = model
            .server_hostname
            .clone()
            .unwrap_or_else(|| "nfs.example.com".to_string());
        let location_uri = model
            .uri
            .unwrap_or_else(|| format!("nfs://{}{}", host, subdirectory));
        datasync_merge_location(&self.service, ctx, &region, location_arn, location_uri).await?;
        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_datasync_location_object_storage
// ---------------------------------------------------------------------------

/// Converts `aws_datasync_location_object_storage` Terraform resources.
pub struct AwsDatasyncLocationObjectStorageConverter {
    service: Arc<DataSyncService>,
}

impl AwsDatasyncLocationObjectStorageConverter {
    pub fn new(service: Arc<DataSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDatasyncLocationObjectStorageConverter {
    fn resource_type(&self) -> &str {
        "aws_datasync_location_object_storage"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDatasyncLocationObjectStorageConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: datasync_gen::DataSyncLocationObjectStorageTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_datasync_location_object_storage", e)
            })?;

        let location_arn = model
            .arn
            .or(model.id)
            .unwrap_or_else(|| datasync_synth_location_arn(ctx, &region));
        let subdirectory = datasync_normalised_subdir(model.subdirectory.as_deref());
        let host = model
            .server_hostname
            .clone()
            .unwrap_or_else(|| "object.example.com".to_string());
        let bucket = model
            .bucket_name
            .clone()
            .unwrap_or_else(|| "bucket".to_string());
        let location_uri = model
            .uri
            .unwrap_or_else(|| format!("object-storage://{}/{}{}", host, bucket, subdirectory));
        datasync_merge_location(&self.service, ctx, &region, location_arn, location_uri).await?;
        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_datasync_location_smb
// ---------------------------------------------------------------------------

/// Converts `aws_datasync_location_smb` Terraform resources.
pub struct AwsDatasyncLocationSmbConverter {
    service: Arc<DataSyncService>,
}

impl AwsDatasyncLocationSmbConverter {
    pub fn new(service: Arc<DataSyncService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDatasyncLocationSmbConverter {
    fn resource_type(&self) -> &str {
        "aws_datasync_location_smb"
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDatasyncLocationSmbConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: datasync_gen::DataSyncLocationSmbTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_datasync_location_smb", e))?;

        let location_arn = model
            .arn
            .or(model.id)
            .unwrap_or_else(|| datasync_synth_location_arn(ctx, &region));
        let subdirectory = datasync_normalised_subdir(model.subdirectory.as_deref());
        let host = model
            .server_hostname
            .clone()
            .unwrap_or_else(|| "smb.example.com".to_string());
        let location_uri = model
            .uri
            .unwrap_or_else(|| format!("smb://{}{}", host, subdirectory));
        datasync_merge_location(&self.service, ctx, &region, location_arn, location_uri).await?;
        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}
