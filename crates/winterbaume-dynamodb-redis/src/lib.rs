//! Redis-backed [`DynamoDbBackend`] implementation.
//!
//! # Key schema
//!
//! All keys are namespaced by `{account_id}` and `{region}`:
//!
//! | Key | Type | Contents |
//! |-----|------|----------|
//! | `ddb:{acct}:{rgn}:tables` | Set | Table names |
//! | `ddb:{acct}:{rgn}:table:{name}` | String | JSON `StoredTable` |
//! | `ddb:{acct}:{rgn}:table:{name}:items` | Hash | `{b64(hash_val)}|{b64(range_val)}` → JSON Item |
//! | `ddb:{acct}:{rgn}:backup_ctr` | String | Monotonic counter for backup IDs |
//! | `ddb:{acct}:{rgn}:backups` | Set | Backup ARNs |
//! | `ddb:{acct}:{rgn}:backup:{arn_b64}` | String | JSON `StoredBackup` |
//! | `ddb:{acct}:{rgn}:tags:{arn_b64}` | Hash | tag_key → tag_value |
//! | `ddb:{acct}:{rgn}:ttl:{table}` | String | JSON `StoredTtl` |
//! | `ddb:{acct}:{rgn}:pitr:{table}` | String | JSON `StoredPitr` |
//! | `ddb:{acct}:{rgn}:policy:{arn_b64}` | String | JSON `StoredPolicy` |
//! | `ddb:{acct}:{rgn}:global_tables` | Set | Global table names |
//! | `ddb:{acct}:{rgn}:global_table:{name}` | String | JSON `StoredGlobalTable` |
//! | `ddb:{acct}:{rgn}:kinesis:{table}` | Hash | `stream_arn_b64` → JSON `StoredKinesisDestination` |
//! | `ddb:{acct}:{rgn}:ci` | Hash | `{table}|{index_or_empty}` → JSON `StoredContributorInsights` |
//! | `ddb:{acct}:{rgn}:exports` | Set | Export ARNs |
//! | `ddb:{acct}:{rgn}:export:{arn_b64}` | String | JSON `StoredExport` |
//! | `ddb:{acct}:{rgn}:export_ctr` | String | Monotonic counter for export IDs |

use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::Utc;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use winterbaume_dynamodb::state::{DynamoDbError, QueryResult};
use winterbaume_dynamodb::types::{StreamChangeRecord, StreamSummary, *};
use winterbaume_dynamodb::views::{
    AttributeDefinitionView, DynamoDbTagView, KeySchemaElementView, ProvisionedThroughputView,
    TableStateView, TtlConfigView,
};
use winterbaume_dynamodb::{DynamoDbBackend, DynamodbStateView, StateViewError};

// ---------------------------------------------------------------------------
// Error helpers
// ---------------------------------------------------------------------------

fn redis_err(e: redis::RedisError) -> DynamoDbError {
    DynamoDbError::InternalError(e.to_string())
}

fn json_err(e: serde_json::Error) -> DynamoDbError {
    DynamoDbError::InternalError(e.to_string())
}

fn table_not_found(name: &str) -> DynamoDbError {
    DynamoDbError::ResourceNotFound(name.to_string())
}

fn backup_not_found(arn: &str) -> DynamoDbError {
    DynamoDbError::BackupNotFound(arn.to_string())
}

fn global_table_not_found(name: &str) -> DynamoDbError {
    DynamoDbError::GlobalTableNotFound(name.to_string())
}

// ---------------------------------------------------------------------------
// Redis key helpers
// ---------------------------------------------------------------------------

fn k_tables(acct: &str, rgn: &str) -> String {
    format!("ddb:{acct}:{rgn}:tables")
}
fn k_table(acct: &str, rgn: &str, name: &str) -> String {
    format!("ddb:{acct}:{rgn}:table:{name}")
}
fn k_items(acct: &str, rgn: &str, name: &str) -> String {
    format!("ddb:{acct}:{rgn}:table:{name}:items")
}
fn k_backup_ctr(acct: &str, rgn: &str) -> String {
    format!("ddb:{acct}:{rgn}:backup_ctr")
}
fn k_backups(acct: &str, rgn: &str) -> String {
    format!("ddb:{acct}:{rgn}:backups")
}
fn k_backup(acct: &str, rgn: &str, arn_b64: &str) -> String {
    format!("ddb:{acct}:{rgn}:backup:{arn_b64}")
}
fn k_tags(acct: &str, rgn: &str, arn_b64: &str) -> String {
    format!("ddb:{acct}:{rgn}:tags:{arn_b64}")
}
fn k_ttl(acct: &str, rgn: &str, table: &str) -> String {
    format!("ddb:{acct}:{rgn}:ttl:{table}")
}
fn k_pitr(acct: &str, rgn: &str, table: &str) -> String {
    format!("ddb:{acct}:{rgn}:pitr:{table}")
}
fn k_policy(acct: &str, rgn: &str, arn_b64: &str) -> String {
    format!("ddb:{acct}:{rgn}:policy:{arn_b64}")
}
fn k_global_tables(acct: &str, rgn: &str) -> String {
    format!("ddb:{acct}:{rgn}:global_tables")
}
fn k_global_table(acct: &str, rgn: &str, name: &str) -> String {
    format!("ddb:{acct}:{rgn}:global_table:{name}")
}
fn k_kinesis(acct: &str, rgn: &str, table: &str) -> String {
    format!("ddb:{acct}:{rgn}:kinesis:{table}")
}
fn k_ci(acct: &str, rgn: &str) -> String {
    format!("ddb:{acct}:{rgn}:ci")
}
fn k_exports(acct: &str, rgn: &str) -> String {
    format!("ddb:{acct}:{rgn}:exports")
}
fn k_export(acct: &str, rgn: &str, arn_b64: &str) -> String {
    format!("ddb:{acct}:{rgn}:export:{arn_b64}")
}
fn k_export_ctr(acct: &str, rgn: &str) -> String {
    format!("ddb:{acct}:{rgn}:export_ctr")
}

// ---------------------------------------------------------------------------
// Item key encoding
// ---------------------------------------------------------------------------

fn b64(s: &str) -> String {
    URL_SAFE_NO_PAD.encode(s.as_bytes())
}

/// Encode a single key attribute value (an AttributeValue) to a base64 string.
fn encode_key_value(val: &AttributeValue) -> String {
    b64(&serde_json::to_string(val).unwrap_or_default())
}

/// Compare two optional `AttributeValue`s for query-result sort ordering.
///
/// `None` (item missing the range-key attribute) sorts after `Some` so that
/// well-formed items take precedence in the result page. Values compare by
/// [`compare_key_values`] ( numbers exactly, not as `f64` ); type mismatches
/// and unsupported types fall back to `Equal`, leaving the relative order of
/// affected items unspecified — DynamoDB only guarantees a consistent order
/// for items whose range key matches the table's declared scalar type.
fn compare_attr_values(
    a: Option<&AttributeValue>,
    b: Option<&AttributeValue>,
) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(av), Some(bv)) => compare_key_values(av, bv).unwrap_or(Ordering::Equal),
    }
}

/// Build the Redis hash field for an item, given the table's key schema.
fn item_field(item: &Item, hash_key: &str, range_key: Option<&str>) -> String {
    let hash_val = item
        .get(hash_key)
        .cloned()
        .unwrap_or(AttributeValue::Null(true));
    let range_val = range_key
        .and_then(|rk| item.get(rk).cloned())
        .unwrap_or(AttributeValue::Null(true));
    format!(
        "{}|{}",
        encode_key_value(&hash_val),
        encode_key_value(&range_val)
    )
}

/// Position of an item in a read: its sort-key value (the index sort key for
/// an index read, the table sort key for a base-table query, none for a
/// scan), with the table primary key as a tie-breaker. The tie-breaker makes
/// the order total, which index reads need because index keys are not
/// unique; `ExclusiveStartKey` carries the table primary key, so its position
/// is computed the same way.
struct ReadPosition {
    sort_value: Option<AttributeValue>,
    hash_value: Option<AttributeValue>,
    range_value: Option<AttributeValue>,
}

impl ReadPosition {
    fn of(item: &Item, table: &StoredTable, sort_attr: Option<&str>) -> Self {
        let value = |attr: Option<&str>| attr.and_then(|a| item.get(a)).cloned();
        Self {
            sort_value: value(sort_attr),
            hash_value: value(Some(&table.hash_key_attr)),
            range_value: value(table.range_key_attr.as_deref()),
        }
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        compare_attr_values(self.sort_value.as_ref(), other.sort_value.as_ref())
            .then_with(|| compare_attr_values(self.hash_value.as_ref(), other.hash_value.as_ref()))
            .then_with(|| {
                compare_attr_values(self.range_value.as_ref(), other.range_value.as_ref())
            })
    }
}

/// Resolve the `(hash, range)` key attribute names of a secondary index.
fn index_key_attrs<'a>(
    table: &'a StoredTable,
    index_name: &str,
) -> Result<(&'a str, Option<&'a str>), DynamoDbError> {
    let index = table
        .global_secondary_indexes
        .iter()
        .chain(table.local_secondary_indexes.iter())
        .find(|index| index.index_name == index_name)
        .ok_or_else(|| {
            DynamoDbError::ValidationError(format!(
                "The table does not have the specified index: {index_name}"
            ))
        })?;
    let hash = index
        .key_schema
        .iter()
        .find(|k| k.key_type == "HASH")
        .map(|k| k.attribute_name.as_str())
        .ok_or(DynamoDbError::NoHashKey)?;
    let range = index
        .key_schema
        .iter()
        .find(|k| k.key_type == "RANGE")
        .map(|k| k.attribute_name.as_str());
    Ok((hash, range))
}

/// Whether `item` belongs to a (sparse) secondary index: it must carry
/// every index key attribute.
fn in_index(item: &Item, hash_attr: &str, range_attr: Option<&str>) -> bool {
    item.contains_key(hash_attr) && range_attr.is_none_or(|r| item.contains_key(r))
}

/// Order `items` by [`ReadPosition`], skip past `exclusive_start_key`, apply
/// `limit`, and emit a `LastEvaluatedKey` made of `key_attrs` when the page
/// is truncated.
fn paginate(
    items: Vec<Item>,
    table: &StoredTable,
    sort_attr: Option<&str>,
    forward: bool,
    exclusive_start_key: Option<&Item>,
    limit: Option<usize>,
    key_attrs: &[&str],
) -> QueryResult {
    let mut positioned: Vec<(ReadPosition, Item)> = items
        .into_iter()
        .map(|item| (ReadPosition::of(&item, table, sort_attr), item))
        .collect();
    positioned.sort_by(|a, b| a.0.cmp(&b.0));
    if !forward {
        positioned.reverse();
    }
    if let Some(start_key) = exclusive_start_key {
        let start = ReadPosition::of(start_key, table, sort_attr);
        positioned.retain(|(position, _)| {
            let ord = position.cmp(&start);
            if forward { ord.is_gt() } else { ord.is_lt() }
        });
    }
    let truncated = limit.is_some_and(|limit| positioned.len() > limit);
    if let Some(limit) = limit {
        positioned.truncate(limit);
    }
    let items: Vec<Item> = positioned.into_iter().map(|(_, item)| item).collect();
    let last_evaluated_key = items.last().filter(|_| truncated).map(|item| {
        key_attrs
            .iter()
            .filter_map(|attr| item.get(*attr).map(|v| (attr.to_string(), v.clone())))
            .collect()
    });
    let count = items.len();
    QueryResult {
        items,
        count,
        scanned_count: count,
        last_evaluated_key,
    }
}

/// The attributes a `LastEvaluatedKey` carries: the table primary key, plus
/// the index key when reading an index.
fn page_key_attrs<'a>(
    table: &'a StoredTable,
    index_keys: Option<(&'a str, Option<&'a str>)>,
) -> Vec<&'a str> {
    let mut attrs = vec![table.hash_key_attr.as_str()];
    attrs.extend(table.range_key_attr.as_deref());
    if let Some((hash, range)) = index_keys {
        for attr in std::iter::once(hash).chain(range) {
            if !attrs.contains(&attr) {
                attrs.push(attr);
            }
        }
    }
    attrs
}

// ---------------------------------------------------------------------------
// Stored types (Serde-serialisable counterparts to domain types)
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone)]
struct StoredKeySchemaElement {
    attribute_name: String,
    key_type: String,
}

impl From<&KeySchemaElement> for StoredKeySchemaElement {
    fn from(k: &KeySchemaElement) -> Self {
        Self {
            attribute_name: k.attribute_name.clone(),
            key_type: k.key_type.clone(),
        }
    }
}

impl From<StoredKeySchemaElement> for KeySchemaElement {
    fn from(s: StoredKeySchemaElement) -> Self {
        Self {
            attribute_name: s.attribute_name,
            key_type: s.key_type,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct StoredAttributeDefinition {
    attribute_name: String,
    attribute_type: String,
}

impl From<&AttributeDefinition> for StoredAttributeDefinition {
    fn from(a: &AttributeDefinition) -> Self {
        Self {
            attribute_name: a.attribute_name.clone(),
            attribute_type: a.attribute_type.clone(),
        }
    }
}

impl From<StoredAttributeDefinition> for AttributeDefinition {
    fn from(s: StoredAttributeDefinition) -> Self {
        Self {
            attribute_name: s.attribute_name,
            attribute_type: s.attribute_type,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct StoredProvisionedThroughput {
    read_capacity_units: i64,
    write_capacity_units: i64,
}

impl From<&ProvisionedThroughput> for StoredProvisionedThroughput {
    fn from(p: &ProvisionedThroughput) -> Self {
        Self {
            read_capacity_units: p.read_capacity_units,
            write_capacity_units: p.write_capacity_units,
        }
    }
}

impl From<StoredProvisionedThroughput> for ProvisionedThroughput {
    fn from(s: StoredProvisionedThroughput) -> Self {
        Self {
            read_capacity_units: s.read_capacity_units,
            write_capacity_units: s.write_capacity_units,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct StoredSecondaryIndex {
    index_name: String,
    key_schema: Vec<StoredKeySchemaElement>,
    projection_type: String,
    non_key_attributes: Vec<String>,
}

impl From<&GlobalSecondaryIndexDef> for StoredSecondaryIndex {
    fn from(index: &GlobalSecondaryIndexDef) -> Self {
        Self {
            index_name: index.index_name.clone(),
            key_schema: index
                .key_schema
                .iter()
                .map(StoredKeySchemaElement::from)
                .collect(),
            projection_type: index.projection_type.clone(),
            non_key_attributes: index.non_key_attributes.clone(),
        }
    }
}

impl From<&LocalSecondaryIndexDef> for StoredSecondaryIndex {
    fn from(index: &LocalSecondaryIndexDef) -> Self {
        Self {
            index_name: index.index_name.clone(),
            key_schema: index
                .key_schema
                .iter()
                .map(StoredKeySchemaElement::from)
                .collect(),
            projection_type: index.projection_type.clone(),
            non_key_attributes: index.non_key_attributes.clone(),
        }
    }
}

impl StoredSecondaryIndex {
    fn into_gsi(self) -> GlobalSecondaryIndexDef {
        GlobalSecondaryIndexDef {
            index_name: self.index_name,
            key_schema: self
                .key_schema
                .into_iter()
                .map(KeySchemaElement::from)
                .collect(),
            projection_type: self.projection_type,
            non_key_attributes: self.non_key_attributes,
        }
    }

    fn into_lsi(self) -> LocalSecondaryIndexDef {
        LocalSecondaryIndexDef {
            index_name: self.index_name,
            key_schema: self
                .key_schema
                .into_iter()
                .map(KeySchemaElement::from)
                .collect(),
            projection_type: self.projection_type,
            non_key_attributes: self.non_key_attributes,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct StoredTable {
    name: String,
    arn: String,
    key_schema: Vec<StoredKeySchemaElement>,
    attribute_definitions: Vec<StoredAttributeDefinition>,
    billing_mode: String,
    provisioned_throughput: Option<StoredProvisionedThroughput>,
    creation_date_time: String, // RFC3339
    table_status: String,
    item_count: usize,
    hash_key_attr: String,
    hash_key_type: String,
    range_key_attr: Option<String>,
    range_key_type: Option<String>,
    #[serde(default)]
    stream_enabled: bool,
    #[serde(default)]
    stream_view_type: Option<String>,
    #[serde(default)]
    latest_stream_arn: Option<String>,
    #[serde(default)]
    latest_stream_label: Option<String>,
    #[serde(default)]
    global_secondary_indexes: Vec<StoredSecondaryIndex>,
    #[serde(default)]
    local_secondary_indexes: Vec<StoredSecondaryIndex>,
}

impl From<&Table> for StoredTable {
    fn from(t: &Table) -> Self {
        Self {
            name: t.name.clone(),
            arn: t.arn.clone(),
            key_schema: t
                .key_schema
                .iter()
                .map(StoredKeySchemaElement::from)
                .collect(),
            attribute_definitions: t
                .attribute_definitions
                .iter()
                .map(StoredAttributeDefinition::from)
                .collect(),
            billing_mode: t.billing_mode.clone(),
            provisioned_throughput: t
                .provisioned_throughput
                .as_ref()
                .map(StoredProvisionedThroughput::from),
            creation_date_time: t.creation_date_time.to_rfc3339(),
            table_status: t.table_status.clone(),
            item_count: t.item_count,
            hash_key_attr: t.hash_key_attr.clone(),
            hash_key_type: t.hash_key_type.clone(),
            range_key_attr: t.range_key_attr.clone(),
            range_key_type: t.range_key_type.clone(),
            stream_enabled: t.stream_enabled,
            stream_view_type: t.stream_view_type.clone(),
            latest_stream_arn: t.latest_stream_arn.clone(),
            latest_stream_label: t.latest_stream_label.clone(),
            global_secondary_indexes: t
                .global_secondary_indexes
                .iter()
                .map(StoredSecondaryIndex::from)
                .collect(),
            local_secondary_indexes: t
                .local_secondary_indexes
                .iter()
                .map(StoredSecondaryIndex::from)
                .collect(),
        }
    }
}

impl TryFrom<StoredTable> for Table {
    type Error = chrono::ParseError;
    fn try_from(s: StoredTable) -> Result<Self, Self::Error> {
        Ok(Table {
            name: s.name,
            arn: s.arn,
            key_schema: s
                .key_schema
                .into_iter()
                .map(KeySchemaElement::from)
                .collect(),
            attribute_definitions: s
                .attribute_definitions
                .into_iter()
                .map(AttributeDefinition::from)
                .collect(),
            billing_mode: s.billing_mode,
            provisioned_throughput: s.provisioned_throughput.map(ProvisionedThroughput::from),
            creation_date_time: chrono::DateTime::parse_from_rfc3339(&s.creation_date_time)
                .map(|dt| dt.with_timezone(&Utc))?,
            table_status: s.table_status,
            item_count: s.item_count,
            hash_key_attr: s.hash_key_attr,
            hash_key_type: s.hash_key_type,
            range_key_attr: s.range_key_attr,
            range_key_type: s.range_key_type,
            stream_enabled: s.stream_enabled,
            stream_view_type: s.stream_view_type,
            latest_stream_arn: s.latest_stream_arn,
            latest_stream_label: s.latest_stream_label,
            global_secondary_indexes: s
                .global_secondary_indexes
                .into_iter()
                .map(StoredSecondaryIndex::into_gsi)
                .collect(),
            local_secondary_indexes: s
                .local_secondary_indexes
                .into_iter()
                .map(StoredSecondaryIndex::into_lsi)
                .collect(),
        })
    }
}

fn table_parse_err(e: chrono::ParseError) -> DynamoDbError {
    DynamoDbError::InternalError(format!("Failed to parse stored table: {e}"))
}

#[derive(Serialize, Deserialize, Clone)]
struct StoredBackup {
    backup_arn: String,
    backup_name: String,
    table_name: String,
    table_arn: String,
    backup_status: String,
    backup_type: String,
    backup_creation_date_time: String,
    backup_size_bytes: i64,
    key_schema: Vec<StoredKeySchemaElement>,
    provisioned_throughput: Option<StoredProvisionedThroughput>,
    item_count: usize,
    table_id: String,
    table_creation_date_time: String,
    billing_mode: String,
    items: HashMap<String, HashMap<String, serde_json::Value>>,
    table_snapshot: StoredTable,
}

impl From<&Backup> for StoredBackup {
    fn from(b: &Backup) -> Self {
        // Serialize items as a nested JSON map
        let items: HashMap<String, HashMap<String, serde_json::Value>> = b
            .items
            .iter()
            .map(|(hk, inner)| {
                let inner_ser: HashMap<String, serde_json::Value> = inner
                    .iter()
                    .map(|(rk, item)| {
                        (
                            rk.clone(),
                            serde_json::to_value(item).unwrap_or(serde_json::Value::Null),
                        )
                    })
                    .collect();
                (hk.clone(), inner_ser)
            })
            .collect();
        Self {
            backup_arn: b.backup_arn.clone(),
            backup_name: b.backup_name.clone(),
            table_name: b.table_name.clone(),
            table_arn: b.table_arn.clone(),
            backup_status: b.backup_status.clone(),
            backup_type: b.backup_type.clone(),
            backup_creation_date_time: b.backup_creation_date_time.to_rfc3339(),
            backup_size_bytes: b.backup_size_bytes,
            key_schema: b
                .key_schema
                .iter()
                .map(StoredKeySchemaElement::from)
                .collect(),
            provisioned_throughput: b
                .provisioned_throughput
                .as_ref()
                .map(StoredProvisionedThroughput::from),
            item_count: b.item_count,
            table_id: b.table_id.clone(),
            table_creation_date_time: b.table_creation_date_time.to_rfc3339(),
            billing_mode: b.billing_mode.clone(),
            items,
            table_snapshot: StoredTable::from(&b.table_snapshot),
        }
    }
}

impl TryFrom<StoredBackup> for Backup {
    type Error = DynamoDbError;
    fn try_from(s: StoredBackup) -> Result<Self, Self::Error> {
        let backup_creation_date_time =
            chrono::DateTime::parse_from_rfc3339(&s.backup_creation_date_time)
                .map(|dt| dt.with_timezone(&Utc))
                .map_err(table_parse_err)?;
        let table_creation_date_time =
            chrono::DateTime::parse_from_rfc3339(&s.table_creation_date_time)
                .map(|dt| dt.with_timezone(&Utc))
                .map_err(table_parse_err)?;
        let table_snapshot = Table::try_from(s.table_snapshot).map_err(table_parse_err)?;

        // Deserialize items
        let items: HashMap<String, HashMap<String, Item>> = s
            .items
            .into_iter()
            .map(|(hk, inner)| {
                let inner_des: HashMap<String, Item> = inner
                    .into_iter()
                    .filter_map(|(rk, val)| {
                        serde_json::from_value::<Item>(val)
                            .ok()
                            .map(|item| (rk, item))
                    })
                    .collect();
                (hk, inner_des)
            })
            .collect();

        Ok(Backup {
            backup_arn: s.backup_arn,
            backup_name: s.backup_name,
            table_name: s.table_name,
            table_arn: s.table_arn,
            backup_status: s.backup_status,
            backup_type: s.backup_type,
            backup_creation_date_time,
            backup_size_bytes: s.backup_size_bytes,
            key_schema: s
                .key_schema
                .into_iter()
                .map(KeySchemaElement::from)
                .collect(),
            provisioned_throughput: s.provisioned_throughput.map(ProvisionedThroughput::from),
            item_count: s.item_count,
            table_id: s.table_id,
            table_creation_date_time,
            billing_mode: s.billing_mode,
            items,
            table_snapshot,
        })
    }
}

#[derive(Serialize, Deserialize)]
struct StoredGlobalTable {
    global_table_name: String,
    global_table_arn: String,
    global_table_status: String,
    creation_date_time: String,
    replication_group: Vec<String>,
}

impl From<&GlobalTableInfo> for StoredGlobalTable {
    fn from(g: &GlobalTableInfo) -> Self {
        Self {
            global_table_name: g.global_table_name.clone(),
            global_table_arn: g.global_table_arn.clone(),
            global_table_status: g.global_table_status.clone(),
            creation_date_time: g.creation_date_time.to_rfc3339(),
            replication_group: g.replication_group.clone(),
        }
    }
}

impl TryFrom<StoredGlobalTable> for GlobalTableInfo {
    type Error = DynamoDbError;
    fn try_from(s: StoredGlobalTable) -> Result<Self, Self::Error> {
        let creation_date_time = chrono::DateTime::parse_from_rfc3339(&s.creation_date_time)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(table_parse_err)?;
        Ok(GlobalTableInfo {
            global_table_name: s.global_table_name,
            global_table_arn: s.global_table_arn,
            global_table_status: s.global_table_status,
            creation_date_time,
            replication_group: s.replication_group,
        })
    }
}

#[derive(Serialize, Deserialize)]
struct StoredKinesisDestination {
    stream_arn: String,
    destination_status: String,
    approximate_creation_date_time_precision: Option<String>,
}

impl From<&KinesisStreamingDestination> for StoredKinesisDestination {
    fn from(k: &KinesisStreamingDestination) -> Self {
        Self {
            stream_arn: k.stream_arn.clone(),
            destination_status: k.destination_status.clone(),
            approximate_creation_date_time_precision: k
                .approximate_creation_date_time_precision
                .clone(),
        }
    }
}

impl From<StoredKinesisDestination> for KinesisStreamingDestination {
    fn from(s: StoredKinesisDestination) -> Self {
        KinesisStreamingDestination {
            stream_arn: s.stream_arn,
            destination_status: s.destination_status,
            approximate_creation_date_time_precision: s.approximate_creation_date_time_precision,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct StoredContributorInsights {
    table_name: String,
    index_name: Option<String>,
    status: String,
    last_update_date_time: String,
}

impl From<&ContributorInsightsConfig> for StoredContributorInsights {
    fn from(c: &ContributorInsightsConfig) -> Self {
        Self {
            table_name: c.table_name.clone(),
            index_name: c.index_name.clone(),
            status: c.status.clone(),
            last_update_date_time: c.last_update_date_time.to_rfc3339(),
        }
    }
}

impl TryFrom<StoredContributorInsights> for ContributorInsightsConfig {
    type Error = DynamoDbError;
    fn try_from(s: StoredContributorInsights) -> Result<Self, Self::Error> {
        let last_update_date_time = chrono::DateTime::parse_from_rfc3339(&s.last_update_date_time)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(table_parse_err)?;
        Ok(ContributorInsightsConfig {
            table_name: s.table_name,
            index_name: s.index_name,
            status: s.status,
            last_update_date_time,
        })
    }
}

#[derive(Serialize, Deserialize)]
struct StoredExport {
    export_arn: String,
    table_arn: String,
    export_status: String,
    start_time: String,
    end_time: Option<String>,
    export_format: String,
    s3_bucket: Option<String>,
    s3_prefix: Option<String>,
}

impl From<&ExportInfo> for StoredExport {
    fn from(e: &ExportInfo) -> Self {
        Self {
            export_arn: e.export_arn.clone(),
            table_arn: e.table_arn.clone(),
            export_status: e.export_status.clone(),
            start_time: e.start_time.to_rfc3339(),
            end_time: e.end_time.map(|t| t.to_rfc3339()),
            export_format: e.export_format.clone(),
            s3_bucket: e.s3_bucket.clone(),
            s3_prefix: e.s3_prefix.clone(),
        }
    }
}

impl TryFrom<StoredExport> for ExportInfo {
    type Error = DynamoDbError;
    fn try_from(s: StoredExport) -> Result<Self, Self::Error> {
        let start_time = chrono::DateTime::parse_from_rfc3339(&s.start_time)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(table_parse_err)?;
        let end_time = s
            .end_time
            .map(|t| {
                chrono::DateTime::parse_from_rfc3339(&t)
                    .map(|dt| dt.with_timezone(&Utc))
                    .map_err(table_parse_err)
            })
            .transpose()?;
        Ok(ExportInfo {
            export_arn: s.export_arn,
            table_arn: s.table_arn,
            export_status: s.export_status,
            start_time,
            end_time,
            export_format: s.export_format,
            s3_bucket: s.s3_bucket,
            s3_prefix: s.s3_prefix,
        })
    }
}

// ---------------------------------------------------------------------------
// Redis helpers (load/save tables and items)
// ---------------------------------------------------------------------------

async fn load_table(
    conn: &mut ConnectionManager,
    acct: &str,
    rgn: &str,
    name: &str,
) -> Result<StoredTable, DynamoDbError> {
    let raw: Option<String> = conn
        .get(k_table(acct, rgn, name))
        .await
        .map_err(redis_err)?;
    match raw {
        None => Err(table_not_found(name)),
        Some(s) => serde_json::from_str::<StoredTable>(&s).map_err(json_err),
    }
}

async fn save_table(
    conn: &mut ConnectionManager,
    acct: &str,
    rgn: &str,
    table: &Table,
) -> Result<(), DynamoDbError> {
    let stored = StoredTable::from(table);
    let json = serde_json::to_string(&stored).map_err(json_err)?;
    conn.set::<_, _, ()>(k_table(acct, rgn, &table.name), json)
        .await
        .map_err(redis_err)?;
    conn.sadd::<_, _, ()>(k_tables(acct, rgn), &table.name)
        .await
        .map_err(redis_err)?;
    Ok(())
}

async fn load_all_items(
    conn: &mut ConnectionManager,
    acct: &str,
    rgn: &str,
    table_name: &str,
) -> Result<Vec<Item>, DynamoDbError> {
    let raw: HashMap<String, String> = conn
        .hgetall(k_items(acct, rgn, table_name))
        .await
        .map_err(redis_err)?;
    let mut items = Vec::with_capacity(raw.len());
    for (_field, json) in raw {
        let item: Item = serde_json::from_str(&json).map_err(json_err)?;
        items.push(item);
    }
    Ok(items)
}

// ---------------------------------------------------------------------------
// RedisDynamoDbBackend
// ---------------------------------------------------------------------------

/// Redis-backed implementation of [`DynamoDbBackend`].
///
/// All state is stored in Redis so it survives process restarts and can be
/// shared across multiple `winterbaume` server instances.
pub struct RedisDynamoDbBackend {
    conn: ConnectionManager,
}

impl RedisDynamoDbBackend {
    pub fn new(conn: ConnectionManager) -> Self {
        Self { conn }
    }

    /// Connect to Redis at `url` and select database `db`, overriding
    /// whatever database number was parsed from the URL.
    ///
    /// Useful for sharing one Redis instance between multiple services
    /// (e.g. DynamoDB on db 0 and SQS on db 1) without having to encode
    /// the database number into the URL path.
    pub async fn from_connection_info(
        info: impl redis::IntoConnectionInfo,
    ) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(info)?;
        let conn = ConnectionManager::new(client).await?;
        Ok(Self { conn })
    }
}

impl DynamoDbBackend for RedisDynamoDbBackend {
    // --- Table management ---

    fn create_table(
        &self,
        account_id: String,
        region: String,
        name: String,
        key_schema: Vec<KeySchemaElement>,
        attribute_definitions: Vec<AttributeDefinition>,
        billing_mode: Option<String>,
        provisioned_throughput: Option<ProvisionedThroughput>,
        stream_enabled: bool,
        stream_view_type: Option<String>,
        global_secondary_indexes: Vec<GlobalSecondaryIndexDef>,
        local_secondary_indexes: Vec<LocalSecondaryIndexDef>,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;

            // Check if table already exists
            let exists: bool = conn
                .sismember(k_tables(acct, rgn), &name)
                .await
                .map_err(redis_err)?;
            if exists {
                return Err(DynamoDbError::ResourceInUse(name.clone()));
            }

            // Determine key attributes
            let hash_schema = key_schema
                .iter()
                .find(|k| k.key_type == "HASH")
                .ok_or(DynamoDbError::NoHashKey)?;
            let range_schema = key_schema.iter().find(|k| k.key_type == "RANGE");

            let hash_key_attr = hash_schema.attribute_name.clone();
            let hash_key_type = attribute_definitions
                .iter()
                .find(|a| a.attribute_name == hash_key_attr)
                .map(|a| a.attribute_type.clone())
                .unwrap_or_else(|| "S".to_string());

            let range_key_attr = range_schema.map(|r| r.attribute_name.clone());
            let range_key_type = range_key_attr.as_ref().and_then(|rk| {
                attribute_definitions
                    .iter()
                    .find(|a| &a.attribute_name == rk)
                    .map(|a| a.attribute_type.clone())
            });

            let table_arn = format!("arn:aws:dynamodb:{region}:{account_id}:table/{name}");
            let billing = billing_mode.unwrap_or_else(|| "PAY_PER_REQUEST".to_string());

            let table = Table {
                name: name.clone(),
                arn: table_arn,
                key_schema,
                attribute_definitions,
                billing_mode: billing,
                provisioned_throughput,
                creation_date_time: Utc::now(),
                table_status: "ACTIVE".to_string(),
                item_count: 0,
                hash_key_attr,
                hash_key_type,
                range_key_attr,
                range_key_type,
                stream_enabled,
                stream_view_type: stream_view_type.clone(),
                latest_stream_arn: None,
                latest_stream_label: None,
                global_secondary_indexes,
                local_secondary_indexes,
            };

            save_table(&mut conn, acct, rgn, &table).await?;
            Ok(table)
        })
    }

    fn delete_table(
        &self,
        account_id: String,
        region: String,
        name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let stored = load_table(&mut conn, acct, rgn, &name).await?;
            let mut table = Table::try_from(stored).map_err(table_parse_err)?;

            conn.del::<_, ()>(k_table(acct, rgn, &name))
                .await
                .map_err(redis_err)?;
            conn.del::<_, ()>(k_items(acct, rgn, &name))
                .await
                .map_err(redis_err)?;
            conn.srem::<_, _, ()>(k_tables(acct, rgn), &name)
                .await
                .map_err(redis_err)?;
            table.table_status = "DELETING".to_string();
            Ok(table)
        })
    }

    fn describe_table(
        &self,
        account_id: String,
        region: String,
        name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let mut stored = load_table(&mut conn, acct, rgn, &name).await?;
            // Update item_count from actual hash size
            let count: usize = conn
                .hlen(k_items(acct, rgn, &name))
                .await
                .map_err(redis_err)?;
            stored.item_count = count;
            Table::try_from(stored).map_err(table_parse_err)
        })
    }

    fn list_tables(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let mut names: Vec<String> = conn
                .smembers(k_tables(acct, rgn))
                .await
                .map_err(redis_err)?;
            names.sort();
            Ok(names)
        })
    }

    fn update_table(
        &self,
        account_id: String,
        region: String,
        name: String,
        billing_mode: Option<String>,
        provisioned_throughput: Option<ProvisionedThroughput>,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let stored = load_table(&mut conn, acct, rgn, &name).await?;
            let mut table = Table::try_from(stored).map_err(table_parse_err)?;
            if let Some(bm) = billing_mode {
                table.billing_mode = bm;
            }
            if let Some(pt) = provisioned_throughput {
                table.provisioned_throughput = Some(pt);
            }
            save_table(&mut conn, acct, rgn, &table).await?;
            Ok(table)
        })
    }

    // --- Item operations ---

    fn put_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        item: Item,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Item>, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let stored = load_table(&mut conn, acct, rgn, &table_name).await?;
            let field = item_field(
                &item,
                &stored.hash_key_attr,
                stored.range_key_attr.as_deref(),
            );

            // Get old item
            let old_raw: Option<String> = conn
                .hget(k_items(acct, rgn, &table_name), &field)
                .await
                .map_err(redis_err)?;
            let old_item: Option<Item> = old_raw
                .as_deref()
                .map(|s| serde_json::from_str(s).map_err(json_err))
                .transpose()?;

            let json = serde_json::to_string(&item).map_err(json_err)?;
            conn.hset::<_, _, _, ()>(k_items(acct, rgn, &table_name), &field, json)
                .await
                .map_err(redis_err)?;
            Ok(old_item)
        })
    }

    fn get_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        key: Item,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Item>, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let stored = load_table(&mut conn, acct, rgn, &table_name).await?;
            let field = item_field(
                &key,
                &stored.hash_key_attr,
                stored.range_key_attr.as_deref(),
            );

            let raw: Option<String> = conn
                .hget(k_items(acct, rgn, &table_name), &field)
                .await
                .map_err(redis_err)?;
            match raw {
                None => Ok(None),
                Some(s) => serde_json::from_str(&s).map(Some).map_err(json_err),
            }
        })
    }

    fn delete_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        key: Item,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Item>, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let stored = load_table(&mut conn, acct, rgn, &table_name).await?;
            let field = item_field(
                &key,
                &stored.hash_key_attr,
                stored.range_key_attr.as_deref(),
            );

            let raw: Option<String> = conn
                .hget(k_items(acct, rgn, &table_name), &field)
                .await
                .map_err(redis_err)?;
            let old_item: Option<Item> = raw
                .as_deref()
                .map(|s| serde_json::from_str(s).map_err(json_err))
                .transpose()?;

            if old_item.is_some() {
                conn.hdel::<_, _, ()>(k_items(acct, rgn, &table_name), &field)
                    .await
                    .map_err(redis_err)?;
            }
            Ok(old_item)
        })
    }

    fn update_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        key: Item,
        actions: Vec<UpdateAction>,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Item>, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let stored = load_table(&mut conn, acct, rgn, &table_name).await?;
            let field = item_field(
                &key,
                &stored.hash_key_attr,
                stored.range_key_attr.as_deref(),
            );

            let raw: Option<String> = conn
                .hget(k_items(acct, rgn, &table_name), &field)
                .await
                .map_err(redis_err)?;
            let mut item: Item = match raw {
                None => key.clone(),
                Some(s) => serde_json::from_str(&s).map_err(json_err)?,
            };

            winterbaume_dynamodb::expr::apply_update_actions(&mut item, &actions)
                .map_err(DynamoDbError::ValidationError)?;

            let json = serde_json::to_string(&item).map_err(json_err)?;
            conn.hset::<_, _, _, ()>(k_items(acct, rgn, &table_name), &field, json)
                .await
                .map_err(redis_err)?;
            Ok(Some(item))
        })
    }

    fn batch_get_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        keys: Vec<Item>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Item>, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let stored = load_table(&mut conn, acct, rgn, &table_name).await?;
            let mut results = Vec::new();
            for key in &keys {
                let field =
                    item_field(key, &stored.hash_key_attr, stored.range_key_attr.as_deref());
                let raw: Option<String> = conn
                    .hget(k_items(acct, rgn, &table_name), &field)
                    .await
                    .map_err(redis_err)?;
                if let Some(s) = raw {
                    let item: Item = serde_json::from_str(&s).map_err(json_err)?;
                    results.push(item);
                }
            }
            Ok(results)
        })
    }

    fn batch_write_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        puts: Vec<Item>,
        deletes: Vec<Item>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let stored = load_table(&mut conn, acct, rgn, &table_name).await?;
            for item in &puts {
                let field = item_field(
                    item,
                    &stored.hash_key_attr,
                    stored.range_key_attr.as_deref(),
                );
                let json = serde_json::to_string(item).map_err(json_err)?;
                conn.hset::<_, _, _, ()>(k_items(acct, rgn, &table_name), &field, json)
                    .await
                    .map_err(redis_err)?;
            }
            for key in &deletes {
                let field =
                    item_field(key, &stored.hash_key_attr, stored.range_key_attr.as_deref());
                conn.hdel::<_, _, ()>(k_items(acct, rgn, &table_name), &field)
                    .await
                    .map_err(redis_err)?;
            }
            Ok(())
        })
    }

    fn query(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        key_conditions: Item,
        sort_key_condition: Option<SortKeyCondition>,
        limit: Option<usize>,
        scan_index_forward: bool,
        exclusive_start_key: Option<Item>,
        index_name: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<QueryResult, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let stored = load_table(&mut conn, acct, rgn, &table_name).await?;
            let all_items = load_all_items(&mut conn, acct, rgn, &table_name).await?;

            // Resolve hash and (optional) range key attribute names from the
            // index when one is named, otherwise from the table.
            let index_keys = index_name
                .as_deref()
                .map(|index_name| index_key_attrs(&stored, index_name))
                .transpose()?;
            let (hash_key, range_key) = index_keys.unwrap_or((
                stored.hash_key_attr.as_str(),
                stored.range_key_attr.as_deref(),
            ));

            // Indexes are sparse: an item missing an index key attribute is
            // not present in the index at all.
            let hash_val = key_conditions.get(hash_key);
            let mut filtered: Vec<Item> = all_items
                .into_iter()
                .filter(|item| in_index(item, hash_key, range_key))
                .filter(|item| hash_val.is_none_or(|hv| item.get(hash_key) == Some(hv)))
                .collect();

            // Apply sort-key equality from `key_conditions` (when the request
            // used `sk = :v` on the range key).
            if let Some(rk_attr) = range_key
                && let Some(rk_eq) = key_conditions.get(rk_attr)
            {
                filtered.retain(|item| item.get(rk_attr) == Some(rk_eq));
            }

            // Apply the sort-key range condition (`<`, `<=`, `>`, `>=`,
            // `BETWEEN`, `begins_with`) on the range-key attribute. Items
            // with an incompatible type are dropped.
            if let (Some(cond), Some(rk_attr)) = (sort_key_condition.as_ref(), range_key) {
                filtered.retain(|item| item.get(rk_attr).is_some_and(|v| cond.matches(v)));
            }

            // Order by the sort key, breaking ties (index keys are not
            // unique) with the table primary key, then page. The same order
            // positions `ExclusiveStartKey`, which carries both key sets.
            let key_attrs = page_key_attrs(&stored, index_keys);
            Ok(paginate(
                filtered,
                &stored,
                range_key,
                scan_index_forward,
                exclusive_start_key.as_ref(),
                limit,
                &key_attrs,
            ))
        })
    }

    fn scan(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        limit: Option<usize>,
        exclusive_start_key: Option<Item>,
    ) -> Pin<Box<dyn Future<Output = Result<QueryResult, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let stored = load_table(&mut conn, acct, rgn, &table_name).await?;
            let all_items = load_all_items(&mut conn, acct, rgn, &table_name).await?;
            let key_attrs = page_key_attrs(&stored, None);
            Ok(paginate(
                all_items,
                &stored,
                None,
                true,
                exclusive_start_key.as_ref(),
                limit,
                &key_attrs,
            ))
        })
    }

    fn scan_index(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        index_name: String,
        limit: Option<usize>,
        exclusive_start_key: Option<Item>,
    ) -> Pin<Box<dyn Future<Output = Result<QueryResult, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let stored = load_table(&mut conn, acct, rgn, &table_name).await?;
            let (hash_key, range_key) = index_key_attrs(&stored, &index_name)?;
            // Drop the items absent from the sparse index before `Limit`
            // applies, so a page is never short because of them.
            let items: Vec<Item> = load_all_items(&mut conn, acct, rgn, &table_name)
                .await?
                .into_iter()
                .filter(|item| in_index(item, hash_key, range_key))
                .collect();
            let key_attrs = page_key_attrs(&stored, Some((hash_key, range_key)));
            Ok(paginate(
                items,
                &stored,
                None,
                true,
                exclusive_start_key.as_ref(),
                limit,
                &key_attrs,
            ))
        })
    }

    fn transact_get_items(
        &self,
        account_id: String,
        region: String,
        keys: Vec<(String, Item)>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Option<Item>>, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let mut results = Vec::new();
            for (table_name, key) in &keys {
                let stored = load_table(&mut conn, acct, rgn, table_name).await?;
                let field =
                    item_field(key, &stored.hash_key_attr, stored.range_key_attr.as_deref());
                let raw: Option<String> = conn
                    .hget(k_items(acct, rgn, table_name), &field)
                    .await
                    .map_err(redis_err)?;
                let item: Option<Item> = raw
                    .as_deref()
                    .map(|s| serde_json::from_str(s).map_err(json_err))
                    .transpose()?;
                results.push(item);
            }
            Ok(results)
        })
    }

    fn transact_write_items(
        &self,
        account_id: String,
        region: String,
        ops: Vec<TransactOp>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;

            // Resolve every action's storage field up front: it doubles as the
            // item's identity, so two actions on one item are caught before
            // anything is written.
            let mut resolved: Vec<(&TransactOp, String)> = Vec::with_capacity(ops.len());
            let mut seen: HashSet<(&str, String)> = HashSet::new();
            for op in &ops {
                let stored = load_table(&mut conn, acct, rgn, op.table_name()).await?;
                let field = item_field(
                    op.target(),
                    &stored.hash_key_attr,
                    stored.range_key_attr.as_deref(),
                );
                if !seen.insert((op.table_name(), field.clone())) {
                    return Err(DynamoDbError::TransactionConflict);
                }
                resolved.push((op, field));
            }

            // Then apply in the caller's order.
            for (op, field) in resolved {
                let table_name = op.table_name();
                let items_key = k_items(acct, rgn, table_name);
                match op {
                    TransactOp::Put { item, .. } => {
                        let json = serde_json::to_string(item).map_err(json_err)?;
                        conn.hset::<_, _, _, ()>(items_key, &field, json)
                            .await
                            .map_err(redis_err)?;
                    }
                    TransactOp::Delete { .. } => {
                        conn.hdel::<_, _, ()>(items_key, &field)
                            .await
                            .map_err(redis_err)?;
                    }
                    TransactOp::Update { key, actions, .. } => {
                        let raw: Option<String> =
                            conn.hget(&items_key, &field).await.map_err(redis_err)?;
                        let mut item: Item = match raw {
                            None => key.clone(),
                            Some(s) => serde_json::from_str(&s).map_err(json_err)?,
                        };
                        winterbaume_dynamodb::expr::apply_update_actions(&mut item, actions)
                            .map_err(DynamoDbError::ValidationError)?;
                        let json = serde_json::to_string(&item).map_err(json_err)?;
                        conn.hset::<_, _, _, ()>(items_key, &field, json)
                            .await
                            .map_err(redis_err)?;
                    }
                }
            }
            Ok(())
        })
    }

    // --- Backup operations ---

    fn create_backup(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        backup_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Backup, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let stored = load_table(&mut conn, acct, rgn, &table_name).await?;
            let table = Table::try_from(stored.clone()).map_err(table_parse_err)?;

            let ctr: i64 = conn
                .incr(k_backup_ctr(acct, rgn), 1)
                .await
                .map_err(redis_err)?;
            let backup_arn = format!(
                "arn:aws:dynamodb:{region}:{account_id}:table/{table_name}/backup/{backup_name}-{ctr:08}"
            );

            // Snapshot all items
            let all_items = load_all_items(&mut conn, acct, rgn, &table_name).await?;
            let hash_key = stored.hash_key_attr.clone();
            let range_key = stored.range_key_attr.clone();
            let mut items_snapshot: HashMap<String, HashMap<String, Item>> = HashMap::new();
            for item in all_items {
                let hv = item
                    .get(&hash_key)
                    .and_then(|v| match v {
                        AttributeValue::S(s) => Some(s.as_str()),
                        AttributeValue::N(n) => Some(n.as_str()),
                        _ => None,
                    })
                    .unwrap_or("")
                    .to_string();
                let rv = range_key
                    .as_ref()
                    .and_then(|rk| item.get(rk))
                    .and_then(|v| match v {
                        AttributeValue::S(s) => Some(s.as_str()),
                        AttributeValue::N(n) => Some(n.as_str()),
                        _ => None,
                    })
                    .unwrap_or("")
                    .to_string();
                items_snapshot.entry(hv).or_default().insert(rv, item);
            }

            let backup = Backup {
                backup_arn: backup_arn.clone(),
                backup_name,
                table_name: table_name.clone(),
                table_arn: table.arn.clone(),
                backup_status: "AVAILABLE".to_string(),
                backup_type: "USER".to_string(),
                backup_creation_date_time: Utc::now(),
                backup_size_bytes: 0,
                key_schema: table.key_schema.clone(),
                provisioned_throughput: table.provisioned_throughput.clone(),
                item_count: items_snapshot.values().map(|m| m.len()).sum(),
                table_id: Uuid::new_v4().to_string(),
                table_creation_date_time: table.creation_date_time,
                billing_mode: table.billing_mode.clone(),
                items: items_snapshot,
                table_snapshot: table,
            };

            let arn_b64 = b64(&backup_arn);
            let json = serde_json::to_string(&StoredBackup::from(&backup)).map_err(json_err)?;
            conn.set::<_, _, ()>(k_backup(acct, rgn, &arn_b64), json)
                .await
                .map_err(redis_err)?;
            conn.sadd::<_, _, ()>(k_backups(acct, rgn), &backup_arn)
                .await
                .map_err(redis_err)?;
            Ok(backup)
        })
    }

    fn delete_backup(
        &self,
        account_id: String,
        region: String,
        backup_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Backup, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let arn_b64 = b64(&backup_arn);
            let raw: Option<String> = conn
                .get(k_backup(acct, rgn, &arn_b64))
                .await
                .map_err(redis_err)?;
            let stored: StoredBackup = match raw {
                None => return Err(backup_not_found(&backup_arn)),
                Some(s) => serde_json::from_str(&s).map_err(json_err)?,
            };
            let backup = Backup::try_from(stored)?;
            conn.del::<_, ()>(k_backup(acct, rgn, &arn_b64))
                .await
                .map_err(redis_err)?;
            conn.srem::<_, _, ()>(k_backups(acct, rgn), &backup_arn)
                .await
                .map_err(redis_err)?;
            Ok(backup)
        })
    }

    fn describe_backup(
        &self,
        account_id: String,
        region: String,
        backup_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Backup, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let arn_b64 = b64(&backup_arn);
            let raw: Option<String> = conn
                .get(k_backup(acct, rgn, &arn_b64))
                .await
                .map_err(redis_err)?;
            match raw {
                None => Err(backup_not_found(&backup_arn)),
                Some(s) => {
                    let stored: StoredBackup = serde_json::from_str(&s).map_err(json_err)?;
                    Backup::try_from(stored)
                }
            }
        })
    }

    fn list_backups(
        &self,
        account_id: String,
        region: String,
        table_name: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Backup>, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let arns: Vec<String> = conn
                .smembers(k_backups(acct, rgn))
                .await
                .map_err(redis_err)?;
            let mut backups = Vec::new();
            for arn in &arns {
                let arn_b64 = b64(arn);
                let raw: Option<String> = conn
                    .get(k_backup(acct, rgn, &arn_b64))
                    .await
                    .map_err(redis_err)?;
                if let Some(s) = raw {
                    let stored: StoredBackup = serde_json::from_str(&s).map_err(json_err)?;
                    if let Some(ref tn) = table_name {
                        if &stored.table_name != tn {
                            continue;
                        }
                    }
                    backups.push(Backup::try_from(stored)?);
                }
            }
            backups.sort_by(|a, b| {
                a.backup_creation_date_time
                    .cmp(&b.backup_creation_date_time)
            });
            Ok(backups)
        })
    }

    fn restore_table_from_backup(
        &self,
        account_id: String,
        region: String,
        backup_arn: String,
        target_table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let arn_b64 = b64(&backup_arn);
            let raw: Option<String> = conn
                .get(k_backup(acct, rgn, &arn_b64))
                .await
                .map_err(redis_err)?;
            let stored: StoredBackup = match raw {
                None => return Err(backup_not_found(&backup_arn)),
                Some(s) => serde_json::from_str(&s).map_err(json_err)?,
            };
            let backup = Backup::try_from(stored)?;

            let new_arn =
                format!("arn:aws:dynamodb:{region}:{account_id}:table/{target_table_name}");
            let mut new_table = backup.table_snapshot.clone();
            new_table.name = target_table_name.clone();
            new_table.arn = new_arn;
            new_table.creation_date_time = Utc::now();
            new_table.table_status = "ACTIVE".to_string();

            save_table(&mut conn, acct, rgn, &new_table).await?;

            // Restore items
            for inner in backup.items.values() {
                for item in inner.values() {
                    let field = item_field(
                        item,
                        &new_table.hash_key_attr,
                        new_table.range_key_attr.as_deref(),
                    );
                    let json = serde_json::to_string(item).map_err(json_err)?;
                    conn.hset::<_, _, _, ()>(k_items(acct, rgn, &target_table_name), &field, json)
                        .await
                        .map_err(redis_err)?;
                }
            }
            Ok(new_table)
        })
    }

    fn restore_table_to_point_in_time(
        &self,
        account_id: String,
        region: String,
        source_table_name: String,
        target_table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            // For mock: just copy the live table as-is
            let stored = load_table(&mut conn, acct, rgn, &source_table_name).await?;
            let source_table = Table::try_from(stored).map_err(table_parse_err)?;

            let new_arn =
                format!("arn:aws:dynamodb:{region}:{account_id}:table/{target_table_name}");
            let mut new_table = source_table;
            new_table.name = target_table_name.clone();
            new_table.arn = new_arn;
            new_table.creation_date_time = Utc::now();

            save_table(&mut conn, acct, rgn, &new_table).await?;

            // Copy items
            let all_items = load_all_items(&mut conn, acct, rgn, &source_table_name).await?;
            for item in &all_items {
                let field = item_field(
                    item,
                    &new_table.hash_key_attr,
                    new_table.range_key_attr.as_deref(),
                );
                let json = serde_json::to_string(item).map_err(json_err)?;
                conn.hset::<_, _, _, ()>(k_items(acct, rgn, &target_table_name), &field, json)
                    .await
                    .map_err(redis_err)?;
            }
            Ok(new_table)
        })
    }

    // --- Tag operations ---

    fn tag_resource(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
        tags: Vec<DynamoDbTag>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let arn_b64 = b64(&resource_arn);
            for tag in &tags {
                conn.hset::<_, _, _, ()>(k_tags(acct, rgn, &arn_b64), &tag.key, &tag.value)
                    .await
                    .map_err(redis_err)?;
            }
            Ok(())
        })
    }

    fn untag_resource(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
        tag_keys: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let arn_b64 = b64(&resource_arn);
            for key in &tag_keys {
                conn.hdel::<_, _, ()>(k_tags(acct, rgn, &arn_b64), key)
                    .await
                    .map_err(redis_err)?;
            }
            Ok(())
        })
    }

    fn list_tags_of_resource(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DynamoDbTag>, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let arn_b64 = b64(&resource_arn);
            let raw: HashMap<String, String> = conn
                .hgetall(k_tags(acct, rgn, &arn_b64))
                .await
                .map_err(redis_err)?;
            let mut tags: Vec<DynamoDbTag> = raw
                .into_iter()
                .map(|(key, value)| DynamoDbTag { key, value })
                .collect();
            tags.sort_by(|a, b| a.key.cmp(&b.key));
            Ok(tags)
        })
    }

    // --- TTL ---

    fn update_time_to_live(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        attribute_name: String,
        enabled: bool,
    ) -> Pin<Box<dyn Future<Output = Result<TtlConfig, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let config = TtlConfig {
                attribute_name: attribute_name.clone(),
                enabled,
            };
            let json = serde_json::to_string(&serde_json::json!({
                "attribute_name": attribute_name,
                "enabled": enabled
            }))
            .map_err(json_err)?;
            conn.set::<_, _, ()>(k_ttl(acct, rgn, &table_name), json)
                .await
                .map_err(redis_err)?;
            Ok(config)
        })
    }

    fn describe_time_to_live(
        &self,
        account_id: String,
        region: String,
        table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<TtlConfig>, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let raw: Option<String> = conn
                .get(k_ttl(acct, rgn, &table_name))
                .await
                .map_err(redis_err)?;
            match raw {
                None => Ok(None),
                Some(s) => {
                    let v: serde_json::Value = serde_json::from_str(&s).map_err(json_err)?;
                    Ok(Some(TtlConfig {
                        attribute_name: v["attribute_name"].as_str().unwrap_or("").to_string(),
                        enabled: v["enabled"].as_bool().unwrap_or(false),
                    }))
                }
            }
        })
    }

    // --- Continuous Backups / PITR ---

    fn update_continuous_backups(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        enabled: bool,
    ) -> Pin<Box<dyn Future<Output = Result<ContinuousBackupsConfig, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let config = ContinuousBackupsConfig {
                point_in_time_recovery_enabled: enabled,
            };
            let json = serde_json::to_string(&serde_json::json!({ "enabled": enabled }))
                .map_err(json_err)?;
            conn.set::<_, _, ()>(k_pitr(acct, rgn, &table_name), json)
                .await
                .map_err(redis_err)?;
            Ok(config)
        })
    }

    fn describe_continuous_backups(
        &self,
        account_id: String,
        region: String,
        table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<ContinuousBackupsConfig>, DynamoDbError>> + Send>>
    {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let raw: Option<String> = conn
                .get(k_pitr(acct, rgn, &table_name))
                .await
                .map_err(redis_err)?;
            match raw {
                None => Ok(None),
                Some(s) => {
                    let v: serde_json::Value = serde_json::from_str(&s).map_err(json_err)?;
                    Ok(Some(ContinuousBackupsConfig {
                        point_in_time_recovery_enabled: v["enabled"].as_bool().unwrap_or(false),
                    }))
                }
            }
        })
    }

    // --- Resource Policy ---

    fn put_resource_policy(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
        policy: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let arn_b64 = b64(&resource_arn);
            let revision_id = Uuid::new_v4().to_string();
            let json = serde_json::to_string(&serde_json::json!({
                "policy": policy,
                "revision_id": revision_id
            }))
            .map_err(json_err)?;
            conn.set::<_, _, ()>(k_policy(acct, rgn, &arn_b64), json)
                .await
                .map_err(redis_err)?;
            Ok(revision_id)
        })
    }

    fn get_resource_policy(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<ResourcePolicy>, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let arn_b64 = b64(&resource_arn);
            let raw: Option<String> = conn
                .get(k_policy(acct, rgn, &arn_b64))
                .await
                .map_err(redis_err)?;
            match raw {
                None => Ok(None),
                Some(s) => {
                    let v: serde_json::Value = serde_json::from_str(&s).map_err(json_err)?;
                    Ok(Some(ResourcePolicy {
                        policy: v["policy"].as_str().unwrap_or("").to_string(),
                        revision_id: v["revision_id"].as_str().unwrap_or("").to_string(),
                    }))
                }
            }
        })
    }

    fn delete_resource_policy(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let arn_b64 = b64(&resource_arn);
            let raw: Option<String> = conn
                .get(k_policy(acct, rgn, &arn_b64))
                .await
                .map_err(redis_err)?;
            let revision_id = raw
                .as_deref()
                .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
                .and_then(|v| v["revision_id"].as_str().map(|s| s.to_string()))
                .unwrap_or_default();
            conn.del::<_, ()>(k_policy(acct, rgn, &arn_b64))
                .await
                .map_err(redis_err)?;
            Ok(revision_id)
        })
    }

    // --- Global Tables (v1 API) ---

    fn create_global_table(
        &self,
        account_id: String,
        region: String,
        global_table_name: String,
        replication_group: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<GlobalTableInfo, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let exists: bool = conn
                .sismember(k_global_tables(acct, rgn), &global_table_name)
                .await
                .map_err(redis_err)?;
            if exists {
                return Err(DynamoDbError::GlobalTableAlreadyExists(
                    global_table_name.clone(),
                ));
            }
            let arn = format!("arn:aws:dynamodb:::{global_table_name}/replication");
            let info = GlobalTableInfo {
                global_table_name: global_table_name.clone(),
                global_table_arn: arn,
                global_table_status: "ACTIVE".to_string(),
                creation_date_time: Utc::now(),
                replication_group,
            };
            let stored = StoredGlobalTable::from(&info);
            let json = serde_json::to_string(&stored).map_err(json_err)?;
            conn.set::<_, _, ()>(k_global_table(acct, rgn, &global_table_name), json)
                .await
                .map_err(redis_err)?;
            conn.sadd::<_, _, ()>(k_global_tables(acct, rgn), &global_table_name)
                .await
                .map_err(redis_err)?;
            Ok(info)
        })
    }

    fn describe_global_table(
        &self,
        account_id: String,
        region: String,
        global_table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<GlobalTableInfo, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let raw: Option<String> = conn
                .get(k_global_table(acct, rgn, &global_table_name))
                .await
                .map_err(redis_err)?;
            match raw {
                None => Err(global_table_not_found(&global_table_name)),
                Some(s) => {
                    let stored: StoredGlobalTable = serde_json::from_str(&s).map_err(json_err)?;
                    GlobalTableInfo::try_from(stored)
                }
            }
        })
    }

    fn update_global_table(
        &self,
        account_id: String,
        region: String,
        global_table_name: String,
        replicas_to_create: Vec<String>,
        replicas_to_delete: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<GlobalTableInfo, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let raw: Option<String> = conn
                .get(k_global_table(acct, rgn, &global_table_name))
                .await
                .map_err(redis_err)?;
            let stored: StoredGlobalTable = match raw {
                None => return Err(global_table_not_found(&global_table_name)),
                Some(s) => serde_json::from_str(&s).map_err(json_err)?,
            };
            let mut info = GlobalTableInfo::try_from(stored)?;
            for r in &replicas_to_create {
                if !info.replication_group.contains(r) {
                    info.replication_group.push(r.clone());
                }
            }
            info.replication_group
                .retain(|r| !replicas_to_delete.contains(r));

            let new_stored = StoredGlobalTable::from(&info);
            let json = serde_json::to_string(&new_stored).map_err(json_err)?;
            conn.set::<_, _, ()>(k_global_table(acct, rgn, &global_table_name), json)
                .await
                .map_err(redis_err)?;
            Ok(info)
        })
    }

    fn list_global_tables(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<GlobalTableInfo>, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let names: Vec<String> = conn
                .smembers(k_global_tables(acct, rgn))
                .await
                .map_err(redis_err)?;
            let mut result = Vec::new();
            for name in &names {
                let raw: Option<String> = conn
                    .get(k_global_table(acct, rgn, name))
                    .await
                    .map_err(redis_err)?;
                if let Some(s) = raw {
                    let stored: StoredGlobalTable = serde_json::from_str(&s).map_err(json_err)?;
                    result.push(GlobalTableInfo::try_from(stored)?);
                }
            }
            result.sort_by(|a, b| a.global_table_name.cmp(&b.global_table_name));
            Ok(result)
        })
    }

    // --- Kinesis Streaming Destinations ---

    fn enable_kinesis_streaming_destination(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        stream_arn: String,
        precision: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<KinesisStreamingDestination, DynamoDbError>> + Send>>
    {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let dest = KinesisStreamingDestination {
                stream_arn: stream_arn.clone(),
                destination_status: "ACTIVE".to_string(),
                approximate_creation_date_time_precision: precision,
            };
            let stored = StoredKinesisDestination::from(&dest);
            let json = serde_json::to_string(&stored).map_err(json_err)?;
            let field = b64(&stream_arn);
            conn.hset::<_, _, _, ()>(k_kinesis(acct, rgn, &table_name), &field, json)
                .await
                .map_err(redis_err)?;
            Ok(dest)
        })
    }

    fn disable_kinesis_streaming_destination(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        stream_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<KinesisStreamingDestination, DynamoDbError>> + Send>>
    {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let field = b64(&stream_arn);
            let raw: Option<String> = conn
                .hget(k_kinesis(acct, rgn, &table_name), &field)
                .await
                .map_err(redis_err)?;
            let dest = match raw {
                None => KinesisStreamingDestination {
                    stream_arn: stream_arn.clone(),
                    destination_status: "INACTIVE".to_string(),
                    approximate_creation_date_time_precision: None,
                },
                Some(s) => {
                    let stored: StoredKinesisDestination =
                        serde_json::from_str(&s).map_err(json_err)?;
                    KinesisStreamingDestination::from(stored)
                }
            };
            conn.hdel::<_, _, ()>(k_kinesis(acct, rgn, &table_name), &field)
                .await
                .map_err(redis_err)?;
            Ok(KinesisStreamingDestination {
                destination_status: "INACTIVE".to_string(),
                ..dest
            })
        })
    }

    fn describe_kinesis_streaming_destinations(
        &self,
        account_id: String,
        region: String,
        table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<KinesisStreamingDestination>, DynamoDbError>> + Send>>
    {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let raw: HashMap<String, String> = conn
                .hgetall(k_kinesis(acct, rgn, &table_name))
                .await
                .map_err(redis_err)?;
            let mut dests = Vec::new();
            for (_field, json) in raw {
                let stored: StoredKinesisDestination =
                    serde_json::from_str(&json).map_err(json_err)?;
                dests.push(KinesisStreamingDestination::from(stored));
            }
            dests.sort_by(|a, b| a.stream_arn.cmp(&b.stream_arn));
            Ok(dests)
        })
    }

    fn update_kinesis_streaming_destination(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        stream_arn: String,
        precision: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<KinesisStreamingDestination, DynamoDbError>> + Send>>
    {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let field = b64(&stream_arn);
            let raw: Option<String> = conn
                .hget(k_kinesis(acct, rgn, &table_name), &field)
                .await
                .map_err(redis_err)?;
            let mut dest = match raw {
                None => KinesisStreamingDestination {
                    stream_arn: stream_arn.clone(),
                    destination_status: "ACTIVE".to_string(),
                    approximate_creation_date_time_precision: precision.clone(),
                },
                Some(s) => {
                    let stored: StoredKinesisDestination =
                        serde_json::from_str(&s).map_err(json_err)?;
                    KinesisStreamingDestination::from(stored)
                }
            };
            dest.approximate_creation_date_time_precision = precision;
            let stored = StoredKinesisDestination::from(&dest);
            let json = serde_json::to_string(&stored).map_err(json_err)?;
            conn.hset::<_, _, _, ()>(k_kinesis(acct, rgn, &table_name), &field, json)
                .await
                .map_err(redis_err)?;
            Ok(dest)
        })
    }

    // --- Contributor Insights ---

    fn update_contributor_insights(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        index_name: Option<String>,
        mode: String,
    ) -> Pin<Box<dyn Future<Output = Result<ContributorInsightsConfig, DynamoDbError>> + Send>>
    {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let status = if mode == "ENABLE" {
                "ENABLED"
            } else {
                "DISABLED"
            }
            .to_string();
            let config = ContributorInsightsConfig {
                table_name: table_name.clone(),
                index_name: index_name.clone(),
                status,
                last_update_date_time: Utc::now(),
            };
            let stored = StoredContributorInsights::from(&config);
            let json = serde_json::to_string(&stored).map_err(json_err)?;
            let ci_field = format!("{}|{}", table_name, index_name.as_deref().unwrap_or(""));
            conn.hset::<_, _, _, ()>(k_ci(acct, rgn), &ci_field, json)
                .await
                .map_err(redis_err)?;
            Ok(config)
        })
    }

    fn describe_contributor_insights(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        index_name: Option<String>,
    ) -> Pin<
        Box<dyn Future<Output = Result<Option<ContributorInsightsConfig>, DynamoDbError>> + Send>,
    > {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let ci_field = format!("{}|{}", table_name, index_name.as_deref().unwrap_or(""));
            let raw: Option<String> = conn
                .hget(k_ci(acct, rgn), &ci_field)
                .await
                .map_err(redis_err)?;
            match raw {
                None => Ok(None),
                Some(s) => {
                    let stored: StoredContributorInsights =
                        serde_json::from_str(&s).map_err(json_err)?;
                    Ok(Some(ContributorInsightsConfig::try_from(stored)?))
                }
            }
        })
    }

    fn list_contributor_insights(
        &self,
        account_id: String,
        region: String,
        table_name: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ContributorInsightsConfig>, DynamoDbError>> + Send>>
    {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let raw: HashMap<String, String> =
                conn.hgetall(k_ci(acct, rgn)).await.map_err(redis_err)?;
            let mut result = Vec::new();
            for (_field, json) in raw {
                let stored: StoredContributorInsights =
                    serde_json::from_str(&json).map_err(json_err)?;
                if let Some(ref tn) = table_name {
                    if &stored.table_name != tn {
                        continue;
                    }
                }
                result.push(ContributorInsightsConfig::try_from(stored)?);
            }
            result.sort_by(|a, b| a.table_name.cmp(&b.table_name));
            Ok(result)
        })
    }

    // --- Export ---

    fn export_table_to_point_in_time(
        &self,
        account_id: String,
        region: String,
        table_arn: String,
        s3_bucket: Option<String>,
        s3_prefix: Option<String>,
        export_format: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<ExportInfo, DynamoDbError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;
            let ctr: i64 = conn
                .incr(k_export_ctr(acct, rgn), 1)
                .await
                .map_err(redis_err)?;
            let export_arn =
                format!("arn:aws:dynamodb:{region}:{account_id}:table/export/{ctr:08}");
            let info = ExportInfo {
                export_arn: export_arn.clone(),
                table_arn,
                export_status: "COMPLETED".to_string(),
                start_time: Utc::now(),
                end_time: Some(Utc::now()),
                export_format: export_format.unwrap_or_else(|| "DYNAMODB_JSON".to_string()),
                s3_bucket,
                s3_prefix,
            };
            let stored = StoredExport::from(&info);
            let json = serde_json::to_string(&stored).map_err(json_err)?;
            let arn_b64 = b64(&export_arn);
            conn.set::<_, _, ()>(k_export(acct, rgn, &arn_b64), json)
                .await
                .map_err(redis_err)?;
            conn.sadd::<_, _, ()>(k_exports(acct, rgn), &export_arn)
                .await
                .map_err(redis_err)?;
            Ok(info)
        })
    }

    fn describe_export(
        &self,
        _account_id: String,
        _region: String,
        _export_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<ExportInfo, DynamoDbError>> + Send>> {
        Box::pin(async move {
            Err(DynamoDbError::InternalError(
                "describe_export not implemented in Redis backend".to_string(),
            ))
        })
    }

    fn list_exports(
        &self,
        _account_id: String,
        _region: String,
    ) -> Pin<Box<dyn Future<Output = Vec<ExportInfo>> + Send>> {
        Box::pin(async move { vec![] })
    }

    fn import_table(
        &self,
        _account_id: String,
        _region: String,
        _table_name: String,
        _s3_bucket_source: Option<String>,
        _input_format: Option<String>,
    ) -> Pin<Box<dyn Future<Output = ImportInfo> + Send>> {
        use winterbaume_dynamodb::types::ImportInfo;
        Box::pin(async move {
            ImportInfo {
                import_arn: String::new(),
                table_arn: None,
                import_status: "FAILED".to_string(),
                start_time: Utc::now(),
                end_time: Some(Utc::now()),
                s3_bucket_source: None,
                input_format: None,
            }
        })
    }

    fn describe_import(
        &self,
        _account_id: String,
        _region: String,
        _import_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<ImportInfo, DynamoDbError>> + Send>> {
        Box::pin(async move {
            Err(DynamoDbError::InternalError(
                "describe_import not implemented in Redis backend".to_string(),
            ))
        })
    }

    fn list_imports(
        &self,
        _account_id: String,
        _region: String,
    ) -> Pin<Box<dyn Future<Output = Vec<ImportInfo>> + Send>> {
        Box::pin(async move { vec![] })
    }

    fn update_stream_specification(
        &self,
        _account_id: String,
        _region: String,
        _table_name: String,
        _stream_enabled: bool,
        _stream_view_type: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>> {
        Box::pin(async move {
            Err(DynamoDbError::InternalError(
                "update_stream_specification not implemented in Redis backend".to_string(),
            ))
        })
    }

    fn list_streams(
        &self,
        _account_id: String,
        _region: String,
        _table_name: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<StreamSummary>, DynamoDbError>> + Send>> {
        Box::pin(async move { Ok(vec![]) })
    }

    fn describe_stream_by_arn(
        &self,
        _account_id: String,
        _region: String,
        stream_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<StreamSummary, DynamoDbError>> + Send>> {
        Box::pin(async move { Err(DynamoDbError::StreamNotFound(stream_arn)) })
    }

    fn get_stream_records(
        &self,
        _account_id: String,
        _region: String,
        _stream_arn: String,
        _next_sequence_number: u64,
        _limit: Option<usize>,
    ) -> Pin<Box<dyn Future<Output = Result<(Vec<StreamChangeRecord>, u64), DynamoDbError>> + Send>>
    {
        Box::pin(async move { Ok((vec![], 0)) })
    }

    fn scopes_with_state(&self) -> Vec<(String, String)> {
        // Redis backend does not maintain a local scope index; return empty
        // so the injector falls back to the default scope.
        vec![]
    }

    fn snapshot(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = DynamodbStateView> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;

            let table_names: Vec<String> =
                conn.smembers(k_tables(acct, rgn)).await.unwrap_or_default();

            let mut tables = HashMap::new();
            for name in &table_names {
                let raw: Option<String> = conn.get(k_table(acct, rgn, name)).await.ok().flatten();
                if let Some(json_str) = raw {
                    if let Ok(stored) = serde_json::from_str::<StoredTable>(&json_str) {
                        if let Ok(table) = Table::try_from(stored) {
                            // Load items
                            let item_map: HashMap<String, String> = conn
                                .hgetall(k_items(acct, rgn, name))
                                .await
                                .unwrap_or_default();
                            let mut items: HashMap<String, HashMap<String, Item>> = HashMap::new();
                            for (compound_key, item_json) in item_map {
                                if let Ok(item) = serde_json::from_str::<Item>(&item_json) {
                                    let parts: Vec<&str> = compound_key.splitn(2, '|').collect();
                                    let (hk, rk) = if parts.len() == 2 {
                                        (parts[0].to_string(), parts[1].to_string())
                                    } else {
                                        (compound_key, String::new())
                                    };
                                    items.entry(hk).or_default().insert(rk, item);
                                }
                            }
                            tables.insert(
                                name.clone(),
                                TableStateView {
                                    name: table.name,
                                    arn: table.arn,
                                    key_schema: table
                                        .key_schema
                                        .iter()
                                        .map(|k| KeySchemaElementView {
                                            attribute_name: k.attribute_name.clone(),
                                            key_type: k.key_type.clone(),
                                        })
                                        .collect(),
                                    attribute_definitions: table
                                        .attribute_definitions
                                        .iter()
                                        .map(|a| AttributeDefinitionView {
                                            attribute_name: a.attribute_name.clone(),
                                            attribute_type: a.attribute_type.clone(),
                                        })
                                        .collect(),
                                    billing_mode: table.billing_mode,
                                    provisioned_throughput: table.provisioned_throughput.map(
                                        |pt| ProvisionedThroughputView {
                                            read_capacity_units: pt.read_capacity_units,
                                            write_capacity_units: pt.write_capacity_units,
                                        },
                                    ),
                                    creation_date_time: table.creation_date_time.to_rfc3339(),
                                    table_status: table.table_status,
                                    hash_key_attr: table.hash_key_attr,
                                    hash_key_type: table.hash_key_type,
                                    range_key_attr: table.range_key_attr,
                                    range_key_type: table.range_key_type,
                                    items,
                                    stream_enabled: table.stream_enabled,
                                    stream_view_type: table.stream_view_type,
                                    latest_stream_arn: table.latest_stream_arn,
                                    latest_stream_label: table.latest_stream_label,
                                    global_secondary_index: vec![],
                                    local_secondary_index: vec![],
                                    import_table: None,
                                    on_demand_throughput: None,
                                    replica: vec![],
                                },
                            );
                        }
                    }
                }
            }

            // Load tags
            let mut tags = HashMap::new();
            for name in &table_names {
                let arn = format!("arn:aws:dynamodb:{rgn}:{acct}:table/{name}");
                let arn_b64 = b64(&arn);
                let tag_map: HashMap<String, String> = conn
                    .hgetall(k_tags(acct, rgn, &arn_b64))
                    .await
                    .unwrap_or_default();
                if !tag_map.is_empty() {
                    tags.insert(
                        arn,
                        tag_map
                            .into_iter()
                            .map(|(k, v)| DynamoDbTagView { key: k, value: v })
                            .collect(),
                    );
                }
            }

            // Load TTL configs
            let mut ttl_configs = HashMap::new();
            for name in &table_names {
                let raw: Option<String> = conn.get(k_ttl(acct, rgn, name)).await.ok().flatten();
                if let Some(json_str) = raw {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&json_str) {
                        ttl_configs.insert(
                            name.clone(),
                            TtlConfigView {
                                attribute_name: v["attribute_name"]
                                    .as_str()
                                    .unwrap_or("")
                                    .to_string(),
                                enabled: v["enabled"].as_bool().unwrap_or(false),
                            },
                        );
                    }
                }
            }

            DynamodbStateView {
                tables,
                tags,
                ttl_configs,
                ..Default::default()
            }
        })
    }

    fn restore(
        &self,
        account_id: String,
        region: String,
        view: DynamodbStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move {
            let acct = &account_id;
            let rgn = &region;

            // Delete existing tables
            let existing: Vec<String> =
                conn.smembers(k_tables(acct, rgn)).await.unwrap_or_default();
            for name in &existing {
                let _: () = conn.del(k_table(acct, rgn, name)).await.unwrap_or(());
                let _: () = conn.del(k_items(acct, rgn, name)).await.unwrap_or(());
            }
            let _: () = conn.del(k_tables(acct, rgn)).await.unwrap_or(());

            // Restore tables from view — delegate to merge for the actual writes
            // but first clear tags and TTL
            for name in &existing {
                let arn = format!("arn:aws:dynamodb:{rgn}:{acct}:table/{name}");
                let arn_b64 = b64(&arn);
                let _: () = conn.del(k_tags(acct, rgn, &arn_b64)).await.unwrap_or(());
                let _: () = conn.del(k_ttl(acct, rgn, name)).await.unwrap_or(());
            }

            // Write new state
            write_view_to_redis(&mut conn, acct, rgn, view).await
        })
    }

    fn merge(
        &self,
        account_id: String,
        region: String,
        view: DynamodbStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>> {
        let mut conn = self.conn.clone();
        Box::pin(async move { write_view_to_redis(&mut conn, &account_id, &region, view).await })
    }
}

async fn write_view_to_redis(
    conn: &mut ConnectionManager,
    acct: &str,
    rgn: &str,
    view: DynamodbStateView,
) -> Result<(), StateViewError> {
    for (name, tv) in view.tables {
        let creation_ts = tv
            .creation_date_time
            .parse::<chrono::DateTime<Utc>>()
            .ok()
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| Utc::now().to_rfc3339());
        let item_count: usize = tv.items.values().map(|m| m.len()).sum();
        let stored = StoredTable {
            name: tv.name.clone(),
            arn: tv.arn,
            key_schema: tv
                .key_schema
                .into_iter()
                .map(|k| StoredKeySchemaElement {
                    attribute_name: k.attribute_name,
                    key_type: k.key_type,
                })
                .collect(),
            attribute_definitions: tv
                .attribute_definitions
                .into_iter()
                .map(|a| StoredAttributeDefinition {
                    attribute_name: a.attribute_name,
                    attribute_type: a.attribute_type,
                })
                .collect(),
            billing_mode: tv.billing_mode,
            provisioned_throughput: tv.provisioned_throughput.map(|pt| {
                StoredProvisionedThroughput {
                    read_capacity_units: pt.read_capacity_units,
                    write_capacity_units: pt.write_capacity_units,
                }
            }),
            creation_date_time: creation_ts,
            table_status: tv.table_status,
            item_count,
            hash_key_attr: tv.hash_key_attr,
            hash_key_type: tv.hash_key_type,
            range_key_attr: tv.range_key_attr,
            range_key_type: tv.range_key_type,
            stream_enabled: tv.stream_enabled,
            stream_view_type: tv.stream_view_type,
            latest_stream_arn: tv.latest_stream_arn,
            latest_stream_label: tv.latest_stream_label,
            global_secondary_indexes: tv
                .global_secondary_index
                .into_iter()
                .map(|index| StoredSecondaryIndex {
                    index_name: index.index_name,
                    key_schema: index
                        .key_schema
                        .into_iter()
                        .map(|key| StoredKeySchemaElement {
                            attribute_name: key.attribute_name,
                            key_type: key.key_type,
                        })
                        .collect(),
                    projection_type: index.projection_type,
                    non_key_attributes: index.non_key_attributes,
                })
                .collect(),
            local_secondary_indexes: tv
                .local_secondary_index
                .into_iter()
                .map(|index| StoredSecondaryIndex {
                    index_name: index.index_name,
                    key_schema: index
                        .key_schema
                        .into_iter()
                        .map(|key| StoredKeySchemaElement {
                            attribute_name: key.attribute_name,
                            key_type: key.key_type,
                        })
                        .collect(),
                    projection_type: index.projection_type,
                    non_key_attributes: index.non_key_attributes,
                })
                .collect(),
        };
        let json =
            serde_json::to_string(&stored).map_err(|e| StateViewError::Invalid(e.to_string()))?;
        conn.set::<_, _, ()>(k_table(acct, rgn, &name), json)
            .await
            .map_err(|e| StateViewError::Invalid(e.to_string()))?;
        conn.sadd::<_, _, ()>(k_tables(acct, rgn), &name)
            .await
            .map_err(|e| StateViewError::Invalid(e.to_string()))?;

        // Write items
        for (hk, range_map) in tv.items {
            for (rk, item) in range_map {
                let compound_key = format!("{hk}|{rk}");
                let item_json = serde_json::to_string(&item)
                    .map_err(|e| StateViewError::Invalid(e.to_string()))?;
                conn.hset::<_, _, _, ()>(k_items(acct, rgn, &name), &compound_key, item_json)
                    .await
                    .map_err(|e| StateViewError::Invalid(e.to_string()))?;
            }
        }
    }

    // Write tags
    for (arn, tag_views) in view.tags {
        let arn_b64 = b64(&arn);
        for tv in tag_views {
            conn.hset::<_, _, _, ()>(k_tags(acct, rgn, &arn_b64), &tv.key, &tv.value)
                .await
                .map_err(|e| StateViewError::Invalid(e.to_string()))?;
        }
    }

    // Write TTL configs
    for (table_name, ttl) in view.ttl_configs {
        let json = serde_json::to_string(&serde_json::json!({
            "attribute_name": ttl.attribute_name,
            "enabled": ttl.enabled
        }))
        .map_err(|e| StateViewError::Invalid(e.to_string()))?;
        conn.set::<_, _, ()>(k_ttl(acct, rgn, &table_name), json)
            .await
            .map_err(|e| StateViewError::Invalid(e.to_string()))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use redis::IntoConnectionInfo;

    /// Regression test for the `dynamodb-backend` Redis-db-number TODO.
    ///
    /// `from_url_with_db` is supposed to parse the supplied URL and then
    /// *override* the database number, so callers can share one Redis
    /// instance across services without encoding the db in the URL path.
    /// We mirror the production code's parsing path here so the assertion
    /// is meaningful even though we never actually connect.
    #[test]
    fn from_url_with_db_overrides_db_field() {
        let url = "redis://127.0.0.1:6379/0";
        let mut info = url.into_connection_info().expect("parse url");
        // Sanity: URL path encodes db 0.
        assert_eq!(info.redis.db, 0);
        // Production override.
        info.redis.db = 7;
        assert_eq!(info.redis.db, 7, "override must take effect");
    }

    fn table() -> super::StoredTable {
        serde_json::from_value(serde_json::json!({
            "name": "t",
            "arn": "arn:aws:dynamodb:us-east-1:123456789012:table/t",
            "key_schema": [],
            "attribute_definitions": [],
            "billing_mode": "PAY_PER_REQUEST",
            "provisioned_throughput": null,
            "creation_date_time": "2026-11-06T00:00:00Z",
            "table_status": "ACTIVE",
            "item_count": 0,
            "hash_key_attr": "pk",
            "hash_key_type": "S",
            "range_key_attr": "sk",
            "range_key_type": "S",
        }))
        .unwrap()
    }

    fn item(pairs: &[(&str, &str)]) -> super::Item {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), super::AttributeValue::S(v.to_string())))
            .collect()
    }

    /// Walk every page with `Limit` = 1, returning the `sk` of each item and
    /// the attribute names of every `LastEvaluatedKey`.
    fn walk(
        items: &[super::Item],
        sort_attr: Option<&str>,
        forward: bool,
        key_attrs: &[&str],
    ) -> (Vec<String>, Vec<Vec<String>>) {
        let table = table();
        let (mut seen, mut keys) = (Vec::new(), Vec::new());
        let mut start = None;
        for _ in 0..=items.len() {
            let page = super::paginate(
                items.to_vec(),
                &table,
                sort_attr,
                forward,
                start.as_ref(),
                Some(1),
                key_attrs,
            );
            seen.extend(page.items.iter().map(|i| match &i["sk"] {
                super::AttributeValue::S(s) => s.clone(),
                other => panic!("{other:?}"),
            }));
            match page.last_evaluated_key {
                Some(key) => {
                    let mut names: Vec<String> = key.keys().cloned().collect();
                    names.sort();
                    keys.push(names);
                    start = Some(key);
                }
                None => return (seen, keys),
            }
        }
        panic!("pagination did not terminate: {seen:?}");
    }

    /// Items sharing an index sort key are ordered by the table primary key,
    /// so paging one item at a time visits each exactly once in either
    /// direction. Before 2026-11-06 the Redis backend positioned
    /// `ExclusiveStartKey` by the index sort key alone and skipped them.
    #[test]
    fn index_pages_break_ties_with_the_table_key() {
        let items = [
            item(&[("pk", "p2"), ("sk", "b"), ("g", "x"), ("gs", "1")]),
            item(&[("pk", "p1"), ("sk", "a"), ("g", "x"), ("gs", "1")]),
            item(&[("pk", "p1"), ("sk", "c"), ("g", "x"), ("gs", "0")]),
        ];
        let key_attrs = ["pk", "sk", "g", "gs"];
        let (seen, keys) = walk(&items, Some("gs"), true, &key_attrs);
        assert_eq!(seen, ["c", "a", "b"]);
        assert!(
            keys.iter().all(|k| k == &["g", "gs", "pk", "sk"]),
            "{keys:?}"
        );
        let (seen, _) = walk(&items, Some("gs"), false, &key_attrs);
        assert_eq!(seen, ["b", "a", "c"]);
    }

    /// An index without a sort key still advances: the table primary key
    /// alone positions `ExclusiveStartKey`.
    #[test]
    fn hash_only_index_and_scan_pages_advance() {
        let items = [
            item(&[("pk", "p2"), ("sk", "b"), ("g", "x")]),
            item(&[("pk", "p1"), ("sk", "a"), ("g", "x")]),
            item(&[("pk", "p1"), ("sk", "c"), ("g", "x")]),
        ];
        let (mut seen, keys) = walk(&items, None, true, &["pk", "sk", "g"]);
        assert_eq!(keys.len(), 2);
        seen.sort();
        assert_eq!(seen, ["a", "b", "c"]);
    }

    #[test]
    fn sparse_index_membership() {
        let full = item(&[("pk", "p"), ("sk", "s"), ("g", "x"), ("gs", "1")]);
        let partial = item(&[("pk", "p"), ("sk", "s"), ("g", "x")]);
        assert!(super::in_index(&full, "g", Some("gs")));
        assert!(!super::in_index(&partial, "g", Some("gs")));
        assert!(super::in_index(&partial, "g", None));
    }

    #[test]
    fn from_url_without_db_path_starts_at_zero() {
        let info = "redis://127.0.0.1:6379"
            .into_connection_info()
            .expect("parse url");
        assert_eq!(info.redis.db, 0, "absent URL db path must default to db 0");
    }
}
