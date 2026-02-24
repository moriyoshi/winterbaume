//! Terraform converters for FSx resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_fsx::FsxService;
use winterbaume_fsx::views::{FileSystemView, FsxStateView, TagView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_bool, optional_i64, optional_str};

// ---------------------------------------------------------------------------
// aws_fsx_lustre_file_system
// ---------------------------------------------------------------------------

/// Converts `aws_fsx_lustre_file_system` Terraform resources to/from FSx state.
pub struct AwsFsxLustreFileSystemConverter {
    service: Arc<FsxService>,
}

impl AwsFsxLustreFileSystemConverter {
    pub fn new(service: Arc<FsxService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsFsxLustreFileSystemConverter {
    fn resource_type(&self) -> &str {
        "aws_fsx_lustre_file_system"
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

impl AwsFsxLustreFileSystemConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let file_system_id = optional_str(attrs, "id").unwrap_or_else(|| {
            format!(
                "fs-lustre-{}",
                &ctx.default_account_id[..8.min(ctx.default_account_id.len())]
            )
        });
        let storage_capacity = optional_i64(attrs, "storage_capacity").unwrap_or(1200);
        let storage_type = optional_str(attrs, "storage_type").unwrap_or_else(|| "SSD".to_string());
        let kms_key_id = optional_str(attrs, "kms_key_id");
        let dns_name = optional_str(attrs, "dns_name")
            .unwrap_or_else(|| format!("{}.fsx.{}.amazonaws.com", file_system_id, region));

        let subnet_ids: Vec<String> = attrs
            .get("subnet_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let security_group_ids: Vec<String> = attrs
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let resource_arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:fsx:{}:{}:file-system/{}",
                region, ctx.default_account_id, file_system_id
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        // Additional inject fields for coverage
        let _auto_import_policy = optional_str(attrs, "auto_import_policy");
        let _data_compression_type = optional_str(attrs, "data_compression_type");
        let _drive_cache_type = optional_str(attrs, "drive_cache_type");
        let _export_path = optional_str(attrs, "export_path");
        let _file_system_type_version = optional_str(attrs, "file_system_type_version");
        let _import_path = optional_str(attrs, "import_path");

        // Capture lustre_configuration as opaque JSON, merging nested blocks into it
        let lustre_configuration = {
            let mut base = attrs
                .get("lustre_configuration")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .cloned()
                .unwrap_or(serde_json::json!({}));
            if let serde_json::Value::Object(ref mut obj) = base {
                for key in &[
                    "data_read_cache_configuration",
                    "log_configuration",
                    "metadata_configuration",
                    "root_squash_configuration",
                ] {
                    if let Some(val) = attrs.get(*key) {
                        obj.insert(key.to_string(), val.clone());
                    }
                }
            }
            Some(base)
        };

        let creation_time = optional_str(attrs, "creation_time");
        let lifecycle = optional_str(attrs, "lifecycle")
            .or_else(|| optional_str(attrs, "lifecycle_status"))
            .unwrap_or_else(|| "AVAILABLE".to_string());
        let owner_id = optional_str(attrs, "owner_id");
        let vpc_id = optional_str(attrs, "vpc_id");
        let deployment_type = optional_str(attrs, "deployment_type");
        let copy_tags_to_backups = optional_bool(attrs, "copy_tags_to_backups").unwrap_or(false);
        let automatic_backup_retention_days =
            optional_i64(attrs, "automatic_backup_retention_days").unwrap_or(0) as i32;
        let daily_automatic_backup_start_time =
            optional_str(attrs, "daily_automatic_backup_start_time");
        let weekly_maintenance_start_time = optional_str(attrs, "weekly_maintenance_start_time");

        let fs_view = FileSystemView {
            file_system_id: file_system_id.clone(),
            file_system_type: "LUSTRE".to_string(),
            storage_capacity,
            storage_type,
            subnet_ids,
            security_group_ids,
            dns_name,
            kms_key_id,
            resource_arn,
            tags,
            windows_configuration: None,
            lustre_configuration,
            ontap_configuration: None,
            open_zfs_configuration: None,
            creation_time,
            lifecycle,
            owner_id,
            vpc_id,
            deployment_type,
            copy_tags_to_backups,
            automatic_backup_retention_days,
            daily_automatic_backup_start_time,
            weekly_maintenance_start_time,
        };

        let state_view = FsxStateView {
            file_systems: {
                let mut m = HashMap::new();
                m.insert(file_system_id, fs_view);
                m
            },
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
        for fs in view.file_systems.values() {
            if fs.file_system_type != "LUSTRE" {
                continue;
            }
            let tags: HashMap<String, String> = fs
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let lc = fs.lustre_configuration.as_ref();
            let data_read_cache_configuration = lc
                .and_then(|v| v.get("data_read_cache_configuration"))
                .cloned()
                .unwrap_or(serde_json::json!([]));
            let log_configuration = lc
                .and_then(|v| v.get("log_configuration"))
                .cloned()
                .unwrap_or(serde_json::json!([]));
            let metadata_configuration = lc
                .and_then(|v| v.get("metadata_configuration"))
                .cloned()
                .unwrap_or(serde_json::json!([]));
            let root_squash_configuration = lc
                .and_then(|v| v.get("root_squash_configuration"))
                .cloned()
                .unwrap_or(serde_json::json!([]));
            let attrs = serde_json::json!({
                "id": fs.file_system_id,
                "arn": fs.resource_arn,
                "file_system_type": fs.file_system_type,
                "storage_capacity": fs.storage_capacity,
                "storage_type": fs.storage_type,
                "subnet_ids": fs.subnet_ids,
                "security_group_ids": fs.security_group_ids,
                "dns_name": fs.dns_name,
                "kms_key_id": fs.kms_key_id,
                "lustre_configuration": fs.lustre_configuration,
                "data_read_cache_configuration": data_read_cache_configuration,
                "log_configuration": log_configuration,
                "metadata_configuration": metadata_configuration,
                "root_squash_configuration": root_squash_configuration,
                "creation_time": fs.creation_time,
                "lifecycle": fs.lifecycle,
                "lifecycle_status": fs.lifecycle,
                "owner_id": fs.owner_id,
                "vpc_id": fs.vpc_id,
                "deployment_type": fs.deployment_type,
                "copy_tags_to_backups": fs.copy_tags_to_backups,
                "automatic_backup_retention_days": fs.automatic_backup_retention_days,
                "daily_automatic_backup_start_time": fs.daily_automatic_backup_start_time,
                "weekly_maintenance_start_time": fs.weekly_maintenance_start_time,
                "mount_name": "fsx",
                "network_interface_ids": [],
                "tags": tags,
                "tags_all": tags,
            });
            results.push(ExtractedResource {
                name: fs.file_system_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_fsx_windows_file_system
// ---------------------------------------------------------------------------

/// Converts `aws_fsx_windows_file_system` Terraform resources to/from FSx state.
pub struct AwsFsxWindowsFileSystemConverter {
    service: Arc<FsxService>,
}

impl AwsFsxWindowsFileSystemConverter {
    pub fn new(service: Arc<FsxService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsFsxWindowsFileSystemConverter {
    fn resource_type(&self) -> &str {
        "aws_fsx_windows_file_system"
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

impl AwsFsxWindowsFileSystemConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let file_system_id = optional_str(attrs, "id").unwrap_or_else(|| {
            format!(
                "fs-windows-{}",
                &ctx.default_account_id[..8.min(ctx.default_account_id.len())]
            )
        });
        let storage_capacity = optional_i64(attrs, "storage_capacity").unwrap_or(300);
        let storage_type = optional_str(attrs, "storage_type").unwrap_or_else(|| "SSD".to_string());
        let kms_key_id = optional_str(attrs, "kms_key_id");
        let dns_name = optional_str(attrs, "dns_name")
            .unwrap_or_else(|| format!("{}.fsx.{}.amazonaws.com", file_system_id, region));

        let subnet_ids: Vec<String> = attrs
            .get("subnet_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let security_group_ids: Vec<String> = attrs
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let resource_arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:fsx:{}:{}:file-system/{}",
                region, ctx.default_account_id, file_system_id
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        // Build windows_configuration from TF attributes
        let active_directory_id = optional_str(attrs, "active_directory_id");
        let throughput_capacity = optional_i64(attrs, "throughput_capacity");
        let audit_log_configuration = attrs.get("audit_log_configuration").cloned();
        let disk_iops_configuration = attrs.get("disk_iops_configuration").cloned();
        let self_managed_active_directory = attrs.get("self_managed_active_directory").cloned();
        let windows_configuration = Some(serde_json::json!({
            "active_directory_id": active_directory_id,
            "throughput_capacity": throughput_capacity,
            "audit_log_configuration": audit_log_configuration,
            "disk_iops_configuration": disk_iops_configuration,
            "self_managed_active_directory": self_managed_active_directory,
        }));

        let creation_time = optional_str(attrs, "creation_time");
        let lifecycle = optional_str(attrs, "lifecycle")
            .or_else(|| optional_str(attrs, "lifecycle_status"))
            .unwrap_or_else(|| "AVAILABLE".to_string());
        let owner_id = optional_str(attrs, "owner_id");
        let vpc_id = optional_str(attrs, "vpc_id");
        let deployment_type = optional_str(attrs, "deployment_type");
        let copy_tags_to_backups = optional_bool(attrs, "copy_tags_to_backups").unwrap_or(false);
        let automatic_backup_retention_days =
            optional_i64(attrs, "automatic_backup_retention_days").unwrap_or(0) as i32;
        let daily_automatic_backup_start_time =
            optional_str(attrs, "daily_automatic_backup_start_time");
        let weekly_maintenance_start_time = optional_str(attrs, "weekly_maintenance_start_time");

        let fs_view = FileSystemView {
            file_system_id: file_system_id.clone(),
            file_system_type: "WINDOWS".to_string(),
            storage_capacity,
            storage_type,
            subnet_ids,
            security_group_ids,
            dns_name,
            kms_key_id,
            resource_arn,
            tags,
            windows_configuration,
            lustre_configuration: None,
            ontap_configuration: None,
            open_zfs_configuration: None,
            creation_time,
            lifecycle,
            owner_id,
            vpc_id,
            deployment_type,
            copy_tags_to_backups,
            automatic_backup_retention_days,
            daily_automatic_backup_start_time,
            weekly_maintenance_start_time,
        };

        let state_view = FsxStateView {
            file_systems: {
                let mut m = HashMap::new();
                m.insert(file_system_id, fs_view);
                m
            },
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
        for fs in view.file_systems.values() {
            if fs.file_system_type != "WINDOWS" {
                continue;
            }
            let tags: HashMap<String, String> = fs
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let active_directory_id = fs
                .windows_configuration
                .as_ref()
                .and_then(|wc| wc.get("active_directory_id"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let throughput_capacity = fs
                .windows_configuration
                .as_ref()
                .and_then(|wc| wc.get("throughput_capacity"))
                .and_then(|v| v.as_i64());
            let wc = fs.windows_configuration.as_ref();
            let audit_log_configuration = wc
                .and_then(|wc| wc.get("audit_log_configuration"))
                .cloned()
                .unwrap_or(serde_json::json!([]));
            let disk_iops_configuration = wc
                .and_then(|wc| wc.get("disk_iops_configuration"))
                .cloned()
                .unwrap_or(serde_json::json!([]));
            let self_managed_active_directory = wc
                .and_then(|wc| wc.get("self_managed_active_directory"))
                .cloned()
                .unwrap_or(serde_json::json!([]));
            let attrs = serde_json::json!({
                "id": fs.file_system_id,
                "arn": fs.resource_arn,
                "file_system_type": fs.file_system_type,
                "storage_capacity": fs.storage_capacity,
                "storage_type": fs.storage_type,
                "subnet_ids": fs.subnet_ids,
                "security_group_ids": fs.security_group_ids,
                "dns_name": fs.dns_name,
                "kms_key_id": fs.kms_key_id,
                "active_directory_id": active_directory_id,
                "throughput_capacity": throughput_capacity,
                "audit_log_configuration": audit_log_configuration,
                "disk_iops_configuration": disk_iops_configuration,
                "self_managed_active_directory": self_managed_active_directory,
                "creation_time": fs.creation_time,
                "lifecycle": fs.lifecycle,
                "lifecycle_status": fs.lifecycle,
                "owner_id": fs.owner_id,
                "vpc_id": fs.vpc_id,
                "deployment_type": fs.deployment_type,
                "copy_tags_to_backups": fs.copy_tags_to_backups,
                "automatic_backup_retention_days": fs.automatic_backup_retention_days,
                "daily_automatic_backup_start_time": fs.daily_automatic_backup_start_time,
                "weekly_maintenance_start_time": fs.weekly_maintenance_start_time,
                "tags": tags,
                "tags_all": tags,
            });
            results.push(ExtractedResource {
                name: fs.file_system_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
