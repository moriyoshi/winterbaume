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
use winterbaume_dynamodb::types::AttributeValue;
use winterbaume_dynamodb::views::{
    AttributeDefinitionView, ContributorInsightsConfigView, DynamoDbTagView, DynamodbStateView,
    GlobalTableInfoView, KeySchemaElementView, KinesisStreamingDestinationView,
    ProvisionedThroughputView, ResourcePolicyView, SecondaryIndexView, TableStateView,
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

/// Resolve a table by `resource_arn`: try matching the recorded `arn` on
/// each table, or fall back to interpreting the trailing `table/<name>`
/// segment of the ARN as a table name.
fn resolve_table_name_by_arn(view: &DynamodbStateView, resource_arn: &str) -> Option<String> {
    if let Some(t) = view.tables.values().find(|t| t.arn == resource_arn) {
        return Some(t.name.clone());
    }
    // Fall back to "table/<name>" suffix parsing.
    resource_arn
        .split_once(":table/")
        .map(|(_, rest)| rest.split('/').next().unwrap_or("").to_string())
        .filter(|s| !s.is_empty() && view.tables.contains_key(s))
}

// ---------------------------------------------------------------------------
// aws_dynamodb_contributor_insights
// ---------------------------------------------------------------------------

/// Converts `aws_dynamodb_contributor_insights` Terraform resources.
pub struct AwsDynamodbContributorInsightsConverter {
    service: Arc<DynamoDbService>,
}

impl AwsDynamodbContributorInsightsConverter {
    pub fn new(service: Arc<DynamoDbService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDynamodbContributorInsightsConverter {
    fn resource_type(&self) -> &str {
        "aws_dynamodb_contributor_insights"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_dynamodb_table"]
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

impl AwsDynamodbContributorInsightsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: dynamodb_gen::ContributorInsightsTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_dynamodb_contributor_insights", e))?;

        let key = match &model.index_name {
            Some(idx) if !idx.is_empty() => format!("{}/{}", model.table_name, idx),
            _ => model.table_name.clone(),
        };

        let view = ContributorInsightsConfigView {
            table_name: model.table_name,
            index_name: model.index_name.filter(|s| !s.is_empty()),
            status: "ENABLED".to_string(),
            last_update_date_time: Utc::now().to_rfc3339(),
        };

        let mut state_view = DynamodbStateView::default();
        state_view.contributor_insights.insert(key, view);
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
        for config in view.contributor_insights.values() {
            let id = match &config.index_name {
                Some(idx) => format!("{}/{}", config.table_name, idx),
                None => config.table_name.clone(),
            };
            let attrs = serde_json::json!({
                "id": id,
                "table_name": config.table_name,
                "index_name": config.index_name,
            });
            results.push(ExtractedResource {
                name: id,
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_dynamodb_global_table
// ---------------------------------------------------------------------------

/// Converts `aws_dynamodb_global_table` Terraform resources.
pub struct AwsDynamodbGlobalTableConverter {
    service: Arc<DynamoDbService>,
}

impl AwsDynamodbGlobalTableConverter {
    pub fn new(service: Arc<DynamoDbService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDynamodbGlobalTableConverter {
    fn resource_type(&self) -> &str {
        "aws_dynamodb_global_table"
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

impl AwsDynamodbGlobalTableConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: dynamodb_gen::GlobalTableTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_dynamodb_global_table", e))?;

        let attrs = &instance.attributes;

        // `replica` is a list of nested blocks: `{ region_name = "us-east-1" }`.
        let replication_group: Vec<String> = attrs
            .get("replica")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        item.as_object()
                            .and_then(|obj| obj.get("region_name"))
                            .and_then(|v| v.as_str())
                            .map(String::from)
                    })
                    .collect()
            })
            .unwrap_or_default();

        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:dynamodb::{}:global-table/{}",
                ctx.default_account_id, model.name
            )
        });

        let view = GlobalTableInfoView {
            global_table_name: model.name.clone(),
            global_table_arn: arn,
            global_table_status: "ACTIVE".to_string(),
            creation_date_time: Utc::now().to_rfc3339(),
            replication_group,
        };

        let mut state_view = DynamodbStateView::default();
        state_view.global_tables.insert(model.name, view);
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
        for gt in view.global_tables.values() {
            let replicas: Vec<serde_json::Value> = gt
                .replication_group
                .iter()
                .map(|r| serde_json::json!({ "region_name": r }))
                .collect();
            let attrs = serde_json::json!({
                "id": gt.global_table_name,
                "name": gt.global_table_name,
                "arn": gt.global_table_arn,
                "replica": replicas,
            });
            results.push(ExtractedResource {
                name: gt.global_table_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_dynamodb_kinesis_streaming_destination
// ---------------------------------------------------------------------------

/// Converts `aws_dynamodb_kinesis_streaming_destination` Terraform resources.
pub struct AwsDynamodbKinesisStreamingDestinationConverter {
    service: Arc<DynamoDbService>,
}

impl AwsDynamodbKinesisStreamingDestinationConverter {
    pub fn new(service: Arc<DynamoDbService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDynamodbKinesisStreamingDestinationConverter {
    fn resource_type(&self) -> &str {
        "aws_dynamodb_kinesis_streaming_destination"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_dynamodb_table"]
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

impl AwsDynamodbKinesisStreamingDestinationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: dynamodb_gen::KinesisStreamingDestinationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_dynamodb_kinesis_streaming_destination", e)
            })?;

        let dest = KinesisStreamingDestinationView {
            stream_arn: model.stream_arn.clone(),
            destination_status: "ACTIVE".to_string(),
            approximate_creation_date_time_precision: model
                .approximate_creation_date_time_precision,
        };

        let mut state_view = DynamodbStateView::default();
        let mut dests = HashMap::new();
        dests.insert(model.stream_arn, dest);
        state_view
            .kinesis_destinations
            .insert(model.table_name, dests);
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
        for (table_name, dests) in &view.kinesis_destinations {
            for dest in dests.values() {
                let id = format!("{}|{}", table_name, dest.stream_arn);
                let attrs = serde_json::json!({
                    "id": id,
                    "table_name": table_name,
                    "stream_arn": dest.stream_arn,
                    "approximate_creation_date_time_precision":
                        dest.approximate_creation_date_time_precision,
                });
                results.push(ExtractedResource {
                    name: id,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_dynamodb_resource_policy
// ---------------------------------------------------------------------------

/// Converts `aws_dynamodb_resource_policy` Terraform resources.
pub struct AwsDynamodbResourcePolicyConverter {
    service: Arc<DynamoDbService>,
}

impl AwsDynamodbResourcePolicyConverter {
    pub fn new(service: Arc<DynamoDbService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDynamodbResourcePolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_dynamodb_resource_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_dynamodb_table"]
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

impl AwsDynamodbResourcePolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: dynamodb_gen::ResourcePolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_dynamodb_resource_policy", e))?;

        let revision_id = model
            .revision_id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let policy_view = ResourcePolicyView {
            policy: model.policy,
            revision_id,
        };

        let mut state_view = DynamodbStateView::default();
        state_view
            .resource_policies
            .insert(model.resource_arn, policy_view);
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
        for (arn, policy) in &view.resource_policies {
            let attrs = serde_json::json!({
                "id": arn,
                "resource_arn": arn,
                "policy": policy.policy,
                "revision_id": policy.revision_id,
            });
            results.push(ExtractedResource {
                name: arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_dynamodb_table_export
// ---------------------------------------------------------------------------

/// Converts `aws_dynamodb_table_export` Terraform resources.
///
/// The DynamoDB state-view layer does not currently expose the
/// `ExportInfo` records that live on `DynamoDbState.exports`. Injection
/// is therefore best-effort: we validate the model and emit a warning
/// so callers know the export was acknowledged but not persisted.
/// Extraction returns no results for the same reason.
pub struct AwsDynamodbTableExportConverter {
    #[allow(dead_code)]
    service: Arc<DynamoDbService>,
}

impl AwsDynamodbTableExportConverter {
    pub fn new(service: Arc<DynamoDbService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDynamodbTableExportConverter {
    fn resource_type(&self) -> &str {
        "aws_dynamodb_table_export"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_dynamodb_table"]
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Exports live on DynamoDbState.exports but are not part of the
        // serializable state view; nothing to extract.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDynamodbTableExportConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: dynamodb_gen::TableExportTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_dynamodb_table_export", e))?;

        let warnings = vec![format!(
            "aws_dynamodb_table_export: state-view layer does not expose exports; \
             export of table '{}' to S3 bucket '{}' was acknowledged but not persisted",
            model.table_arn,
            model.s3_bucket.as_deref().unwrap_or("(unset)"),
        )];

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_dynamodb_table_item
// ---------------------------------------------------------------------------

/// Converts `aws_dynamodb_table_item` Terraform resources.
///
/// The `item` attribute is a JSON-encoded string in DynamoDB
/// AttributeValue format (e.g. `{"id": {"S": "..."}}`). It is parsed
/// into the typed in-memory `Item` and inserted into the owning
/// `TableStateView.items` map.
pub struct AwsDynamodbTableItemConverter {
    service: Arc<DynamoDbService>,
}

impl AwsDynamodbTableItemConverter {
    pub fn new(service: Arc<DynamoDbService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDynamodbTableItemConverter {
    fn resource_type(&self) -> &str {
        "aws_dynamodb_table_item"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_dynamodb_table"]
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

impl AwsDynamodbTableItemConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: dynamodb_gen::TableItemTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_dynamodb_table_item", e))?;

        let mut warnings = vec![];

        // Parse the `item` JSON string into a typed Item.
        let item: HashMap<String, AttributeValue> = match serde_json::from_str(&model.item) {
            Ok(i) => i,
            Err(e) => {
                warnings.push(format!(
                    "aws_dynamodb_table_item: failed to parse `item` JSON for table '{}': {}",
                    model.table_name, e
                ));
                return Ok(ConversionResult { region, warnings });
            }
        };

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        let Some(table) = snapshot.tables.get_mut(&model.table_name) else {
            warnings.push(format!(
                "aws_dynamodb_table_item: table '{}' not found; item not inserted",
                model.table_name
            ));
            return Ok(ConversionResult { region, warnings });
        };

        let hash_value = match item.get(&model.hash_key) {
            Some(v) => serialize_attr_for_key(v),
            None => {
                warnings.push(format!(
                    "aws_dynamodb_table_item: item is missing hash key '{}'",
                    model.hash_key
                ));
                return Ok(ConversionResult { region, warnings });
            }
        };

        let range_value = match model.range_key.as_deref() {
            Some(rk) if !rk.is_empty() => match item.get(rk) {
                Some(v) => serialize_attr_for_key(v),
                None => {
                    warnings.push(format!(
                        "aws_dynamodb_table_item: item is missing range key '{}'",
                        rk
                    ));
                    return Ok(ConversionResult { region, warnings });
                }
            },
            _ => String::new(),
        };

        table
            .items
            .entry(hash_value)
            .or_default()
            .insert(range_value, item);

        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
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
            for range_map in table.items.values() {
                for item in range_map.values() {
                    let item_json = match serde_json::to_string(item) {
                        Ok(s) => s,
                        Err(_) => continue,
                    };
                    let hash_key_val = item
                        .get(&table.hash_key_attr)
                        .map(serialize_attr_for_key)
                        .unwrap_or_default();
                    let range_key_val = match &table.range_key_attr {
                        Some(rk) => item.get(rk).map(serialize_attr_for_key).unwrap_or_default(),
                        None => String::new(),
                    };
                    let id = if range_key_val.is_empty() {
                        format!("{}|{}|{}", table.name, table.hash_key_attr, hash_key_val)
                    } else {
                        format!(
                            "{}|{}|{}|{}|{}",
                            table.name,
                            table.hash_key_attr,
                            hash_key_val,
                            table.range_key_attr.as_deref().unwrap_or(""),
                            range_key_val,
                        )
                    };
                    let attrs = serde_json::json!({
                        "id": id,
                        "table_name": table.name,
                        "hash_key": table.hash_key_attr,
                        "range_key": table.range_key_attr,
                        "item": item_json,
                    });
                    results.push(ExtractedResource {
                        name: id,
                        account_id: ctx.default_account_id.clone(),
                        region: ctx.default_region.clone(),
                        attributes: attrs,
                    });
                }
            }
        }
        Ok(results)
    }
}

/// Render a single DynamoDB AttributeValue as a stable string suitable
/// for use as a HashMap key in the in-memory item store. Mirrors the
/// internal `serialize_key_value` helper in `winterbaume_dynamodb::state`.
fn serialize_attr_for_key(value: &AttributeValue) -> String {
    match value {
        AttributeValue::S(s) => format!("S:{s}"),
        AttributeValue::N(n) => format!("N:{n}"),
        AttributeValue::B(b) => format!("B:{b}"),
        other => format!("{other:?}"),
    }
}

// ---------------------------------------------------------------------------
// aws_dynamodb_table_replica
// ---------------------------------------------------------------------------

/// Converts `aws_dynamodb_table_replica` Terraform resources.
///
/// The state-view layer keeps `replica` as a free-form `Vec<Value>` on
/// `TableStateView`; this converter appends a JSON blob describing the
/// replica region (extracted from the conversion context) into that
/// list on the owning table.
pub struct AwsDynamodbTableReplicaConverter {
    service: Arc<DynamoDbService>,
}

impl AwsDynamodbTableReplicaConverter {
    pub fn new(service: Arc<DynamoDbService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDynamodbTableReplicaConverter {
    fn resource_type(&self) -> &str {
        "aws_dynamodb_table_replica"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_dynamodb_table"]
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Replicas already round-trip as nested blocks under their
        // parent `aws_dynamodb_table`'s `replica` attribute.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDynamodbTableReplicaConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: dynamodb_gen::TableReplicaTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_dynamodb_table_replica", e))?;

        let mut warnings = vec![];

        // The global_table_arn points at the source table; resolve it to a
        // table name so we can add the replica entry to its view.
        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        let Some(table_name) = resolve_table_name_by_arn(&snapshot, &model.global_table_arn) else {
            warnings.push(format!(
                "aws_dynamodb_table_replica: source table for ARN '{}' not found; \
                 replica not recorded",
                model.global_table_arn
            ));
            return Ok(ConversionResult { region, warnings });
        };

        let replica_entry = serde_json::json!({
            "region_name": region,
            "kms_key_arn": model.kms_key_arn,
            "point_in_time_recovery": model.point_in_time_recovery,
            "table_class_override": model.table_class_override,
        });

        if let Some(table) = snapshot.tables.get_mut(&table_name) {
            table.replica.push(replica_entry);
        }

        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult { region, warnings })
    }
}

// ---------------------------------------------------------------------------
// aws_dynamodb_tag
// ---------------------------------------------------------------------------

/// Converts `aws_dynamodb_tag` Terraform resources.
///
/// Tags are keyed by `resource_arn` in the state view. When the
/// referenced ARN does not match any known resource a warning is
/// emitted and the tag is dropped, mirroring the `aws_ec2_tag`
/// pattern.
pub struct AwsDynamodbTagConverter {
    service: Arc<DynamoDbService>,
}

impl AwsDynamodbTagConverter {
    pub fn new(service: Arc<DynamoDbService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDynamodbTagConverter {
    fn resource_type(&self) -> &str {
        "aws_dynamodb_tag"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_dynamodb_table"]
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
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Tags already round-trip through the parent resource's `tags` map.
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsDynamodbTagConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: dynamodb_gen::TagTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_dynamodb_tag", e))?;

        let mut snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        let entry = snapshot.tags.entry(model.resource_arn.clone()).or_default();
        entry.retain(|t| t.key != model.key);
        entry.push(DynamoDbTagView {
            key: model.key,
            value: model.value,
        });

        self.service
            .restore(&ctx.default_account_id, &region, snapshot)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}
