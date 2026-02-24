//! Terraform converter for Timestream Write resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;
use winterbaume_timestreamwrite::TimestreamWriteService;
use winterbaume_timestreamwrite::views::{DatabaseView, TableView, TimestreamWriteStateView};

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_i64, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_timestreamwrite_database
// ---------------------------------------------------------------------------

/// Converts `aws_timestreamwrite_database` Terraform resources to/from Timestream Write state.
pub struct AwsTimestreamwriteDatabaseConverter {
    service: Arc<TimestreamWriteService>,
}

impl AwsTimestreamwriteDatabaseConverter {
    pub fn new(service: Arc<TimestreamWriteService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsTimestreamwriteDatabaseConverter {
    fn resource_type(&self) -> &str {
        "aws_timestreamwrite_database"
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

impl AwsTimestreamwriteDatabaseConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let database_name = require_str(attrs, "database_name", "aws_timestreamwrite_database")?;
        let region = extract_region(attrs, &ctx.default_region);
        let kms_key_id = optional_str(attrs, "kms_key_id");

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:timestream:{}:{}:database/{}",
                region, ctx.default_account_id, database_name
            )
        });

        let now = chrono::Utc::now().to_rfc3339();
        let db_view = DatabaseView {
            database_name: database_name.to_string(),
            arn,
            kms_key_id,
            table_count: optional_i64(attrs, "table_count").unwrap_or(0),
            creation_time: now.clone(),
            last_updated_time: now,
            tags: extract_tags(attrs),
        };

        let mut state_view = TimestreamWriteStateView {
            databases: HashMap::new(),
            tables: HashMap::new(),
            batch_load_tasks: HashMap::new(),
        };
        state_view
            .databases
            .insert(database_name.to_string(), db_view);
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
                "id": db.database_name,
                "database_name": db.database_name,
                "arn": db.arn,
                "kms_key_id": db.kms_key_id,
                "table_count": db.table_count,
                "tags": db.tags,
            });
            results.push(ExtractedResource {
                name: db.database_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_timestreamwrite_table
// ---------------------------------------------------------------------------

/// Converts `aws_timestreamwrite_table` Terraform resources to/from Timestream Write state.
pub struct AwsTimestreamwriteTableConverter {
    service: Arc<TimestreamWriteService>,
}

impl AwsTimestreamwriteTableConverter {
    pub fn new(service: Arc<TimestreamWriteService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsTimestreamwriteTableConverter {
    fn resource_type(&self) -> &str {
        "aws_timestreamwrite_table"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_timestreamwrite_database"]
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

impl AwsTimestreamwriteTableConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let table_name = require_str(attrs, "table_name", "aws_timestreamwrite_table")?;
        let database_name = require_str(attrs, "database_name", "aws_timestreamwrite_table")?;
        let region = extract_region(attrs, &ctx.default_region);

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:timestream:{}:{}:database/{}/table/{}",
                region, ctx.default_account_id, database_name, table_name
            )
        });

        // Parse retention_properties block
        let rp = attrs.get("retention_properties");
        let rp_obj = rp.and_then(|v| v.as_array().and_then(|a| a.first()).or(Some(v)));
        let memory_hours = rp_obj
            .and_then(|o| {
                o.get("memory_store_retention_period_in_hours")
                    .and_then(|v| v.as_i64())
            })
            .unwrap_or(6);
        let magnetic_days = rp_obj
            .and_then(|o| {
                o.get("magnetic_store_retention_period_in_days")
                    .and_then(|v| v.as_i64())
            })
            .unwrap_or(73000);

        // Parse magnetic_store_write_properties block
        let mswp = attrs.get("magnetic_store_write_properties");
        let mswp_obj = mswp.and_then(|v| v.as_array().and_then(|a| a.first()).or(Some(v)));
        let enable_magnetic = mswp_obj
            .and_then(|o| {
                o.get("enable_magnetic_store_writes")
                    .and_then(|v| v.as_bool())
            })
            .unwrap_or(false);

        let now = chrono::Utc::now().to_rfc3339();
        let tbl_view = TableView {
            table_name: table_name.to_string(),
            database_name: database_name.to_string(),
            arn,
            table_status: "ACTIVE".to_string(),
            memory_store_retention_period_in_hours: memory_hours,
            magnetic_store_retention_period_in_days: magnetic_days,
            enable_magnetic_store_writes: enable_magnetic,
            creation_time: now.clone(),
            last_updated_time: now,
            tags: extract_tags(attrs),
        };

        let key = format!("{}\x1f{}", database_name, table_name);
        let mut state_view = TimestreamWriteStateView {
            databases: HashMap::new(),
            tables: HashMap::new(),
            batch_load_tasks: HashMap::new(),
        };
        state_view.tables.insert(key, tbl_view);
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
            let attrs = serde_json::json!({
                "id": format!("{}:{}", tbl.database_name, tbl.table_name),
                "table_name": tbl.table_name,
                "database_name": tbl.database_name,
                "arn": tbl.arn,
                "retention_properties": [{
                    "memory_store_retention_period_in_hours": tbl.memory_store_retention_period_in_hours,
                    "magnetic_store_retention_period_in_days": tbl.magnetic_store_retention_period_in_days,
                }],
                "magnetic_store_write_properties": [{
                    "enable_magnetic_store_writes": tbl.enable_magnetic_store_writes,
                }],
                "tags": tbl.tags,
            });
            results.push(ExtractedResource {
                name: format!("{}-{}", tbl.database_name, tbl.table_name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
