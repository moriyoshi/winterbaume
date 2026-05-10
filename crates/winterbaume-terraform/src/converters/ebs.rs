//! Terraform converters for EBS resources.
//!
//! `VolumeTfModel` and `SnapshotTfModel` are generated from
//! `specs/ebs.toml`. The synthesised volume/snapshot IDs (UUID), the
//! `availability_zone` `{region}a` template, the Option<i64> `iops` /
//! `throughput` raw reads, the snapshot `block_size` (i32) raw read, and
//! the `state` / `status` constants are wired up here.

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
use crate::generated::ebs as ebs_gen;
use crate::util::{classify_deserialize_error, extract_region, optional_str};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ebs_gen::VolumeTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_ebs_volume", e))?;

        let attrs = &instance.attributes;

        let volume_id = optional_str(attrs, "id")
            .or(model.volume_id)
            .unwrap_or_else(|| format!("vol-{}", &uuid::Uuid::new_v4().simple().to_string()[..17]));
        let availability_zone = model
            .availability_zone
            .unwrap_or_else(|| format!("{}a", region));
        let volume_type = model.volume_type.unwrap_or_else(|| "gp2".to_string());

        // Option<i64> not in spec vocabulary — read raw.
        let iops = attrs.get("iops").and_then(|v| v.as_i64());
        let throughput = attrs.get("throughput").and_then(|v| v.as_i64());

        let volume_view = VolumeView {
            volume_id: volume_id.clone(),
            availability_zone,
            size: model.size,
            volume_type,
            iops,
            throughput,
            encrypted: model.encrypted,
            kms_key_id: model.kms_key_id,
            snapshot_id: model.snapshot_id,
            state: "available".to_string(),
            tags: model.tags,
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ebs_gen::SnapshotTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ebs_snapshot", e))?;

        let attrs = &instance.attributes;

        let snapshot_id = optional_str(attrs, "id")
            .or(model.snapshot_id)
            .unwrap_or_else(|| format!("snap-{}", uuid::Uuid::new_v4().simple()));
        let description = model.description.unwrap_or_default();
        let status = model.status.unwrap_or_else(|| "completed".to_string());
        // i32 not in spec vocabulary — read raw.
        let block_size = attrs
            .get("block_size")
            .and_then(|v| v.as_i64())
            .unwrap_or(524288) as i32;

        let snapshot_view = SnapshotView {
            snapshot_id: snapshot_id.clone(),
            volume_size: model.volume_size,
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
