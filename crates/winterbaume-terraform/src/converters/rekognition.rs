//! Terraform converter for Rekognition resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_rekognition::RekognitionService;
use winterbaume_rekognition::views::{CollectionView, RekognitionStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_rekognition_collection
// ---------------------------------------------------------------------------

/// Converts `aws_rekognition_collection` Terraform resources to/from Rekognition state.
pub struct AwsRekognitionCollectionConverter {
    service: Arc<RekognitionService>,
}

impl AwsRekognitionCollectionConverter {
    pub fn new(service: Arc<RekognitionService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsRekognitionCollectionConverter {
    fn resource_type(&self) -> &str {
        "aws_rekognition_collection"
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

impl AwsRekognitionCollectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let collection_id = require_str(attrs, "collection_id", "aws_rekognition_collection")?;
        let region = extract_region(attrs, &ctx.default_region);
        let collection_arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:rekognition:{}:{}:collection/{}",
                region, ctx.default_account_id, collection_id
            )
        });
        let face_model_version =
            optional_str(attrs, "face_model_version").unwrap_or_else(|| "6.0".to_string());
        let tags = extract_tags(attrs);

        let collection_view = CollectionView {
            collection_id: collection_id.to_string(),
            collection_arn: collection_arn.clone(),
            face_count: 0,
            face_model_version,
            creation_timestamp: 0.0,
            user_count: 0,
            tags,
        };

        let mut state_view = RekognitionStateView {
            collections: HashMap::new(),
            video_jobs: HashMap::new(),
        };
        state_view
            .collections
            .insert(collection_id.to_string(), collection_view);
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
        for collection in view.collections.values() {
            let attrs = serde_json::json!({
                "id": collection.collection_id,
                "collection_id": collection.collection_id,
                "arn": collection.collection_arn,
                "face_count": collection.face_count,
                "face_model_version": collection.face_model_version,
                "creation_timestamp": collection.creation_timestamp,
                "user_count": collection.user_count,
                "tags": collection.tags,
            });
            results.push(ExtractedResource {
                name: collection.collection_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
