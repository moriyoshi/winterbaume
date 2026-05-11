//! Terraform converters for Glue resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_glue::GlueService;
use winterbaume_glue::views::{
    ConnectionView, CrawlerView, DataCatalogEncryptionSettingsView, DatabaseView, DevEndpointView,
    GlueStateView, JobView, MLTransformView, PartitionView, RegistryView, ResourcePolicyView,
    SchemaView, SecurityConfigurationView, TableView, TriggerView, WorkflowView,
};
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

// ---------------------------------------------------------------------------
// aws_glue_catalog_table
// ---------------------------------------------------------------------------

pub struct AwsGlueCatalogTableConverter {
    service: Arc<GlueService>,
}

impl AwsGlueCatalogTableConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlueCatalogTableConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_catalog_table"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_glue_catalog_database"]
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

impl AwsGlueCatalogTableConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glue_gen::CatalogTableTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_glue_catalog_table", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let database_name = model.database_name.clone();
        let catalog_id = model
            .catalog_id
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let description = model.description.unwrap_or_default();
        let owner = model.owner.unwrap_or_default();
        let table_type = model.table_type.unwrap_or_default();
        let retention = model.retention as i32;

        let parameters: HashMap<String, String> = attrs
            .get("parameters")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let storage_descriptor = attrs.get("storage_descriptor").cloned();
        let partition_keys = attrs.get("partition_keys").cloned();

        let now = Utc::now().to_rfc3339();
        let table_view = TableView {
            name: name.clone(),
            database_name: database_name.clone(),
            catalog_id,
            description,
            owner,
            create_time: now.clone(),
            update_time: now,
            table_type,
            parameters,
            storage_descriptor,
            partition_keys,
            retention,
            versions: vec![],
            version_id: "0".to_string(),
        };

        // Use the same `dbname\0tblname` composite key as the view From impl.
        let key = format!("{}\x00{}", database_name, name);
        let mut state_view = GlueStateView::default();
        state_view.tables.insert(key, table_view);
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
        for tbl in view.tables.values() {
            let id = format!("{}:{}:{}", tbl.catalog_id, tbl.database_name, tbl.name);
            let arn = format!(
                "arn:aws:glue:{}:{}:table/{}/{}",
                ctx.default_region, tbl.catalog_id, tbl.database_name, tbl.name
            );
            let attrs = serde_json::json!({
                "id": id,
                "arn": arn,
                "name": tbl.name,
                "database_name": tbl.database_name,
                "catalog_id": tbl.catalog_id,
                "description": tbl.description,
                "owner": tbl.owner,
                "table_type": tbl.table_type,
                "retention": tbl.retention,
                "parameters": tbl.parameters,
                "storage_descriptor": tbl.storage_descriptor,
                "partition_keys": tbl.partition_keys,
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: format!("{}_{}", tbl.database_name, tbl.name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_glue_connection
// ---------------------------------------------------------------------------

pub struct AwsGlueConnectionConverter {
    service: Arc<GlueService>,
}

impl AwsGlueConnectionConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlueConnectionConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_connection"
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

impl AwsGlueConnectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glue_gen::ConnectionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_glue_connection", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let connection_type = model.connection_type.unwrap_or_else(|| "JDBC".to_string());
        let description = model.description.unwrap_or_default();

        let connection_properties: HashMap<String, String> = attrs
            .get("connection_properties")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let match_criteria: Vec<String> = attrs
            .get("match_criteria")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let physical_connection_requirements =
            attrs.get("physical_connection_requirements").cloned();

        let now = Utc::now().to_rfc3339();
        let conn_view = ConnectionView {
            name: name.clone(),
            connection_type,
            connection_properties,
            description,
            creation_time: now.clone(),
            last_updated_time: now,
            match_criteria,
            physical_connection_requirements,
        };

        let mut state_view = GlueStateView::default();
        state_view.connections.insert(name, conn_view);
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
        for conn in view.connections.values() {
            let id = format!("{}:{}", ctx.default_account_id, conn.name);
            let arn = format!(
                "arn:aws:glue:{}:{}:connection/{}",
                ctx.default_region, ctx.default_account_id, conn.name
            );
            let attrs = serde_json::json!({
                "id": id,
                "arn": arn,
                "name": conn.name,
                "catalog_id": ctx.default_account_id,
                "connection_type": conn.connection_type,
                "connection_properties": conn.connection_properties,
                "description": conn.description,
                "match_criteria": conn.match_criteria,
                "physical_connection_requirements": conn.physical_connection_requirements,
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: conn.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_glue_data_catalog_encryption_settings
// ---------------------------------------------------------------------------

pub struct AwsGlueDataCatalogEncryptionSettingsConverter {
    service: Arc<GlueService>,
}

impl AwsGlueDataCatalogEncryptionSettingsConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlueDataCatalogEncryptionSettingsConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_data_catalog_encryption_settings"
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

impl AwsGlueDataCatalogEncryptionSettingsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: glue_gen::DataCatalogEncryptionSettingsTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_glue_data_catalog_encryption_settings", e)
            })?;

        let attrs = &instance.attributes;
        // The TF `data_catalog_encryption_settings` block carries the two nested
        // policy blocks. They may be addressed either at the top level or under
        // a `data_catalog_encryption_settings` wrapper depending on how the
        // state was serialised.
        let wrapper = attrs.get("data_catalog_encryption_settings");
        let outer = wrapper
            .and_then(|w| w.as_array())
            .and_then(|arr| arr.first())
            .or(wrapper)
            .unwrap_or(attrs);

        let encryption_at_rest = outer.get("encryption_at_rest").cloned();
        let connection_password_encryption = outer.get("connection_password_encryption").cloned();

        let view = DataCatalogEncryptionSettingsView {
            encryption_at_rest,
            connection_password_encryption,
        };

        let state_view = GlueStateView {
            data_catalog_encryption_settings: Some(view),
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
        if let Some(settings) = &view.data_catalog_encryption_settings {
            let id = ctx.default_account_id.clone();
            let attrs = serde_json::json!({
                "id": id,
                "catalog_id": ctx.default_account_id,
                "data_catalog_encryption_settings": [{
                    "encryption_at_rest": settings.encryption_at_rest,
                    "connection_password_encryption": settings.connection_password_encryption,
                }],
            });
            results.push(ExtractedResource {
                name: ctx.default_account_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_glue_dev_endpoint
// ---------------------------------------------------------------------------

pub struct AwsGlueDevEndpointConverter {
    service: Arc<GlueService>,
}

impl AwsGlueDevEndpointConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlueDevEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_dev_endpoint"
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

impl AwsGlueDevEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glue_gen::DevEndpointTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_glue_dev_endpoint", e))?;

        let attrs = &instance.attributes;
        let endpoint_name = model.endpoint_name.clone();
        let role_arn = model.role_arn.unwrap_or_default();
        let subnet_id = model.subnet_id.unwrap_or_default();
        let number_of_nodes = model.number_of_nodes as i32;
        let worker_type = model.worker_type;
        let glue_version = model.glue_version;
        let public_key = model.public_key;
        let extra_python_libs_s3_path = model.extra_python_libs_s3_path;
        let extra_jars_s3_path = model.extra_jars_s3_path;

        let security_group_ids: Vec<String> = attrs
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let public_keys: Vec<String> = attrs
            .get("public_keys")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let number_of_workers = attrs
            .get("number_of_workers")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);

        let arguments: HashMap<String, String> = attrs
            .get("arguments")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let now = Utc::now().to_rfc3339();
        let dev_endpoint_view = DevEndpointView {
            endpoint_name: endpoint_name.clone(),
            role_arn,
            security_group_ids,
            subnet_id,
            number_of_nodes,
            number_of_workers,
            worker_type,
            glue_version,
            status: "READY".to_string(),
            created_timestamp: now.clone(),
            last_modified_timestamp: now,
            public_key,
            public_keys,
            extra_python_libs_s3_path,
            extra_jars_s3_path,
            arguments,
        };

        let mut state_view = GlueStateView::default();
        state_view
            .dev_endpoints
            .insert(endpoint_name, dev_endpoint_view);
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
        for dev in view.dev_endpoints.values() {
            let arn = format!(
                "arn:aws:glue:{}:{}:devEndpoint/{}",
                ctx.default_region, ctx.default_account_id, dev.endpoint_name
            );
            let attrs = serde_json::json!({
                "id": dev.endpoint_name,
                "name": dev.endpoint_name,
                "arn": arn,
                "role_arn": dev.role_arn,
                "subnet_id": dev.subnet_id,
                "number_of_nodes": dev.number_of_nodes,
                "number_of_workers": dev.number_of_workers,
                "worker_type": dev.worker_type,
                "glue_version": dev.glue_version,
                "status": dev.status,
                "created_timestamp": dev.created_timestamp,
                "last_modified_timestamp": dev.last_modified_timestamp,
                "public_key": dev.public_key,
                "public_keys": dev.public_keys,
                "extra_python_libs_s3_path": dev.extra_python_libs_s3_path,
                "extra_jars_s3_path": dev.extra_jars_s3_path,
                "arguments": dev.arguments,
                "security_group_ids": dev.security_group_ids,
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: dev.endpoint_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_glue_ml_transform
// ---------------------------------------------------------------------------

pub struct AwsGlueMlTransformConverter {
    service: Arc<GlueService>,
}

impl AwsGlueMlTransformConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlueMlTransformConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_ml_transform"
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

impl AwsGlueMlTransformConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glue_gen::MlTransformTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_glue_ml_transform", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let role = model.role.unwrap_or_default();
        let description = model.description.unwrap_or_default();
        let glue_version = model.glue_version;
        let worker_type = model.worker_type;

        let max_capacity = attrs.get("max_capacity").and_then(|v| v.as_f64());
        let max_retries = attrs
            .get("max_retries")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let timeout = attrs
            .get("timeout")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let number_of_workers = attrs
            .get("number_of_workers")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let parameters = attrs.get("parameters").cloned();
        let input_record_tables: Vec<serde_json::Value> = attrs
            .get("input_record_tables")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        // The transform_id is normally returned by Glue; if the TF state carries
        // an `id`, prefer it, otherwise synthesise one from the name so the
        // state map key is stable.
        let transform_id = attrs
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("tfm-{}", name));

        let now = Utc::now().to_rfc3339();
        let view = MLTransformView {
            transform_id: transform_id.clone(),
            name,
            description,
            role,
            glue_version,
            max_capacity,
            max_retries,
            timeout,
            number_of_workers,
            worker_type,
            parameters,
            input_record_tables,
            status: "READY".to_string(),
            created_on: now.clone(),
            last_modified_on: now,
        };

        let mut state_view = GlueStateView::default();
        state_view.ml_transforms.insert(transform_id, view);
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
        for ml in view.ml_transforms.values() {
            let arn = format!(
                "arn:aws:glue:{}:{}:mlTransform/{}",
                ctx.default_region, ctx.default_account_id, ml.transform_id
            );
            let attrs = serde_json::json!({
                "id": ml.transform_id,
                "name": ml.name,
                "arn": arn,
                "role_arn": ml.role,
                "description": ml.description,
                "glue_version": ml.glue_version,
                "max_capacity": ml.max_capacity,
                "max_retries": ml.max_retries,
                "timeout": ml.timeout,
                "number_of_workers": ml.number_of_workers,
                "worker_type": ml.worker_type,
                "parameters": ml.parameters,
                "input_record_tables": ml.input_record_tables,
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: ml.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_glue_partition
// ---------------------------------------------------------------------------

pub struct AwsGluePartitionConverter {
    service: Arc<GlueService>,
}

impl AwsGluePartitionConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGluePartitionConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_partition"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_glue_catalog_table"]
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

impl AwsGluePartitionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glue_gen::PartitionTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_glue_partition", e))?;

        let attrs = &instance.attributes;
        let database_name = model.database_name.clone();
        let table_name = model.table_name.clone();
        let catalog_id = model
            .catalog_id
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let values: Vec<String> = attrs
            .get("partition_values")
            .and_then(|v| v.as_array())
            .or_else(|| attrs.get("values").and_then(|v| v.as_array()))
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let parameters: HashMap<String, String> = attrs
            .get("parameters")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let storage_descriptor = attrs.get("storage_descriptor").cloned();

        let now = Utc::now().to_rfc3339();
        let partition_view = PartitionView {
            values: values.clone(),
            database_name: database_name.clone(),
            table_name: table_name.clone(),
            catalog_id,
            creation_time: now,
            last_access_time: None,
            parameters,
            storage_descriptor,
        };

        // The composite key matches the `partition_key_view` helper in views.rs.
        let values_key = values.join("#");
        let key = format!("{}\x00{}\x00{}", database_name, table_name, values_key);
        let mut state_view = GlueStateView::default();
        state_view.partitions.insert(key, partition_view);
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
        for p in view.partitions.values() {
            let id = format!(
                "{}:{}:{}:{}",
                p.catalog_id,
                p.database_name,
                p.table_name,
                p.values.join("#")
            );
            let attrs = serde_json::json!({
                "id": id,
                "database_name": p.database_name,
                "table_name": p.table_name,
                "catalog_id": p.catalog_id,
                "partition_values": p.values,
                "values": p.values,
                "creation_time": p.creation_time,
                "last_accessed_time": p.last_access_time,
                "parameters": p.parameters,
                "storage_descriptor": p.storage_descriptor,
            });
            results.push(ExtractedResource {
                name: format!(
                    "{}_{}_{}",
                    p.database_name,
                    p.table_name,
                    p.values.join("_")
                ),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_glue_registry
// ---------------------------------------------------------------------------

pub struct AwsGlueRegistryConverter {
    service: Arc<GlueService>,
}

impl AwsGlueRegistryConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlueRegistryConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_registry"
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

impl AwsGlueRegistryConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glue_gen::RegistryTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_glue_registry", e))?;

        let attrs = &instance.attributes;
        let registry_name = model.registry_name.clone();
        let description = model.description.unwrap_or_default();

        let tags: HashMap<String, String> = attrs
            .get("tags_all")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let registry_arn = format!(
            "arn:aws:glue:{}:{}:registry/{}",
            region, ctx.default_account_id, registry_name
        );

        let now = Utc::now().to_rfc3339();
        let view = RegistryView {
            registry_name: registry_name.clone(),
            registry_arn,
            description,
            created_time: now.clone(),
            updated_time: now,
            status: "AVAILABLE".to_string(),
            tags,
        };

        let mut state_view = GlueStateView::default();
        state_view.registries.insert(registry_name, view);
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
        for r in view.registries.values() {
            let attrs = serde_json::json!({
                "id": r.registry_arn,
                "arn": r.registry_arn,
                "registry_name": r.registry_name,
                "description": r.description,
                "tags": r.tags,
                "tags_all": r.tags,
            });
            results.push(ExtractedResource {
                name: r.registry_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_glue_resource_policy
// ---------------------------------------------------------------------------

pub struct AwsGlueResourcePolicyConverter {
    service: Arc<GlueService>,
}

impl AwsGlueResourcePolicyConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlueResourcePolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_resource_policy"
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

impl AwsGlueResourcePolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glue_gen::ResourcePolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_glue_resource_policy", e))?;

        let policy_in_json = model.policy_in_json.clone();
        // A short opaque hash placeholder; real Glue returns an opaque token.
        let policy_hash = format!("{:x}", djb2_hash(&policy_in_json));
        let now = Utc::now().to_rfc3339();
        let view = ResourcePolicyView {
            policy_in_json,
            policy_hash,
            create_time: now.clone(),
            update_time: now,
        };

        let state_view = GlueStateView {
            resource_policy: Some(view),
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
        if let Some(p) = &view.resource_policy {
            let attrs = serde_json::json!({
                "id": ctx.default_account_id,
                "policy": p.policy_in_json,
                "policy_hash": p.policy_hash,
                "create_time": p.create_time,
                "update_time": p.update_time,
            });
            results.push(ExtractedResource {
                name: ctx.default_account_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// A trivial djb2 hash so we can produce a deterministic policy_hash
// stand-in without pulling in a crypto dependency. The real Glue API returns
// an opaque token; round-tripping does not need cryptographic strength.
fn djb2_hash(s: &str) -> u64 {
    let mut h: u64 = 5381;
    for b in s.bytes() {
        h = h.wrapping_mul(33).wrapping_add(b as u64);
    }
    h
}

// ---------------------------------------------------------------------------
// aws_glue_schema
// ---------------------------------------------------------------------------

pub struct AwsGlueSchemaConverter {
    service: Arc<GlueService>,
}

impl AwsGlueSchemaConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlueSchemaConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_schema"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_glue_registry"]
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

impl AwsGlueSchemaConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glue_gen::SchemaTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_glue_schema", e))?;

        let attrs = &instance.attributes;
        let schema_name = model.schema_name.clone();
        let registry_arn = model.registry_arn.unwrap_or_default();
        let registry_name = registry_arn
            .rsplit('/')
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or("default-registry")
            .to_string();
        let data_format = model.data_format.unwrap_or_else(|| "AVRO".to_string());
        let compatibility = model.compatibility.unwrap_or_else(|| "NONE".to_string());
        let description = model.description.unwrap_or_default();

        let tags: HashMap<String, String> = attrs
            .get("tags_all")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let schema_arn = format!(
            "arn:aws:glue:{}:{}:schema/{}/{}",
            region, ctx.default_account_id, registry_name, schema_name
        );

        let now = Utc::now().to_rfc3339();
        let view = SchemaView {
            schema_name: schema_name.clone(),
            schema_arn,
            registry_name,
            registry_arn,
            data_format,
            compatibility,
            description,
            schema_status: "AVAILABLE".to_string(),
            created_time: now.clone(),
            updated_time: now,
            latest_schema_version: 1,
            next_schema_version: 2,
            schema_checkpoint: 1,
            tags,
            versions: vec![],
        };

        let mut state_view = GlueStateView::default();
        state_view.schemas.insert(schema_name, view);
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
        for s in view.schemas.values() {
            let attrs = serde_json::json!({
                "id": s.schema_arn,
                "arn": s.schema_arn,
                "schema_name": s.schema_name,
                "registry_name": s.registry_name,
                "registry_arn": s.registry_arn,
                "data_format": s.data_format,
                "compatibility": s.compatibility,
                "description": s.description,
                "schema_status": s.schema_status,
                "latest_schema_version": s.latest_schema_version,
                "next_schema_version": s.next_schema_version,
                "schema_checkpoint": s.schema_checkpoint,
                "tags": s.tags,
                "tags_all": s.tags,
            });
            results.push(ExtractedResource {
                name: s.schema_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_glue_security_configuration
// ---------------------------------------------------------------------------

pub struct AwsGlueSecurityConfigurationConverter {
    service: Arc<GlueService>,
}

impl AwsGlueSecurityConfigurationConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlueSecurityConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_security_configuration"
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

impl AwsGlueSecurityConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glue_gen::SecurityConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_glue_security_configuration", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let encryption_configuration = attrs.get("encryption_configuration").cloned();

        let now = Utc::now().to_rfc3339();
        let view = SecurityConfigurationView {
            name: name.clone(),
            created_time_stamp: now,
            encryption_configuration,
        };

        let mut state_view = GlueStateView::default();
        state_view.security_configurations.insert(name, view);
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
        for s in view.security_configurations.values() {
            let attrs = serde_json::json!({
                "id": s.name,
                "name": s.name,
                "created_time_stamp": s.created_time_stamp,
                "encryption_configuration": s.encryption_configuration,
            });
            results.push(ExtractedResource {
                name: s.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_glue_trigger
// ---------------------------------------------------------------------------

pub struct AwsGlueTriggerConverter {
    service: Arc<GlueService>,
}

impl AwsGlueTriggerConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlueTriggerConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_trigger"
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

impl AwsGlueTriggerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glue_gen::TriggerTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_glue_trigger", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let trigger_type = model.trigger_type.clone();
        let description = model.description.unwrap_or_default();
        let schedule = model.schedule;
        let workflow_name = model.workflow_name;
        let state = model.state.unwrap_or_else(|| "CREATED".to_string());

        let actions = attrs.get("actions").cloned();
        let predicate = attrs.get("predicate").cloned();

        let view = TriggerView {
            name: name.clone(),
            trigger_type,
            state,
            description,
            schedule,
            actions,
            predicate,
            workflow_name,
        };

        let mut state_view = GlueStateView::default();
        state_view.triggers.insert(name, view);
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
        for t in view.triggers.values() {
            let arn = format!(
                "arn:aws:glue:{}:{}:trigger/{}",
                ctx.default_region, ctx.default_account_id, t.name
            );
            let attrs = serde_json::json!({
                "id": t.name,
                "name": t.name,
                "arn": arn,
                "type": t.trigger_type,
                "state": t.state,
                "description": t.description,
                "schedule": t.schedule,
                "workflow_name": t.workflow_name,
                "actions": t.actions,
                "predicate": t.predicate,
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: t.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_glue_workflow
// ---------------------------------------------------------------------------

pub struct AwsGlueWorkflowConverter {
    service: Arc<GlueService>,
}

impl AwsGlueWorkflowConverter {
    pub fn new(service: Arc<GlueService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsGlueWorkflowConverter {
    fn resource_type(&self) -> &str {
        "aws_glue_workflow"
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

impl AwsGlueWorkflowConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: glue_gen::WorkflowTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_glue_workflow", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let description = model.description.unwrap_or_default();
        let max_concurrent_runs = if model.max_concurrent_runs > 0 {
            Some(model.max_concurrent_runs as i32)
        } else {
            None
        };

        let default_run_properties: HashMap<String, String> = attrs
            .get("default_run_properties")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let now = Utc::now().to_rfc3339();
        let view = WorkflowView {
            name: name.clone(),
            description,
            default_run_properties,
            created_on: now.clone(),
            last_modified_on: now,
            max_concurrent_runs,
            runs: vec![],
        };

        let mut state_view = GlueStateView::default();
        state_view.workflows.insert(name, view);
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
        for w in view.workflows.values() {
            let arn = format!(
                "arn:aws:glue:{}:{}:workflow/{}",
                ctx.default_region, ctx.default_account_id, w.name
            );
            let attrs = serde_json::json!({
                "id": w.name,
                "name": w.name,
                "arn": arn,
                "description": w.description,
                "default_run_properties": w.default_run_properties,
                "max_concurrent_runs": w.max_concurrent_runs,
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: w.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
