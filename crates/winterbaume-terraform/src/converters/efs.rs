//! Terraform converter for EFS resources.
//!
//! `FileSystemTfModel` is generated from `specs/efs.toml`. The
//! synthesised file-system ID, the ARN template, the `creation_token`
//! UUID fallback, the Vec<TagView> tag merge, the `lifecycle_policy`
//! and `protection` nested-block parses, and the `Name`-tag-derived
//! `name` field are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_efs::EfsService;
use winterbaume_efs::views::{
    AccessPointView, EfsStateView, FileSizeValueView, FileSystemProtectionView, FileSystemView,
    LifecyclePolicyView, MountTargetView, ReplicationConfigurationView, TagView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::efs as efs_gen;
use crate::util::{classify_deserialize_error, extract_region};

/// Converts `aws_efs_file_system` Terraform resources to/from EFS state.
pub struct AwsEfsFileSystemConverter {
    service: Arc<EfsService>,
}

impl AwsEfsFileSystemConverter {
    pub fn new(service: Arc<EfsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEfsFileSystemConverter {
    fn resource_type(&self) -> &str {
        "aws_efs_file_system"
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

impl AwsEfsFileSystemConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: efs_gen::FileSystemTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_efs_file_system", e))?;

        let attrs = &instance.attributes;

        let fs_id = model
            .id
            .unwrap_or_else(|| format!("fs-{}", &uuid::Uuid::new_v4().to_string()[..8]));

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:elasticfilesystem:{}:{}:file-system/{}",
                region, ctx.default_account_id, fs_id
            )
        });

        let performance_mode = model
            .performance_mode
            .unwrap_or_else(|| "generalPurpose".to_string());
        let throughput_mode = model
            .throughput_mode
            .unwrap_or_else(|| "bursting".to_string());
        let encrypted = model.encrypted;

        // Tags — view stores Vec<TagView>, merge tags_all (lower precedence) and tags.
        let tags: Vec<TagView> = {
            let mut map: HashMap<String, String> = HashMap::new();
            if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
                for (k, v) in obj {
                    if let Some(s) = v.as_str() {
                        map.insert(k.clone(), s.to_string());
                    }
                }
            }
            if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
                for (k, v) in obj {
                    if let Some(s) = v.as_str() {
                        map.insert(k.clone(), s.to_string());
                    }
                }
            }
            map.into_iter()
                .map(|(k, v)| TagView { key: k, value: v })
                .collect()
        };

        let name_tag = tags
            .iter()
            .find(|t| t.key == "Name")
            .map(|t| t.value.clone());
        let creation_token = model
            .creation_token
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        // Parse lifecycle_policy blocks
        let lifecycle_policies: Vec<LifecyclePolicyView> = attrs
            .get("lifecycle_policy")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        let obj = item.as_object()?;
                        Some(LifecyclePolicyView {
                            transition_to_ia: obj
                                .get("transition_to_ia")
                                .and_then(|v| v.as_str())
                                .filter(|s| !s.is_empty())
                                .map(|s| s.to_string()),
                            transition_to_primary_storage_class: obj
                                .get("transition_to_primary_storage_class")
                                .and_then(|v| v.as_str())
                                .filter(|s| !s.is_empty())
                                .map(|s| s.to_string()),
                            transition_to_archive: obj
                                .get("transition_to_archive")
                                .and_then(|v| v.as_str())
                                .filter(|s| !s.is_empty())
                                .map(|s| s.to_string()),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Parse protection block
        let protection: Option<FileSystemProtectionView> = attrs
            .get("protection")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.as_object())
            .map(|obj| FileSystemProtectionView {
                replication_overwrite_protection: obj
                    .get("replication_overwrite")
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string()),
            });

        let fs_view = FileSystemView {
            file_system_id: fs_id.clone(),
            file_system_arn: arn.clone(),
            creation_time: Some(Utc::now().to_rfc3339()),
            owner_id: ctx.default_account_id.clone(),
            creation_token: creation_token.clone(),
            performance_mode,
            throughput_mode,
            life_cycle_state: "available".to_string(),
            number_of_mount_targets: 0,
            size_in_bytes: FileSizeValueView {
                value: 0,
                timestamp: None,
                value_in_ia: 0,
                value_in_standard: 0,
            },
            encrypted,
            tags,
            name: name_tag,
            policy: None,
            lifecycle_policies,
            backup_policy_status: None,
        };

        let mut state_view = EfsStateView::default();
        state_view.file_systems.insert(fs_id.clone(), fs_view);
        state_view
            .creation_tokens
            .insert(creation_token, fs_id.clone());
        if let Some(prot) = protection {
            state_view.fs_protection.insert(fs_id, prot);
        }
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
            let tags: HashMap<String, String> = fs
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();

            // Serialize lifecycle_policy as array of objects
            let lifecycle_policy: Vec<serde_json::Value> = fs
                .lifecycle_policies
                .iter()
                .map(|lp| {
                    let mut obj = serde_json::Map::new();
                    if let Some(ref v) = lp.transition_to_ia {
                        obj.insert(
                            "transition_to_ia".to_string(),
                            serde_json::Value::String(v.clone()),
                        );
                    } else {
                        obj.insert(
                            "transition_to_ia".to_string(),
                            serde_json::Value::String(String::new()),
                        );
                    }
                    if let Some(ref v) = lp.transition_to_primary_storage_class {
                        obj.insert(
                            "transition_to_primary_storage_class".to_string(),
                            serde_json::Value::String(v.clone()),
                        );
                    } else {
                        obj.insert(
                            "transition_to_primary_storage_class".to_string(),
                            serde_json::Value::String(String::new()),
                        );
                    }
                    if let Some(ref v) = lp.transition_to_archive {
                        obj.insert(
                            "transition_to_archive".to_string(),
                            serde_json::Value::String(v.clone()),
                        );
                    } else {
                        obj.insert(
                            "transition_to_archive".to_string(),
                            serde_json::Value::String(String::new()),
                        );
                    }
                    serde_json::Value::Object(obj)
                })
                .collect();

            // Serialize protection as array of one object
            let protection: Vec<serde_json::Value> = view
                .fs_protection
                .get(&fs.file_system_id)
                .map(|prot| {
                    let mut obj = serde_json::Map::new();
                    obj.insert(
                        "replication_overwrite".to_string(),
                        serde_json::Value::String(
                            prot.replication_overwrite_protection
                                .clone()
                                .unwrap_or_default(),
                        ),
                    );
                    vec![serde_json::Value::Object(obj)]
                })
                .unwrap_or_default();

            let attrs = serde_json::json!({
                "id": fs.file_system_id,
                "arn": fs.file_system_arn,
                "creation_token": fs.creation_token,
                "life_cycle_state": fs.life_cycle_state,
                "number_of_mount_targets": fs.number_of_mount_targets,
                "size_in_bytes": {
                    "value": fs.size_in_bytes.value,
                    "value_in_ia": fs.size_in_bytes.value_in_ia,
                    "value_in_standard": fs.size_in_bytes.value_in_standard,
                },
                "name": fs.name,
                "performance_mode": fs.performance_mode,
                "throughput_mode": fs.throughput_mode,
                "encrypted": fs.encrypted,
                "tags": tags,
                "lifecycle_policy": lifecycle_policy,
                "protection": protection,
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
// aws_efs_access_point
// ---------------------------------------------------------------------------

pub struct AwsEfsAccessPointConverter {
    service: Arc<EfsService>,
}

impl AwsEfsAccessPointConverter {
    pub fn new(service: Arc<EfsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEfsAccessPointConverter {
    fn resource_type(&self) -> &str {
        "aws_efs_access_point"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_efs_file_system"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let attrs = &instance.attributes;
            let region = extract_region(attrs, &ctx.default_region);
            let model: efs_gen::AccessPointTfModel = serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_efs_access_point", e))?;

            let ap_id = model
                .id
                .unwrap_or_else(|| format!("fsap-{}", &uuid::Uuid::new_v4().to_string()[..8]));
            let arn = model.arn.unwrap_or_else(|| {
                format!(
                    "arn:aws:elasticfilesystem:{}:{}:access-point/{}",
                    region, ctx.default_account_id, ap_id
                )
            });

            let tags: Vec<TagView> = attrs
                .get("tags")
                .and_then(|v| v.as_object())
                .map(|obj| {
                    obj.iter()
                        .filter_map(|(k, v)| {
                            v.as_str().map(|s| TagView {
                                key: k.clone(),
                                value: s.to_string(),
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            let ap = AccessPointView {
                access_point_id: ap_id.clone(),
                access_point_arn: arn,
                client_token: ap_id.clone(),
                file_system_id: model.file_system_id,
                life_cycle_state: "available".to_string(),
                name: model.name,
                owner_id: ctx.default_account_id.clone(),
                posix_user: None,
                root_directory: None,
                tags,
            };

            let mut state_view = EfsStateView::default();
            state_view.access_points.insert(ap_id, ap);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for ap in view.access_points.values() {
                let tags: HashMap<String, String> = ap
                    .tags
                    .iter()
                    .map(|t| (t.key.clone(), t.value.clone()))
                    .collect();
                let attrs = serde_json::json!({
                    "id": ap.access_point_id,
                    "arn": ap.access_point_arn,
                    "file_system_id": ap.file_system_id,
                    "name": ap.name,
                    "owner_id": ap.owner_id,
                    "tags": tags,
                    "tags_all": tags,
                });
                results.push(ExtractedResource {
                    name: ap.access_point_id.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_efs_backup_policy — mutates existing file_system (warning-only).
// ---------------------------------------------------------------------------

pub struct AwsEfsBackupPolicyConverter {
    #[allow(dead_code)]
    service: Arc<EfsService>,
}

impl AwsEfsBackupPolicyConverter {
    pub fn new(service: Arc<EfsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEfsBackupPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_efs_backup_policy"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: efs_gen::BackupPolicyTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_efs_backup_policy", e))?;
            let warn_msg = "backup_policy attaches to an existing FileSystemView field; \
                            no merge path exposed via the view layer — inject is a no-op"
                .to_string();
            eprintln!("warning: aws_efs_backup_policy: {warn_msg}");
            Ok(ConversionResult {
                region,
                warnings: vec![format!("aws_efs_backup_policy: {warn_msg}")],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_efs_file_system_policy — mutates existing file_system (warning-only).
// ---------------------------------------------------------------------------

pub struct AwsEfsFileSystemPolicyConverter {
    #[allow(dead_code)]
    service: Arc<EfsService>,
}

impl AwsEfsFileSystemPolicyConverter {
    pub fn new(service: Arc<EfsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEfsFileSystemPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_efs_file_system_policy"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: efs_gen::FileSystemPolicyTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_efs_file_system_policy", e))?;
            let warn_msg = "file_system policy attaches to an existing FileSystemView.policy \
                            field; no merge path exposed via the view layer — inject is a no-op"
                .to_string();
            eprintln!("warning: aws_efs_file_system_policy: {warn_msg}");
            Ok(ConversionResult {
                region,
                warnings: vec![format!("aws_efs_file_system_policy: {warn_msg}")],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_efs_mount_target
// ---------------------------------------------------------------------------

pub struct AwsEfsMountTargetConverter {
    service: Arc<EfsService>,
}

impl AwsEfsMountTargetConverter {
    pub fn new(service: Arc<EfsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEfsMountTargetConverter {
    fn resource_type(&self) -> &str {
        "aws_efs_mount_target"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_efs_file_system"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let attrs = &instance.attributes;
            let region = extract_region(attrs, &ctx.default_region);
            let model: efs_gen::MountTargetTfModel = serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_efs_mount_target", e))?;

            let mt_id = model
                .id
                .unwrap_or_else(|| format!("fsmt-{}", &uuid::Uuid::new_v4().to_string()[..8]));

            let security_groups: Vec<String> = attrs
                .get("security_groups")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();

            let mt = MountTargetView {
                mount_target_id: mt_id.clone(),
                file_system_id: model.file_system_id,
                subnet_id: model.subnet_id,
                life_cycle_state: "available".to_string(),
                ip_address: model.ip_address.unwrap_or_else(|| "127.0.0.1".to_string()),
                network_interface_id: format!("eni-{}", &uuid::Uuid::new_v4().to_string()[..8]),
                availability_zone_id: format!("{}-az1", region),
                availability_zone_name: format!("{}a", region),
                owner_id: ctx.default_account_id.clone(),
                security_groups,
            };

            let mut state_view = EfsStateView::default();
            state_view.mount_targets.insert(mt_id, mt);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for mt in view.mount_targets.values() {
                let attrs = serde_json::json!({
                    "id": mt.mount_target_id,
                    "file_system_id": mt.file_system_id,
                    "subnet_id": mt.subnet_id,
                    "ip_address": mt.ip_address,
                    "network_interface_id": mt.network_interface_id,
                    "availability_zone_id": mt.availability_zone_id,
                    "availability_zone_name": mt.availability_zone_name,
                    "owner_id": mt.owner_id,
                    "security_groups": mt.security_groups,
                });
                results.push(ExtractedResource {
                    name: mt.mount_target_id.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ---------------------------------------------------------------------------
// aws_efs_replication_configuration
// ---------------------------------------------------------------------------

pub struct AwsEfsReplicationConfigurationConverter {
    service: Arc<EfsService>,
}

impl AwsEfsReplicationConfigurationConverter {
    pub fn new(service: Arc<EfsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEfsReplicationConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_efs_replication_configuration"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_efs_file_system"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let attrs = &instance.attributes;
            let region = extract_region(attrs, &ctx.default_region);
            let model: efs_gen::ReplicationConfigurationTfModel =
                serde_json::from_value(attrs.clone()).map_err(|e| {
                    classify_deserialize_error("aws_efs_replication_configuration", e)
                })?;

            let source_arn = format!(
                "arn:aws:elasticfilesystem:{}:{}:file-system/{}",
                region, ctx.default_account_id, model.source_file_system_id
            );

            let rc = ReplicationConfigurationView {
                source_file_system_id: model.source_file_system_id.clone(),
                source_file_system_arn: source_arn.clone(),
                original_source_file_system_arn: source_arn,
                source_file_system_region: region.clone(),
                source_file_system_owner_id: ctx.default_account_id.clone(),
                creation_time: Some(Utc::now().to_rfc3339()),
                destinations: vec![],
            };

            let mut state_view = EfsStateView::default();
            state_view
                .replication_configurations
                .insert(model.source_file_system_id, rc);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;

            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for r in view.replication_configurations.values() {
                let attrs = serde_json::json!({
                    "id": r.source_file_system_id,
                    "source_file_system_id": r.source_file_system_id,
                    "source_file_system_arn": r.source_file_system_arn,
                    "original_source_file_system_arn": r.original_source_file_system_arn,
                    "source_file_system_region": r.source_file_system_region,
                    "creation_time": r.creation_time,
                });
                results.push(ExtractedResource {
                    name: r.source_file_system_id.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}
