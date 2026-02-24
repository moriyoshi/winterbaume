//! Terraform converter for EFS resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_efs::EfsService;
use winterbaume_efs::views::{
    EfsStateView, FileSizeValueView, FileSystemProtectionView, FileSystemView, LifecyclePolicyView,
    TagView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_bool, optional_str};

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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let fs_id = optional_str(attrs, "id")
            .unwrap_or_else(|| format!("fs-{}", &uuid::Uuid::new_v4().to_string()[..8]));

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:elasticfilesystem:{}:{}:file-system/{}",
                region, ctx.default_account_id, fs_id
            )
        });

        let _tags_all = attrs.get("tags_all");
        let _availability_zone_name = optional_str(attrs, "availability_zone_name");
        let _kms_key_id = optional_str(attrs, "kms_key_id");
        let _provisioned_throughput_in_mibps = attrs.get("provisioned_throughput_in_mibps");
        let _ = attrs.get("lifecycle_policy");
        let performance_mode =
            optional_str(attrs, "performance_mode").unwrap_or_else(|| "generalPurpose".to_string());
        let throughput_mode =
            optional_str(attrs, "throughput_mode").unwrap_or_else(|| "bursting".to_string());
        let encrypted = optional_bool(attrs, "encrypted").unwrap_or(false);

        // Tags
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
        let creation_token = optional_str(attrs, "creation_token")
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
