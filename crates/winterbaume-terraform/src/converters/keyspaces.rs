//! Terraform converters for Amazon Keyspaces resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_keyspaces::KeyspacesService;
use winterbaume_keyspaces::views::{
    ClusteringKeyView, ColumnDefinitionView, KeyspaceView, KeyspacesStateView,
    SchemaDefinitionView, TableView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_keyspaces_keyspace
// ---------------------------------------------------------------------------

pub struct AwsKeyspacesKeyspaceConverter {
    service: Arc<KeyspacesService>,
}

impl AwsKeyspacesKeyspaceConverter {
    pub fn new(service: Arc<KeyspacesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKeyspacesKeyspaceConverter {
    fn resource_type(&self) -> &str {
        "aws_keyspaces_keyspace"
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

impl AwsKeyspacesKeyspaceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_keyspaces_keyspace")?.to_string();

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:cassandra:{region}:{account}:/keyspace/{name}/",
                account = ctx.default_account_id
            )
        });

        let mut tags = HashMap::new();
        if let Some(tags_all) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in tags_all {
                if let Some(val) = v.as_str() {
                    tags.insert(k.clone(), val.to_string());
                }
            }
        }

        let view = KeyspacesStateView {
            keyspaces: {
                let mut m = HashMap::new();
                m.insert(
                    name.clone(),
                    KeyspaceView {
                        name: name.clone(),
                        arn,
                        replication_strategy: "SINGLE_REGION".to_string(),
                        replication_regions: vec![],
                        tags,
                        creation_timestamp: None,
                        status: "ACTIVE".to_string(),
                    },
                );
                m
            },
            ..Default::default()
        };

        self.service
            .merge(&ctx.default_account_id, &region, view)
            .await
            .map_err(ConversionError::StateView)?;

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

        let mut results = Vec::new();
        for ks in view.keyspaces.values() {
            let mut attrs = serde_json::Map::new();
            attrs.insert("name".into(), serde_json::Value::String(ks.name.clone()));
            attrs.insert("arn".into(), serde_json::Value::String(ks.arn.clone()));

            if !ks.tags.is_empty() {
                let tags_obj: serde_json::Map<String, serde_json::Value> = ks
                    .tags
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                    .collect();
                attrs.insert("tags".into(), serde_json::Value::Object(tags_obj.clone()));
                attrs.insert("tags_all".into(), serde_json::Value::Object(tags_obj));
            }

            results.push(ExtractedResource {
                name: ks.name.replace('-', "_"),
                attributes: serde_json::Value::Object(attrs),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
            });
        }

        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_keyspaces_table
// ---------------------------------------------------------------------------

pub struct AwsKeyspacesTableConverter {
    service: Arc<KeyspacesService>,
}

impl AwsKeyspacesTableConverter {
    pub fn new(service: Arc<KeyspacesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKeyspacesTableConverter {
    fn resource_type(&self) -> &str {
        "aws_keyspaces_table"
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

impl AwsKeyspacesTableConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let keyspace_name = require_str(attrs, "keyspace_name", "aws_keyspaces_table")?.to_string();
        let table_name = require_str(attrs, "table_name", "aws_keyspaces_table")?.to_string();

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:cassandra:{region}:{account}:/keyspace/{keyspace_name}/table/{table_name}",
                account = ctx.default_account_id
            )
        });

        let schema = parse_schema_definition(attrs);

        let mut tags = HashMap::new();
        if let Some(tags_all) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in tags_all {
                if let Some(val) = v.as_str() {
                    tags.insert(k.clone(), val.to_string());
                }
            }
        }

        let table_key = format!("{keyspace_name}/{table_name}");
        let view = KeyspacesStateView {
            tables: {
                let mut m = HashMap::new();
                m.insert(
                    table_key,
                    TableView {
                        keyspace_name,
                        table_name,
                        arn,
                        schema_definition: schema,
                        capacity_mode: "PAY_PER_REQUEST".to_string(),
                        read_capacity_units: None,
                        write_capacity_units: None,
                        encryption_type: "AWS_OWNED_KMS_KEY".to_string(),
                        kms_key_identifier: None,
                        point_in_time_recovery_enabled: false,
                        ttl_status: "ENABLED".to_string(),
                        default_time_to_live: None,
                        comment: String::new(),
                        client_side_timestamps_enabled: false,
                        tags,
                        creation_timestamp: None,
                        status: "ACTIVE".to_string(),
                    },
                );
                m
            },
            ..Default::default()
        };

        self.service
            .merge(&ctx.default_account_id, &region, view)
            .await
            .map_err(ConversionError::StateView)?;

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

        let mut results = Vec::new();
        for table in view.tables.values() {
            let mut attrs = serde_json::Map::new();
            attrs.insert(
                "keyspace_name".into(),
                serde_json::Value::String(table.keyspace_name.clone()),
            );
            attrs.insert(
                "table_name".into(),
                serde_json::Value::String(table.table_name.clone()),
            );
            attrs.insert("arn".into(), serde_json::Value::String(table.arn.clone()));

            let local = format!("{}_{}", table.keyspace_name, table.table_name).replace('-', "_");
            results.push(ExtractedResource {
                name: local,
                attributes: serde_json::Value::Object(attrs),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
            });
        }

        Ok(results)
    }
}

fn parse_schema_definition(attrs: &serde_json::Value) -> SchemaDefinitionView {
    let schema_block = attrs
        .get("schema_definition")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first());

    let all_columns = schema_block
        .and_then(|s| s.get("column").or_else(|| s.get("columns")))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|c| {
                    let name = c.get("name")?.as_str()?.to_string();
                    let col_type = c.get("type")?.as_str()?.to_string();
                    Some(ColumnDefinitionView {
                        name,
                        column_type: col_type,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let partition_keys = schema_block
        .and_then(|s| s.get("partition_key"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|p| p.get("name")?.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let clustering_keys = schema_block
        .and_then(|s| s.get("clustering_key"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|c| {
                    let name = c.get("name")?.as_str()?.to_string();
                    let order_by = c
                        .get("order_by")
                        .and_then(|v| v.as_str())
                        .unwrap_or("ASC")
                        .to_string();
                    Some(ClusteringKeyView { name, order_by })
                })
                .collect()
        })
        .unwrap_or_default();

    let static_columns = schema_block
        .and_then(|s| s.get("static_column"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|s| s.get("name")?.as_str().map(|n| n.to_string()))
                .collect()
        })
        .unwrap_or_default();

    SchemaDefinitionView {
        all_columns,
        partition_keys,
        clustering_keys,
        static_columns,
    }
}
