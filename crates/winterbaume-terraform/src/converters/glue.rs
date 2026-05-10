//! Terraform converters for Glue resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_glue::GlueService;
use winterbaume_glue::views::{CrawlerView, DatabaseView, GlueStateView, JobView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::glue as glue_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_glue_catalog_database
// ---------------------------------------------------------------------------

pub struct AwsGlueCatalogDatabaseConverter {
    service: Arc<GlueService>,
}

impl AwsGlueCatalogDatabaseConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlueCatalogDatabaseConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_catalog_database"
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

impl AwsGlueCatalogDatabaseConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glue_gen::CatalogDatabaseTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_glue_catalog_database", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let description = model.description.unwrap_or_default();
        let location_uri = model.location_uri.unwrap_or_default();

        let parameters: HashMap<String, String> = attrs
            .get("parameters")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let catalog_id = model
            .catalog_id
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let db_view = DatabaseView {
            name: name.clone(),
            description,
            location_uri,
            parameters,
            create_time: Utc::now().to_rfc3339(),
            catalog_id,
        };

        let mut state_view = GlueStateView::default();
        state_view.databases.insert(name, db_view);
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
        for db in view.databases.values() {
            let attrs = serde_json::json!({
                "id": format!("{}:{}", db.catalog_id, db.name),
                "name": db.name,
                "catalog_id": db.catalog_id,
                "description": db.description,
                "location_uri": db.location_uri,
                "parameters": db.parameters,
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: db.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_glue_job
// ---------------------------------------------------------------------------

pub struct AwsGlueJobConverter {
    service: Arc<GlueService>,
}

impl AwsGlueJobConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlueJobConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_job"
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

impl AwsGlueJobConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glue_gen::JobTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_glue_job", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let role = model.role_arn.unwrap_or_default();
        let description = model.description.unwrap_or_default();
        let worker_type = model.worker_type;
        let glue_version = model.glue_version;
        let max_retries = model.max_retries as i32;
        let timeout = model.timeout as i32;
        let number_of_workers = attrs
            .get("number_of_workers")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let max_capacity = attrs.get("max_capacity").and_then(|v| v.as_f64());

        let command = attrs.get("command").cloned();

        let default_arguments: HashMap<String, String> = attrs
            .get("default_arguments")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let now = Utc::now().to_rfc3339();
        let job_view = JobView {
            name: name.clone(),
            description,
            role,
            command,
            default_arguments,
            max_retries,
            timeout,
            max_capacity,
            number_of_workers,
            worker_type,
            glue_version,
            created_on: now.clone(),
            last_modified_on: now,
        };

        let mut state_view = GlueStateView::default();
        state_view.jobs.insert(name, job_view);
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
        for job in view.jobs.values() {
            let arn = format!(
                "arn:aws:glue:{}:{}:job/{}",
                ctx.default_region, ctx.default_account_id, job.name
            );
            let attrs = serde_json::json!({
                "id": job.name,
                "name": job.name,
                "role_arn": job.role,
                "description": job.description,
                "worker_type": job.worker_type,
                "glue_version": job.glue_version,
                "max_retries": job.max_retries,
                "timeout": job.timeout,
                "number_of_workers": job.number_of_workers,
                "tags_all": {},
                "connections": [],
                "max_capacity": job.max_capacity,
                "arn": arn,
            });
            results.push(ExtractedResource {
                name: job.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_glue_crawler
// ---------------------------------------------------------------------------

pub struct AwsGlueCrawlerConverter {
    service: Arc<GlueService>,
}

impl AwsGlueCrawlerConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlueCrawlerConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_crawler"
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

impl AwsGlueCrawlerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glue_gen::CrawlerTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_glue_crawler", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let role = model.role.unwrap_or_default();
        let database_name = model.database_name.unwrap_or_default();
        let description = model.description.unwrap_or_default();
        let schedule = model.schedule;
        let table_prefix = model.table_prefix.unwrap_or_default();

        // Merge s3_target, dynamodb_target, etc. as JSON targets
        let targets = attrs.get("s3_target").cloned();

        let now = Utc::now().to_rfc3339();
        let crawler_view = CrawlerView {
            name: name.clone(),
            role,
            database_name,
            description,
            targets,
            schedule,
            classifiers: vec![],
            table_prefix,
            configuration: String::new(),
            state: "READY".to_string(),
            creation_time: now.clone(),
            last_updated: now,
            version: 1,
        };

        let mut state_view = GlueStateView::default();
        state_view.crawlers.insert(name, crawler_view);
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
        for crawler in view.crawlers.values() {
            let attrs = serde_json::json!({
                "id": crawler.name,
                "name": crawler.name,
                "role": crawler.role,
                "database_name": crawler.database_name,
                "description": crawler.description,
                "schedule": crawler.schedule,
                "state": crawler.state,
                "table_prefix": crawler.table_prefix,
                "creation_time": crawler.creation_time,
                "last_updated": crawler.last_updated,
                "version": crawler.version,
                "classifiers": crawler.classifiers,
                "configuration": crawler.configuration,
                "tags_all": {},
                "lineage_configuration": [{"crawler_lineage_settings": "DISABLE"}],
                "recrawl_policy": [{"recrawl_behavior": "CRAWL_EVERYTHING"}],
                "schema_change_policy": [{"delete_behavior": "DEPRECATE_IN_DATABASE", "update_behavior": "UPDATE_IN_DATABASE"}],
                "arn": format!("arn:aws:glue:{}:000000000000:crawler/{}", ctx.default_region, crawler.name),
            });
            results.push(ExtractedResource {
                name: crawler.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
