use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A DynamoDB table.
#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub arn: String,
    pub key_schema: Vec<KeySchemaElement>,
    pub attribute_definitions: Vec<AttributeDefinition>,
    pub billing_mode: String,
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    pub creation_date_time: DateTime<Utc>,
    pub table_status: String,
    pub item_count: usize,
    pub hash_key_attr: String,
    pub hash_key_type: String,
    pub range_key_attr: Option<String>,
    pub range_key_type: Option<String>,
    /// Whether DynamoDB Streams is enabled for this table.
    pub stream_enabled: bool,
    /// The stream view type (NEW_IMAGE, OLD_IMAGE, NEW_AND_OLD_IMAGES, KEYS_ONLY).
    pub stream_view_type: Option<String>,
    /// The ARN of the latest stream for this table.
    pub latest_stream_arn: Option<String>,
    /// The label of the latest stream (ISO 8601 timestamp string).
    pub latest_stream_label: Option<String>,
    /// Global secondary index definitions.
    pub global_secondary_indexes: Vec<GlobalSecondaryIndexDef>,
    /// Local secondary index definitions.
    pub local_secondary_indexes: Vec<LocalSecondaryIndexDef>,
}

/// A global secondary index definition stored on the table.
#[derive(Debug, Clone)]
pub struct GlobalSecondaryIndexDef {
    pub index_name: String,
    pub key_schema: Vec<KeySchemaElement>,
    pub projection_type: String,
    pub non_key_attributes: Vec<String>,
}

/// A local secondary index definition stored on the table.
#[derive(Debug, Clone)]
pub struct LocalSecondaryIndexDef {
    pub index_name: String,
    pub key_schema: Vec<KeySchemaElement>,
    pub projection_type: String,
    pub non_key_attributes: Vec<String>,
}

/// Summary of a DynamoDB stream, returned by ListStreams / DescribeStream.
#[derive(Debug, Clone)]
pub struct StreamSummary {
    pub stream_arn: String,
    pub stream_label: String,
    pub table_name: String,
    pub key_schema: Vec<KeySchemaElement>,
    pub stream_view_type: String,
}

#[derive(Debug, Clone)]
pub struct KeySchemaElement {
    pub attribute_name: String,
    pub key_type: String, // "HASH" or "RANGE"
}

#[derive(Debug, Clone)]
pub struct AttributeDefinition {
    pub attribute_name: String,
    pub attribute_type: String, // "S", "N", "B"
}

#[derive(Debug, Clone)]
pub struct ProvisionedThroughput {
    pub read_capacity_units: i64,
    pub write_capacity_units: i64,
}

/// A DynamoDB typed attribute value.
///
/// Serializes/deserializes to DynamoDB JSON format:
/// - `S("foo")` → `{"S": "foo"}`
/// - `N("42")` → `{"N": "42"}`
/// - `Bool(true)` → `{"BOOL": true}`
/// - etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttributeValue {
    S(String),
    N(String),
    B(String), // base64-encoded bytes
    #[serde(rename = "BOOL")]
    Bool(bool),
    #[serde(rename = "NULL")]
    Null(bool),
    SS(Vec<String>),
    NS(Vec<String>),
    BS(Vec<String>),
    L(Vec<AttributeValue>),
    M(HashMap<String, AttributeValue>),
}

impl Default for AttributeValue {
    fn default() -> Self {
        AttributeValue::Null(true)
    }
}

/// A DynamoDB item (map of attribute name -> typed value).
pub type Item = HashMap<String, AttributeValue>;

/// Convert the codegen `model::AttributeValue` (struct-of-options) into the
/// in-memory tagged enum used by the store, expression evaluator, and PartiQL
/// layer. Picks the first variant in canonical DynamoDB priority order; an
/// AttributeValue with no field set falls back to `Null(true)`, mirroring the
/// existing `json_val_to_attr` fallback for malformed JSON.
impl From<crate::model::AttributeValue> for AttributeValue {
    fn from(m: crate::model::AttributeValue) -> Self {
        if let Some(v) = m.s {
            AttributeValue::S(v)
        } else if let Some(v) = m.n {
            AttributeValue::N(v)
        } else if let Some(v) = m.b {
            AttributeValue::B(v)
        } else if let Some(v) = m.b_o_o_l {
            AttributeValue::Bool(v)
        } else if let Some(v) = m.n_u_l_l {
            AttributeValue::Null(v)
        } else if let Some(v) = m.s_s {
            AttributeValue::SS(v)
        } else if let Some(v) = m.n_s {
            AttributeValue::NS(v)
        } else if let Some(v) = m.b_s {
            AttributeValue::BS(v)
        } else if let Some(v) = m.l {
            AttributeValue::L(v.into_iter().map(AttributeValue::from).collect())
        } else if let Some(v) = m.m {
            AttributeValue::M(v.into_iter().map(|(k, v)| (k, v.into())).collect())
        } else {
            AttributeValue::Null(true)
        }
    }
}

/// Convert a wire `HashMap<String, model::AttributeValue>` (the shape produced
/// by codegen for `Item`, `Key`, `ExpressionAttributeValues`, etc.) into the
/// in-memory `Item`.
pub fn item_from_wire(map: HashMap<String, crate::model::AttributeValue>) -> Item {
    map.into_iter().map(|(k, v)| (k, v.into())).collect()
}

/// Convert an `Option<HashMap<String, model::AttributeValue>>` (codegen output
/// for optional `ExpressionAttributeValues`-style fields) into a typed map,
/// substituting an empty map when absent.
pub fn attr_map_from_wire(
    map: Option<HashMap<String, crate::model::AttributeValue>>,
) -> HashMap<String, AttributeValue> {
    map.map(item_from_wire).unwrap_or_default()
}

/// A sort-key condition extracted from a Query's `KeyConditionExpression`.
///
/// DynamoDB allows at most one sort-key condition per query. Equality on the
/// sort key is represented through the regular `key_conditions: Item` channel
/// alongside the hash-key equality, so this enum only covers the operators
/// that cannot be expressed as a single equality.
#[derive(Debug, Clone, PartialEq)]
pub enum SortKeyCondition {
    LessThan(AttributeValue),
    LessThanOrEqual(AttributeValue),
    GreaterThan(AttributeValue),
    GreaterThanOrEqual(AttributeValue),
    Between(AttributeValue, AttributeValue),
    BeginsWith(AttributeValue),
}

impl SortKeyCondition {
    /// Return whether `value` (an item's sort-key attribute) satisfies this
    /// condition. Returns `false` when the item is missing the sort-key
    /// attribute or when the stored type is not comparable with the operand.
    pub fn matches(&self, value: &AttributeValue) -> bool {
        use std::cmp::Ordering;
        match self {
            SortKeyCondition::LessThan(op) => attr_cmp(value, op) == Some(Ordering::Less),
            SortKeyCondition::LessThanOrEqual(op) => {
                matches!(attr_cmp(value, op), Some(Ordering::Less | Ordering::Equal))
            }
            SortKeyCondition::GreaterThan(op) => attr_cmp(value, op) == Some(Ordering::Greater),
            SortKeyCondition::GreaterThanOrEqual(op) => matches!(
                attr_cmp(value, op),
                Some(Ordering::Greater | Ordering::Equal)
            ),
            SortKeyCondition::Between(lo, hi) => {
                matches!(
                    attr_cmp(value, lo),
                    Some(Ordering::Greater | Ordering::Equal)
                ) && matches!(attr_cmp(value, hi), Some(Ordering::Less | Ordering::Equal))
            }
            SortKeyCondition::BeginsWith(prefix) => match (value, prefix) {
                (AttributeValue::S(a), AttributeValue::S(b)) => a.starts_with(b.as_str()),
                (AttributeValue::B(a), AttributeValue::B(b)) => a.starts_with(b.as_str()),
                _ => false,
            },
        }
    }
}

/// Compare two DynamoDB-typed [`AttributeValue`]s using DynamoDB's per-type
/// ordering rules.
///
/// - Strings compare lexicographically by UTF-8 bytes.
/// - Numbers compare numerically (parsed as `f64`; non-parsable values yield
///   `None`).
/// - Binary values compare lexicographically as their stored base64 encoding.
///
/// Returns `None` for type mismatches and for unsupported types (sets, lists,
/// maps, bool, null) — sort-key conditions only apply to `S`, `N`, `B` per
/// the DynamoDB spec.
fn attr_cmp(a: &AttributeValue, b: &AttributeValue) -> Option<std::cmp::Ordering> {
    match (a, b) {
        (AttributeValue::S(x), AttributeValue::S(y)) => Some(x.cmp(y)),
        (AttributeValue::N(x), AttributeValue::N(y)) => {
            let xn: f64 = x.parse().ok()?;
            let yn: f64 = y.parse().ok()?;
            xn.partial_cmp(&yn)
        }
        (AttributeValue::B(x), AttributeValue::B(y)) => Some(x.cmp(y)),
        _ => None,
    }
}

/// A single action within an `UpdateExpression`.
///
/// Captures the four DynamoDB update clauses (`SET`, `REMOVE`, `ADD`,
/// `DELETE`) at a granularity that lets the apply layer evaluate
/// `list_append`, `if_not_exists`, dotted paths, and set-typed `ADD`/`DELETE`
/// without re-parsing strings.
///
/// Paths are stored as a non-empty list of segments. A single-element path
/// is a top-level attribute; longer paths address nested map fields.
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateAction {
    /// `SET <path> = <value>`.
    SetValue {
        path: Vec<String>,
        value: AttributeValue,
    },
    /// `SET <path> = <path> + :delta` or `... - :delta` (delta pre-negated
    /// for subtraction). The current value is read at apply time and treated
    /// as 0 if missing.
    SetArithmetic {
        path: Vec<String>,
        delta: AttributeValue,
    },
    /// `SET <path> = list_append(<path>, :value)`. Only same-path appends
    /// are supported; if the existing attribute is absent, an empty list is
    /// used as the source.
    SetListAppend {
        path: Vec<String>,
        value: AttributeValue,
    },
    /// `SET <path> = if_not_exists(<path>, :value)`. Only same-path
    /// short-circuits are supported.
    SetIfNotExists {
        path: Vec<String>,
        value: AttributeValue,
    },
    /// `REMOVE <path>`.
    Remove(Vec<String>),
    /// `ADD <attr> :v`. The action is polymorphic at apply time:
    /// - numeric `N` value with missing/`N` current → numeric increment
    /// - `SS` / `NS` / `BS` value with missing/matching current → set union
    Add(String, AttributeValue),
    /// `DELETE <attr> :v` — set difference on a top-level set attribute
    /// (`SS` / `NS` / `BS`). No-op when the attribute is missing.
    Delete(String, AttributeValue),
}

/// A captured stream change record from a DynamoDB write operation.
#[derive(Debug, Clone)]
pub struct StreamChangeRecord {
    /// Unique event identifier.
    pub event_id: String,
    /// "INSERT", "MODIFY", or "REMOVE".
    pub event_name: String,
    /// Monotonically increasing sequence number within the table's stream.
    pub sequence_number: u64,
    /// Unix epoch timestamp (seconds, as f64) when the event occurred.
    pub approximate_creation_date_time: f64,
    /// Key attributes of the affected item.
    pub keys: Item,
    /// Full image before the change (None for INSERT).
    pub old_image: Option<Item>,
    /// Full image after the change (None for REMOVE).
    pub new_image: Option<Item>,
}

/// A DynamoDB backup.
#[derive(Debug, Clone)]
pub struct Backup {
    pub backup_arn: String,
    pub backup_name: String,
    pub table_name: String,
    pub table_arn: String,
    pub backup_status: String,
    pub backup_type: String,
    pub backup_creation_date_time: DateTime<Utc>,
    pub backup_size_bytes: i64,
    pub key_schema: Vec<KeySchemaElement>,
    pub provisioned_throughput: Option<ProvisionedThroughput>,
    pub item_count: usize,
    pub table_id: String,
    pub table_creation_date_time: DateTime<Utc>,
    pub billing_mode: String,
    /// Snapshot of items at backup time
    pub items: HashMap<String, HashMap<String, Item>>,
    /// Snapshot of table definition at backup time
    pub table_snapshot: Table,
}

/// A DynamoDB tag.
#[derive(Debug, Clone)]
pub struct DynamoDbTag {
    pub key: String,
    pub value: String,
}

/// TTL configuration for a table.
#[derive(Debug, Clone)]
pub struct TtlConfig {
    pub attribute_name: String,
    pub enabled: bool,
}

/// Continuous backups / PITR configuration for a table.
#[derive(Debug, Clone)]
pub struct ContinuousBackupsConfig {
    pub point_in_time_recovery_enabled: bool,
}

/// Resource policy for a table.
#[derive(Debug, Clone)]
pub struct ResourcePolicy {
    pub policy: String,
    pub revision_id: String,
}

/// A DynamoDB Global Table (v1 API).
#[derive(Debug, Clone)]
pub struct GlobalTableInfo {
    pub global_table_name: String,
    pub global_table_arn: String,
    pub global_table_status: String,
    pub creation_date_time: DateTime<Utc>,
    pub replication_group: Vec<String>, // region names
}

/// Kinesis streaming destination for a table.
#[derive(Debug, Clone)]
pub struct KinesisStreamingDestination {
    pub stream_arn: String,
    pub destination_status: String,
    pub approximate_creation_date_time_precision: Option<String>,
}

/// Contributor Insights configuration for a table or GSI.
#[derive(Debug, Clone)]
pub struct ContributorInsightsConfig {
    pub table_name: String,
    pub index_name: Option<String>,
    pub status: String, // ENABLED, DISABLED, ENABLING, DISABLING
    pub last_update_date_time: DateTime<Utc>,
}

/// Export description for ExportTableToPointInTime.
#[derive(Debug, Clone)]
pub struct ExportInfo {
    pub export_arn: String,
    pub table_arn: String,
    pub export_status: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub export_format: String,
    pub s3_bucket: Option<String>,
    pub s3_prefix: Option<String>,
}

/// Import description for ImportTable.
#[derive(Debug, Clone)]
pub struct ImportInfo {
    pub import_arn: String,
    pub table_arn: Option<String>,
    pub import_status: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub s3_bucket_source: Option<String>,
    pub input_format: Option<String>,
}
