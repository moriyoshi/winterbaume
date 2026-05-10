//! Terraform converter for Rekognition resources.
//!
//! `CollectionTfModel` is generated from `specs/rekognition.toml`. The
//! constant counters (`face_count = 0`, `user_count = 0`,
//! `creation_timestamp = 0.0`) and the ARN template are wired up here.

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
use crate::generated::rekognition as rekognition_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: rekognition_gen::CollectionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_rekognition_collection", e))?;

        let collection_id = model.collection_id.clone();
        let collection_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:rekognition:{}:{}:collection/{}",
                region, ctx.default_account_id, collection_id
            )
        });
        let face_model_version = model
            .face_model_version
            .unwrap_or_else(|| "6.0".to_string());

        let collection_view = CollectionView {
            collection_id: collection_id.clone(),
            collection_arn,
            face_count: 0,
            face_model_version,
            creation_timestamp: 0.0,
            user_count: 0,
            tags: model.tags,
        };

        let mut state_view = RekognitionStateView {
            collections: HashMap::new(),
            video_jobs: HashMap::new(),
        };
        state_view
            .collections
            .insert(collection_id, collection_view);
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
