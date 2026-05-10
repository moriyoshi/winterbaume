//! Terraform converter for `aws_dynamodb_table` resources.
//!
//! `TableTfModel` is generated from `specs/dynamodb.toml`. The ARN
//! template (with `id` fallback), the `attribute` /
//! `global_secondary_index` / `local_secondary_index` / `replica`
//! nested-block parsing, the merged tags_all+tags collection, the
//! `import_table` / `on_demand_throughput` raw-JSON capture, and the
//! `creation_date_time` / `table_status` constants are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_dynamodb::DynamoDbService;
use winterbaume_dynamodb::views::{
    AttributeDefinitionView, DynamoDbTagView, DynamodbStateView, KeySchemaElementView,
    ProvisionedThroughputView, SecondaryIndexView, TableStateView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::dynamodb as dynamodb_gen;
use crate::util::{classify_deserialize_error, extract_region};

/// Converts `aws_dynamodb_table` Terraform resources to/from DynamoDB state.
pub struct AwsDynamodbTableConverter {
    service: Arc<DynamoDbService>,
}

impl AwsDynamodbTableConverter {
    pub fn new(service: Arc<DynamoDbService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDynamodbTableConverter {
    fn resource_type(&self) -> &str {
        "aws_dynamodb_table"
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

impl AwsDynamodbTableConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: dynamodb_gen::TableTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_dynamodb_table", e))?;

        let attrs = &instance.attributes;

        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:dynamodb:{}:{}:table/{}",
                region, ctx.default_account_id, model.name
            )
        });

        let billing_mode = model
            .billing_mode
            .unwrap_or_else(|| "PAY_PER_REQUEST".to_string());

        // Attribute definitions (nested-block array).
        let attribute_defs: Vec<AttributeDefinitionView> = attrs
            .get("attribute")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        let attr_name = item.get("name")?.as_str()?.to_string();
                        let attr_type = item.get("type")?.as_str()?.to_string();
                        Some(AttributeDefinitionView {
                            attribute_name: attr_name,
                            attribute_type: attr_type,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Determine hash key type
        let hash_key_type = attribute_defs
            .iter()
            .find(|a| a.attribute_name == model.hash_key)
            .map(|a| a.attribute_type.clone())
            .unwrap_or_else(|| "S".to_string());

        let range_key_type = model.range_key.as_deref().and_then(|rk| {
            attribute_defs
                .iter()
                .find(|a| a.attribute_name == rk)
                .map(|a| a.attribute_type.clone())
        });

        // Key schema
        let mut key_schema = vec![KeySchemaElementView {
            attribute_name: model.hash_key.clone(),
            key_type: "HASH".to_string(),
        }];
        if let Some(rk) = &model.range_key {
            key_schema.push(KeySchemaElementView {
                attribute_name: rk.clone(),
                key_type: "RANGE".to_string(),
            });
        }

        let provisioned_throughput = if billing_mode == "PROVISIONED" {
            Some(ProvisionedThroughputView {
                read_capacity_units: model.read_capacity,
                write_capacity_units: model.write_capacity,
            })
        } else {
            None
        };

        // Tags: merge tags_all (lower precedence) + tags (higher).
        let mut tags_map: HashMap<String, String> = HashMap::new();
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags_map.insert(k.clone(), s.to_string());
                }
            }
        }
        if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags_map.insert(k.clone(), s.to_string());
                }
            }
        }
        let tags: Vec<DynamoDbTagView> = tags_map
            .into_iter()
            .map(|(k, v)| DynamoDbTagView { key: k, value: v })
            .collect();

        let global_secondary_index = parse_secondary_index_blocks(attrs, "global_secondary_index");
        let local_secondary_index = parse_secondary_index_blocks(attrs, "local_secondary_index");
        let replica: Vec<serde_json::Value> = attrs
            .get("replica")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let import_table = attrs
            .get("import_table")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let on_demand_throughput = attrs
            .get("on_demand_throughput")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });

        let table_view = TableStateView {
            name: model.name.clone(),
            arn: arn.clone(),
            key_schema,
            attribute_definitions: attribute_defs,
            billing_mode,
            provisioned_throughput,
            creation_date_time: Utc::now().to_rfc3339(),
            table_status: "ACTIVE".to_string(),
            hash_key_attr: model.hash_key,
            hash_key_type,
            range_key_attr: model.range_key,
            range_key_type,
            items: HashMap::new(),
            stream_enabled: model.stream_enabled,
            stream_view_type: model.stream_view_type,
            latest_stream_arn: None,
            latest_stream_label: None,
            global_secondary_index,
            local_secondary_index,
            replica,
            import_table,
            on_demand_throughput,
        };

        let mut state_view = DynamodbStateView {
            ..Default::default()
        };
        state_view.tables.insert(model.name, table_view);
        if !tags.is_empty() {
            state_view.tags.insert(arn, tags);
        }
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
        for table in view.tables.values() {
            let tags: HashMap<String, String> = view
                .tags
                .get(&table.arn)
                .map(|tag_list| {
                    tag_list
                        .iter()
                        .map(|t| (t.key.clone(), t.value.clone()))
                        .collect()
                })
                .unwrap_or_default();
            let (read_capacity, write_capacity) = table
                .provisioned_throughput
                .as_ref()
                .map(|pt| (pt.read_capacity_units, pt.write_capacity_units))
                .unwrap_or((0, 0));
            let attribute_defs_json: Vec<serde_json::Value> = table
                .attribute_definitions
                .iter()
                .map(|a| serde_json::json!({"name": a.attribute_name, "type": a.attribute_type}))
                .collect();
            let attrs = serde_json::json!({
                "id": table.name,
                "name": table.name,
                "arn": table.arn,
                "hash_key": table.hash_key_attr,
                "range_key": table.range_key_attr,
                "billing_mode": table.billing_mode,
                "status": table.table_status,
                "stream_enabled": table.stream_enabled,
                "stream_view_type": table.stream_view_type,
                "read_capacity": read_capacity,
                "write_capacity": write_capacity,
                "tags": tags,
                "tags_all": tags,
                "point_in_time_recovery": [{"enabled": false}],
                "ttl": [{"enabled": false, "attribute_name": ""}],
                "server_side_encryption": [{"enabled": false}],
                "table_class": "STANDARD",
                "attribute": attribute_defs_json,
                "global_secondary_index": table.global_secondary_index,
                "local_secondary_index": table.local_secondary_index,
                "replica": table.replica,
                "import_table": table.import_table,
                "on_demand_throughput": table.on_demand_throughput,
            });
            results.push(ExtractedResource {
                name: table.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

fn parse_secondary_index_blocks(attrs: &serde_json::Value, field: &str) -> Vec<SecondaryIndexView> {
    attrs
        .get(field)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    let obj = item.as_object()?;
                    let index_name = obj
                        .get("name")
                        .or_else(|| obj.get("index_name"))
                        .and_then(|v| v.as_str())?
                        .to_string();
                    let mut key_schema = vec![KeySchemaElementView {
                        attribute_name: obj.get("hash_key")?.as_str()?.to_string(),
                        key_type: "HASH".to_string(),
                    }];
                    if let Some(range_key) = obj.get("range_key").and_then(|v| v.as_str()) {
                        if !range_key.is_empty() {
                            key_schema.push(KeySchemaElementView {
                                attribute_name: range_key.to_string(),
                                key_type: "RANGE".to_string(),
                            });
                        }
                    }
                    let non_key_attributes = obj
                        .get("non_key_attributes")
                        .and_then(|v| v.as_array())
                        .map(|values| {
                            values
                                .iter()
                                .filter_map(|value| value.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default();
                    Some(SecondaryIndexView {
                        index_name,
                        key_schema,
                        projection_type: obj
                            .get("projection_type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("ALL")
                            .to_string(),
                        non_key_attributes,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}
