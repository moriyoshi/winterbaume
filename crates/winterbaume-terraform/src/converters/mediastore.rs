//! Terraform converter for MediaStore resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_mediastore::MediaStoreService;
use winterbaume_mediastore::views::{ContainerView, MediaStoreStateView, TagView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_media_store_container
// ---------------------------------------------------------------------------

/// Converts `aws_media_store_container` Terraform resources to/from MediaStore state.
pub struct AwsMediaStoreContainerConverter {
    service: Arc<MediaStoreService>,
}

impl AwsMediaStoreContainerConverter {
    pub fn new(service: Arc<MediaStoreService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMediaStoreContainerConverter {
    fn resource_type(&self) -> &str {
        "aws_media_store_container"
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

impl AwsMediaStoreContainerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_media_store_container")?;
        let region = extract_region(attrs, &ctx.default_region);

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:mediastore:{}:{}:container/{}",
                region, ctx.default_account_id, name
            )
        });
        let endpoint = optional_str(attrs, "endpoint").unwrap_or_else(|| {
            format!(
                "https://{}.mediastore.{}.amazonaws.com",
                uuid::Uuid::new_v4().to_string().replace('-', ""),
                region
            )
        });

        let tags = if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
            obj.iter()
                .map(|(k, v)| TagView {
                    key: k.clone(),
                    value: v.as_str().map(|s| s.to_string()),
                })
                .collect()
        } else {
            vec![]
        };

        let container_view = ContainerView {
            arn,
            name: name.to_string(),
            endpoint,
            status: "ACTIVE".to_string(),
            creation_time: chrono::Utc::now().to_rfc3339(),
            lifecycle_policy: None,
            policy: None,
            metric_policy_json: None,
            tags,
        };

        let mut state_view = MediaStoreStateView {
            containers: HashMap::new(),
        };
        state_view
            .containers
            .insert(name.to_string(), container_view);
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
        for container in view.containers.values() {
            let tags: serde_json::Value = container
                .tags
                .iter()
                .map(|t| {
                    (
                        t.key.clone(),
                        serde_json::Value::String(t.value.clone().unwrap_or_default()),
                    )
                })
                .collect::<serde_json::Map<String, serde_json::Value>>()
                .into();
            let attrs = serde_json::json!({
                "id": container.name,
                "name": container.name,
                "arn": container.arn,
                "endpoint": container.endpoint,
                "tags": tags,
            });
            results.push(ExtractedResource {
                name: container.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
