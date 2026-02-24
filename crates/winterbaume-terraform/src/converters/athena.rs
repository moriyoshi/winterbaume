//! Terraform converters for Athena resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_athena::AthenaService;
use winterbaume_athena::views::{AthenaStateView, DataCatalogView, WorkGroupView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_athena_workgroup
// ---------------------------------------------------------------------------

pub struct AwsAthenaWorkgroupConverter {
    service: Arc<AthenaService>,
}

impl AwsAthenaWorkgroupConverter {
    pub fn new(service: Arc<AthenaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAthenaWorkgroupConverter {
    fn resource_type(&self) -> &str {
        "aws_athena_workgroup"
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

impl AwsAthenaWorkgroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_athena_workgroup")?;
        let description = optional_str(attrs, "description").unwrap_or_default();

        // configuration.result_configuration.output_location
        let output_location = attrs
            .get("configuration")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("result_configuration"))
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|rc| rc.get("output_location"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let enforce_work_group_configuration = attrs
            .get("configuration")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("enforce_workgroup_configuration"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let mut tags: HashMap<String, String> = HashMap::new();
        if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.insert(k.clone(), s.to_string());
                }
            }
        }
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let wg_view = WorkGroupView {
            name: name.to_string(),
            state: "ENABLED".to_string(),
            description,
            creation_time: None,
            output_location,
            enforce_work_group_configuration,
            tags,
        };

        let mut state_view = minimal_athena_state_view();
        state_view.work_groups.insert(name.to_string(), wg_view);
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
        for wg in view.work_groups.values() {
            let attrs = serde_json::json!({
                "id": wg.name,
                "name": wg.name,
                "description": wg.description,
                "state": wg.state,
                "tags": wg.tags,
                "tags_all": wg.tags,
            });
            results.push(ExtractedResource {
                name: wg.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_athena_data_catalog
// ---------------------------------------------------------------------------

pub struct AwsAthenaDataCatalogConverter {
    service: Arc<AthenaService>,
}

impl AwsAthenaDataCatalogConverter {
    pub fn new(service: Arc<AthenaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAthenaDataCatalogConverter {
    fn resource_type(&self) -> &str {
        "aws_athena_data_catalog"
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

impl AwsAthenaDataCatalogConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_athena_data_catalog")?;
        let catalog_type = require_str(attrs, "type", "aws_athena_data_catalog")?;
        let description = optional_str(attrs, "description").unwrap_or_default();

        let parameters: HashMap<String, String> = attrs
            .get("parameters")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let mut tags: HashMap<String, String> = HashMap::new();
        if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.insert(k.clone(), s.to_string());
                }
            }
        }

        let catalog_view = DataCatalogView {
            name: name.to_string(),
            catalog_type: catalog_type.to_string(),
            description,
            parameters,
            tags,
        };

        let mut state_view = minimal_athena_state_view();
        state_view
            .data_catalogs
            .insert(name.to_string(), catalog_view);
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
        for catalog in view.data_catalogs.values() {
            let attrs = serde_json::json!({
                "id": catalog.name,
                "name": catalog.name,
                "type": catalog.catalog_type,
                "description": catalog.description,
                "parameters": catalog.parameters,
                "tags": catalog.tags,
            });
            results.push(ExtractedResource {
                name: catalog.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn minimal_athena_state_view() -> AthenaStateView {
    AthenaStateView {
        work_groups: HashMap::new(),
        capacity_reservations: HashMap::new(),
        data_catalogs: HashMap::new(),
        named_queries: HashMap::new(),
        prepared_statements: HashMap::new(),
        tags: HashMap::new(),
    }
}
