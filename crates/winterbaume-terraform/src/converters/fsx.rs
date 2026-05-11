//! Terraform converters for FSx resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_fsx::FsxService;
use winterbaume_fsx::views::{BackupView, FileSystemView, FsxStateView, TagView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::fsx as fsx_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: fsx_gen::LustreFileSystemTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_fsx_lustre_file_system", e))?;

        let attrs = &instance.attributes;
        let file_system_id = model.id.unwrap_or_else(|| {
            format!(
                "fs-lustre-{}",
                &ctx.default_account_id[..8.min(ctx.default_account_id.len())]
            )
        });
        let storage_capacity = model.storage_capacity;
        let storage_type = model.storage_type.unwrap_or_else(|| "SSD".to_string());
        let kms_key_id = model.kms_key_id;
        let dns_name = model
            .dns_name
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

        let resource_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:fsx:{}:{}:file-system/{}",
                region, ctx.default_account_id, file_system_id
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

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

        let lifecycle = model
            .lifecycle
            .or(model.lifecycle_status)
            .unwrap_or_else(|| "AVAILABLE".to_string());

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
            creation_time: model.creation_time,
            lifecycle,
            owner_id: model.owner_id,
            vpc_id: model.vpc_id,
            deployment_type: model.deployment_type,
            copy_tags_to_backups: model.copy_tags_to_backups,
            automatic_backup_retention_days: model.automatic_backup_retention_days as i32,
            daily_automatic_backup_start_time: model.daily_automatic_backup_start_time,
            weekly_maintenance_start_time: model.weekly_maintenance_start_time,
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: fsx_gen::WindowsFileSystemTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_fsx_windows_file_system", e))?;

        let attrs = &instance.attributes;
        let file_system_id = model.id.unwrap_or_else(|| {
            format!(
                "fs-windows-{}",
                &ctx.default_account_id[..8.min(ctx.default_account_id.len())]
            )
        });
        let storage_capacity = model.storage_capacity;
        let storage_type = model.storage_type.unwrap_or_else(|| "SSD".to_string());
        let kms_key_id = model.kms_key_id;
        let dns_name = model
            .dns_name
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

        let resource_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:fsx:{}:{}:file-system/{}",
                region, ctx.default_account_id, file_system_id
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        // Build windows_configuration from TF attributes (raw blobs not in the model)
        let active_directory_id = attrs
            .get("active_directory_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let throughput_capacity = attrs.get("throughput_capacity").and_then(|v| v.as_i64());
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

        let lifecycle = model
            .lifecycle
            .or(model.lifecycle_status)
            .unwrap_or_else(|| "AVAILABLE".to_string());

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
            creation_time: model.creation_time,
            lifecycle,
            owner_id: model.owner_id,
            vpc_id: model.vpc_id,
            deployment_type: model.deployment_type,
            copy_tags_to_backups: model.copy_tags_to_backups,
            automatic_backup_retention_days: model.automatic_backup_retention_days as i32,
            daily_automatic_backup_start_time: model.daily_automatic_backup_start_time,
            weekly_maintenance_start_time: model.weekly_maintenance_start_time,
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

// ---------------------------------------------------------------------------
// aws_fsx_backup
// ---------------------------------------------------------------------------

/// Converts `aws_fsx_backup` Terraform resources to/from FSx state.
pub struct AwsFsxBackupConverter {
    service: Arc<FsxService>,
}

impl AwsFsxBackupConverter {
    pub fn new(service: Arc<FsxService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsFsxBackupConverter {
    fn resource_type(&self) -> &str {
        "aws_fsx_backup"
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

impl AwsFsxBackupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: fsx_gen::BackupTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_fsx_backup", e))?;

        let attrs = &instance.attributes;
        let backup_id = model.id.unwrap_or_else(|| {
            format!(
                "fsx-backup-{}",
                &ctx.default_account_id[..8.min(ctx.default_account_id.len())]
            )
        });
        let file_system_id = model.file_system_id.unwrap_or_default();
        let resource_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:fsx:{}:{}:backup/{}",
                region, ctx.default_account_id, backup_id
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        let backup_view = BackupView {
            backup_id: backup_id.clone(),
            file_system_id,
            lifecycle: "AVAILABLE".to_string(),
            creation_time: String::new(),
            resource_arn,
            tags,
        };

        let state_view = FsxStateView {
            backups: {
                let mut m = HashMap::new();
                m.insert(backup_id, backup_view);
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
        for backup in view.backups.values() {
            let tags: HashMap<String, String> = backup
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": backup.backup_id,
                "arn": backup.resource_arn,
                "file_system_id": backup.file_system_id,
                "tags": tags,
                "tags_all": tags,
            });
            results.push(ExtractedResource {
                name: backup.backup_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_fsx_ontap_file_system
// ---------------------------------------------------------------------------

/// Converts `aws_fsx_ontap_file_system` Terraform resources to/from FSx state.
pub struct AwsFsxOntapFileSystemConverter {
    service: Arc<FsxService>,
}

impl AwsFsxOntapFileSystemConverter {
    pub fn new(service: Arc<FsxService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsFsxOntapFileSystemConverter {
    fn resource_type(&self) -> &str {
        "aws_fsx_ontap_file_system"
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

impl AwsFsxOntapFileSystemConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: fsx_gen::OntapFileSystemTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_fsx_ontap_file_system", e))?;

        let attrs = &instance.attributes;
        let file_system_id = model.id.unwrap_or_else(|| {
            format!(
                "fs-ontap-{}",
                &ctx.default_account_id[..8.min(ctx.default_account_id.len())]
            )
        });
        let storage_capacity = model.storage_capacity;
        let storage_type = model.storage_type.unwrap_or_else(|| "SSD".to_string());
        let kms_key_id = model.kms_key_id;
        let dns_name = model
            .dns_name
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

        let resource_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:fsx:{}:{}:file-system/{}",
                region, ctx.default_account_id, file_system_id
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        // Capture ontap_configuration as opaque JSON, merging nested-block
        // fields from instance.attributes alongside scalars from the model.
        let ontap_configuration = {
            let mut obj = serde_json::Map::new();
            obj.insert(
                "preferred_subnet_id".to_string(),
                serde_json::Value::from(model.preferred_subnet_id.clone()),
            );
            obj.insert(
                "throughput_capacity".to_string(),
                serde_json::Value::from(model.throughput_capacity),
            );
            obj.insert(
                "ha_pairs".to_string(),
                serde_json::Value::from(model.ha_pairs),
            );
            obj.insert(
                "throughput_capacity_per_ha_pair".to_string(),
                serde_json::Value::from(model.throughput_capacity_per_ha_pair),
            );
            obj.insert(
                "endpoint_ip_address_range".to_string(),
                serde_json::Value::from(model.endpoint_ip_address_range.clone()),
            );
            obj.insert(
                "fsx_admin_password".to_string(),
                serde_json::Value::from(model.fsx_admin_password.clone()),
            );
            for key in &[
                "route_table_ids",
                "disk_iops_configuration",
                "endpoints",
                "ha_pair_subnet_mapping",
            ] {
                if let Some(val) = attrs.get(*key) {
                    obj.insert((*key).to_string(), val.clone());
                }
            }
            Some(serde_json::Value::Object(obj))
        };

        let lifecycle = model
            .lifecycle
            .or(model.lifecycle_status)
            .unwrap_or_else(|| "AVAILABLE".to_string());

        let fs_view = FileSystemView {
            file_system_id: file_system_id.clone(),
            file_system_type: "ONTAP".to_string(),
            storage_capacity,
            storage_type,
            subnet_ids,
            security_group_ids,
            dns_name,
            kms_key_id,
            resource_arn,
            tags,
            windows_configuration: None,
            lustre_configuration: None,
            ontap_configuration,
            open_zfs_configuration: None,
            creation_time: model.creation_time,
            lifecycle,
            owner_id: model.owner_id,
            vpc_id: model.vpc_id,
            deployment_type: model.deployment_type,
            copy_tags_to_backups: false,
            automatic_backup_retention_days: model.automatic_backup_retention_days as i32,
            daily_automatic_backup_start_time: model.daily_automatic_backup_start_time,
            weekly_maintenance_start_time: model.weekly_maintenance_start_time,
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
            if fs.file_system_type != "ONTAP" {
                continue;
            }
            let tags: HashMap<String, String> = fs
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let oc = fs.ontap_configuration.as_ref();
            let preferred_subnet_id = oc
                .and_then(|v| v.get("preferred_subnet_id"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let throughput_capacity = oc
                .and_then(|v| v.get("throughput_capacity"))
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            let ha_pairs = oc
                .and_then(|v| v.get("ha_pairs"))
                .and_then(|v| v.as_i64())
                .unwrap_or(1);
            let throughput_capacity_per_ha_pair = oc
                .and_then(|v| v.get("throughput_capacity_per_ha_pair"))
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            let endpoint_ip_address_range = oc
                .and_then(|v| v.get("endpoint_ip_address_range"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
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
                "preferred_subnet_id": preferred_subnet_id,
                "throughput_capacity": throughput_capacity,
                "ha_pairs": ha_pairs,
                "throughput_capacity_per_ha_pair": throughput_capacity_per_ha_pair,
                "endpoint_ip_address_range": endpoint_ip_address_range,
                "creation_time": fs.creation_time,
                "lifecycle": fs.lifecycle,
                "lifecycle_status": fs.lifecycle,
                "owner_id": fs.owner_id,
                "vpc_id": fs.vpc_id,
                "deployment_type": fs.deployment_type,
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

// ---------------------------------------------------------------------------
// aws_fsx_openzfs_file_system
// ---------------------------------------------------------------------------

/// Converts `aws_fsx_openzfs_file_system` Terraform resources to/from FSx state.
pub struct AwsFsxOpenzfsFileSystemConverter {
    service: Arc<FsxService>,
}

impl AwsFsxOpenzfsFileSystemConverter {
    pub fn new(service: Arc<FsxService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsFsxOpenzfsFileSystemConverter {
    fn resource_type(&self) -> &str {
        "aws_fsx_openzfs_file_system"
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

impl AwsFsxOpenzfsFileSystemConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: fsx_gen::OpenzfsFileSystemTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_fsx_openzfs_file_system", e))?;

        let attrs = &instance.attributes;
        let file_system_id = model.id.unwrap_or_else(|| {
            format!(
                "fs-openzfs-{}",
                &ctx.default_account_id[..8.min(ctx.default_account_id.len())]
            )
        });
        let storage_capacity = model.storage_capacity;
        let storage_type = model.storage_type.unwrap_or_else(|| "SSD".to_string());
        let kms_key_id = model.kms_key_id;
        let dns_name = model
            .dns_name
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

        let resource_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:fsx:{}:{}:file-system/{}",
                region, ctx.default_account_id, file_system_id
            )
        });

        let tags: Vec<TagView> = extract_tags(attrs)
            .into_iter()
            .map(|(k, v)| TagView { key: k, value: v })
            .collect();

        // Capture openzfs_configuration with scalars from the model plus
        // raw nested-block fields from the TF attributes.
        let open_zfs_configuration = {
            let mut obj = serde_json::Map::new();
            obj.insert(
                "throughput_capacity".to_string(),
                serde_json::Value::from(model.throughput_capacity),
            );
            obj.insert(
                "copy_tags_to_volumes".to_string(),
                serde_json::Value::from(model.copy_tags_to_volumes),
            );
            obj.insert(
                "root_volume_id".to_string(),
                serde_json::Value::from(model.root_volume_id.clone()),
            );
            obj.insert(
                "endpoint_ip_address_range".to_string(),
                serde_json::Value::from(model.endpoint_ip_address_range.clone()),
            );
            obj.insert(
                "preferred_subnet_id".to_string(),
                serde_json::Value::from(model.preferred_subnet_id.clone()),
            );
            for key in &[
                "route_table_ids",
                "disk_iops_configuration",
                "root_volume_configuration",
            ] {
                if let Some(val) = attrs.get(*key) {
                    obj.insert((*key).to_string(), val.clone());
                }
            }
            Some(serde_json::Value::Object(obj))
        };

        let lifecycle = model
            .lifecycle
            .or(model.lifecycle_status)
            .unwrap_or_else(|| "AVAILABLE".to_string());

        let fs_view = FileSystemView {
            file_system_id: file_system_id.clone(),
            file_system_type: "OPENZFS".to_string(),
            storage_capacity,
            storage_type,
            subnet_ids,
            security_group_ids,
            dns_name,
            kms_key_id,
            resource_arn,
            tags,
            windows_configuration: None,
            lustre_configuration: None,
            ontap_configuration: None,
            open_zfs_configuration,
            creation_time: model.creation_time,
            lifecycle,
            owner_id: model.owner_id,
            vpc_id: model.vpc_id,
            deployment_type: model.deployment_type,
            copy_tags_to_backups: model.copy_tags_to_backups,
            automatic_backup_retention_days: model.automatic_backup_retention_days as i32,
            daily_automatic_backup_start_time: model.daily_automatic_backup_start_time,
            weekly_maintenance_start_time: model.weekly_maintenance_start_time,
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
            if fs.file_system_type != "OPENZFS" {
                continue;
            }
            let tags: HashMap<String, String> = fs
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let oc = fs.open_zfs_configuration.as_ref();
            let throughput_capacity = oc
                .and_then(|v| v.get("throughput_capacity"))
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            let copy_tags_to_volumes = oc
                .and_then(|v| v.get("copy_tags_to_volumes"))
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let root_volume_id = oc
                .and_then(|v| v.get("root_volume_id"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let endpoint_ip_address_range = oc
                .and_then(|v| v.get("endpoint_ip_address_range"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let preferred_subnet_id = oc
                .and_then(|v| v.get("preferred_subnet_id"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
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
                "throughput_capacity": throughput_capacity,
                "copy_tags_to_volumes": copy_tags_to_volumes,
                "root_volume_id": root_volume_id,
                "endpoint_ip_address_range": endpoint_ip_address_range,
                "preferred_subnet_id": preferred_subnet_id,
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

// ---------------------------------------------------------------------------
// Helpers shared by warning-only converters
// ---------------------------------------------------------------------------

/// Inject a resource type that has no slot in `FsxStateView`.
/// The model is parsed (so attribute validation still runs), then a warning
/// is logged and an empty result is returned. Extract is a no-op.
macro_rules! warning_only_converter {
    (
        $struct_name:ident,
        $tf_type:literal,
        $model:ident,
        $warn_msg:literal $(,)?
    ) => {
        pub struct $struct_name {
            service: Arc<FsxService>,
        }

        impl $struct_name {
            pub fn new(service: Arc<FsxService>) -> Self {
                Self { service }
            }
        }

        impl TerraformResourceConverter for $struct_name {
            fn resource_type(&self) -> &str {
                $tf_type
            }

            fn inject<'a>(
                &'a self,
                instance: &'a ResourceInstance,
                ctx: &'a ConversionContext,
            ) -> Pin<
                Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>,
            > {
                Box::pin(async move { self.do_inject(instance, ctx).await })
            }

            fn extract<'a>(
                &'a self,
                _ctx: &'a ConversionContext,
            ) -> Pin<
                Box<
                    dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>>
                        + Send
                        + 'a,
                >,
            > {
                Box::pin(async move { Ok(vec![]) })
            }
        }

        impl $struct_name {
            async fn do_inject(
                &self,
                instance: &ResourceInstance,
                ctx: &ConversionContext,
            ) -> Result<ConversionResult, ConversionError> {
                let region = extract_region(&instance.attributes, &ctx.default_region);
                let _model: fsx_gen::$model = serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error($tf_type, e))?;
                eprintln!(
                    "warning: {}: FsxStateView has no slot; inject is a no-op",
                    $tf_type
                );
                // Touch the service so the merge log records visiting the scope.
                self.service
                    .merge(&ctx.default_account_id, &region, FsxStateView::default())
                    .await?;
                Ok(ConversionResult {
                    region,
                    warnings: vec![$warn_msg.to_string()],
                })
            }
        }
    };
}

// ---------------------------------------------------------------------------
// aws_fsx_data_repository_association
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsFsxDataRepositoryAssociationConverter,
    "aws_fsx_data_repository_association",
    DataRepositoryAssociationTfModel,
    "aws_fsx_data_repository_association state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_fsx_file_cache
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsFsxFileCacheConverter,
    "aws_fsx_file_cache",
    FileCacheTfModel,
    "aws_fsx_file_cache state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_fsx_ontap_storage_virtual_machine
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsFsxOntapStorageVirtualMachineConverter,
    "aws_fsx_ontap_storage_virtual_machine",
    OntapStorageVirtualMachineTfModel,
    "aws_fsx_ontap_storage_virtual_machine state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_fsx_ontap_volume
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsFsxOntapVolumeConverter,
    "aws_fsx_ontap_volume",
    OntapVolumeTfModel,
    "aws_fsx_ontap_volume state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_fsx_openzfs_snapshot
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsFsxOpenzfsSnapshotConverter,
    "aws_fsx_openzfs_snapshot",
    OpenzfsSnapshotTfModel,
    "aws_fsx_openzfs_snapshot state not persisted (no view slot)",
);

// ---------------------------------------------------------------------------
// aws_fsx_openzfs_volume
// ---------------------------------------------------------------------------

warning_only_converter!(
    AwsFsxOpenzfsVolumeConverter,
    "aws_fsx_openzfs_volume",
    OpenzfsVolumeTfModel,
    "aws_fsx_openzfs_volume state not persisted (no view slot)",
);
