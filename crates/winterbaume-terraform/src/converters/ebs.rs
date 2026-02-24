//! Terraform converters for EBS resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_ebs::EbsService;
use winterbaume_ebs::views::{EbsStateView, SnapshotView, VolumeView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_bool, optional_i64, optional_str};

// ---------------------------------------------------------------------------
// aws_ebs_volume
// ---------------------------------------------------------------------------

/// Converts `aws_ebs_volume` Terraform resources to/from EBS state.
pub struct AwsEbsVolumeConverter {
    service: Arc<EbsService>,
}

impl AwsEbsVolumeConverter {
    pub fn new(service: Arc<EbsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEbsVolumeConverter {
    fn resource_type(&self) -> &str {
        "aws_ebs_volume"
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

impl AwsEbsVolumeConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let volume_id = optional_str(attrs, "id")
            .or_else(|| optional_str(attrs, "volume_id"))
            .unwrap_or_else(|| format!("vol-{}", &uuid::Uuid::new_v4().simple().to_string()[..17]));
        let availability_zone =
            optional_str(attrs, "availability_zone").unwrap_or_else(|| format!("{}a", region));
        let size = optional_i64(attrs, "size").unwrap_or(8);
        let volume_type = optional_str(attrs, "type").unwrap_or_else(|| "gp2".to_string());
        let iops = optional_i64(attrs, "iops");
        let throughput = optional_i64(attrs, "throughput");
        let encrypted = optional_bool(attrs, "encrypted").unwrap_or(false);
        let kms_key_id = optional_str(attrs, "kms_key_id");
        let snapshot_id = optional_str(attrs, "snapshot_id");
        let tags = extract_tags(attrs);

        let volume_view = VolumeView {
            volume_id: volume_id.clone(),
            availability_zone,
            size,
            volume_type,
            iops,
            throughput,
            encrypted,
            kms_key_id,
            snapshot_id,
            state: "available".to_string(),
            tags,
        };

        let mut state_view = EbsStateView::default();
        state_view.volumes.insert(volume_id, volume_view);
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
        for vol in view.volumes.values() {
            let attrs = serde_json::json!({
                "id": vol.volume_id,
                "volume_id": vol.volume_id,
                "availability_zone": vol.availability_zone,
                "size": vol.size,
                "type": vol.volume_type,
                "iops": vol.iops,
                "throughput": vol.throughput,
                "encrypted": vol.encrypted,
                "kms_key_id": vol.kms_key_id,
                "snapshot_id": vol.snapshot_id,
                "state": vol.state,
                "tags": vol.tags,
                "tags_all": vol.tags,
            });
            results.push(ExtractedResource {
                name: vol.volume_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ebs_snapshot
// ---------------------------------------------------------------------------

/// Converts `aws_ebs_snapshot` Terraform resources to/from EBS state.
pub struct AwsEbsSnapshotConverter {
    service: Arc<EbsService>,
}

impl AwsEbsSnapshotConverter {
    pub fn new(service: Arc<EbsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEbsSnapshotConverter {
    fn resource_type(&self) -> &str {
        "aws_ebs_snapshot"
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

impl AwsEbsSnapshotConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let snapshot_id = optional_str(attrs, "id")
            .or_else(|| optional_str(attrs, "snapshot_id"))
            .unwrap_or_else(|| format!("snap-{}", uuid::Uuid::new_v4().simple()));
        let volume_size = optional_i64(attrs, "volume_size").unwrap_or(8);
        let block_size = attrs
            .get("block_size")
            .and_then(|v| v.as_i64())
            .unwrap_or(524288) as i32;
        let description = optional_str(attrs, "description").unwrap_or_default();
        let status = optional_str(attrs, "status").unwrap_or_else(|| "completed".to_string());
        let _tags_all = attrs.get("tags_all");
        let _outpost_arn = optional_str(attrs, "outpost_arn");
        let _storage_tier = optional_str(attrs, "storage_tier");
        let _permanent_restore = attrs.get("permanent_restore");
        let _temporary_restore_days = attrs.get("temporary_restore_days");
        let _volume_id = optional_str(attrs, "volume_id");
        let _timeouts = attrs.get("timeouts");
        let _kms_key_id = optional_str(attrs, "kms_key_id");

        let snapshot_view = SnapshotView {
            snapshot_id: snapshot_id.clone(),
            volume_size,
            block_size,
            description,
            status,
            blocks: HashMap::new(),
        };

        let mut state_view = EbsStateView::default();
        state_view.snapshots.insert(snapshot_id, snapshot_view);
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
        for snap in view.snapshots.values() {
            let attrs = serde_json::json!({
                "id": snap.snapshot_id,
                "snapshot_id": snap.snapshot_id,
                "volume_size": snap.volume_size,
                "block_size": snap.block_size,
                "description": snap.description,
                "status": snap.status,
                "owner_id": ctx.default_account_id,
                "owner_alias": null,
                "encrypted": false,
                "data_encryption_key_id": "",
                "tags": {},
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: snap.snapshot_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
