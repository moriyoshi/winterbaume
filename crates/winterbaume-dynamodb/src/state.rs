use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

use crate::types::*;

/// In-memory state for the DynamoDB service.
#[derive(Debug, Default)]
pub struct DynamoDbState {
    /// Tables keyed by table name.
    pub tables: HashMap<String, TableState>,
    /// Backups keyed by backup ARN.
    pub backups: HashMap<String, Backup>,
    /// Tags keyed by resource ARN.
    pub tags: HashMap<String, Vec<DynamoDbTag>>,
    /// TTL configs keyed by table name.
    pub ttl_configs: HashMap<String, TtlConfig>,
    /// Continuous backups config keyed by table name.
    pub continuous_backups: HashMap<String, ContinuousBackupsConfig>,
    /// Resource policies keyed by resource ARN.
    pub resource_policies: HashMap<String, ResourcePolicy>,
    /// Counter for generating unique backup IDs.
    pub backup_counter: u64,
    /// Global tables keyed by global table name.
    pub global_tables: HashMap<String, GlobalTableInfo>,
    /// Kinesis streaming destinations keyed by table name -> stream ARN.
    pub kinesis_destinations: HashMap<String, HashMap<String, KinesisStreamingDestination>>,
    /// Contributor Insights keyed by "table_name" or "table_name/index_name".
    pub contributor_insights: HashMap<String, ContributorInsightsConfig>,
    /// Exports keyed by export ARN.
    pub exports: HashMap<String, ExportInfo>,
    /// Export counter for generating unique export ARNs.
    pub export_counter: u64,
    /// Imports keyed by import ARN.
    pub imports: HashMap<String, ImportInfo>,
    /// Import counter for generating unique import ARNs.
    pub import_counter: u64,
}

/// State for a single DynamoDB table.
#[derive(Debug, Clone)]
pub struct TableState {
    pub table: Table,
    /// Items stored as: hash_key_value (serialized) -> range_key_value (serialized) -> Item
    /// For tables without range key: hash_key_value -> "" -> Item
    pub items: HashMap<String, HashMap<String, Item>>,
    /// Captured stream change records (appended on every write when streaming is enabled).
    pub stream_records: Vec<StreamChangeRecord>,
    /// Next sequence number to assign.
    pub stream_sequence_counter: u64,
}

/// Error type for DynamoDB operations.
#[derive(Debug, thiserror::Error)]
pub enum DynamoDbError {
    #[error("Table already exists: {0}")]
    ResourceInUse(String),

    #[error("No Hash Key specified in KeySchema")]
    NoHashKey,

    #[error("One or more parameter values were invalid: Missing the key {0} in the item")]
    MissingKey(String),

    #[error("Query condition missed key schema element")]
    QueryConditionMissedKey,

    #[error("Requested resource not found: Table: {0} not found")]
    ResourceNotFound(String),

    #[error("Backup not found: {0}")]
    BackupNotFound(String),

    #[error("Table already exists: {0}")]
    TableAlreadyExists(String),

    #[error("Source table not found: {0}")]
    SourceTableNotFound(String),

    #[error("Global table already exists: {0}")]
    GlobalTableAlreadyExists(String),

    #[error("Global table not found: {0}")]
    GlobalTableNotFound(String),

    #[error("Kinesis streaming destination not found for table {table} and stream {stream}")]
    KinesisDestinationNotFound { table: String, stream: String },

    #[error("Table not found: {0}")]
    TableNotFound(String),

    #[error("Table not found for ARN: {0}")]
    TableNotFoundByArn(String),

    #[error("Stream not found: {0}")]
    StreamNotFound(String),

    #[error("The conditional request failed")]
    ConditionalCheckFailed,

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Export not found: {0}")]
    ExportNotFound(String),

    #[error("Import not found: {0}")]
    ImportNotFound(String),
}

/// Result of a query or scan.
#[derive(Clone)]
pub struct QueryResult {
    pub items: Vec<Item>,
    pub count: usize,
    pub scanned_count: usize,
    pub last_evaluated_key: Option<Item>,
}

impl DynamoDbState {
    pub fn create_table(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        key_schema: Vec<KeySchemaElement>,
        attribute_definitions: Vec<AttributeDefinition>,
        billing_mode: Option<&str>,
        provisioned_throughput: Option<ProvisionedThroughput>,
        stream_enabled: bool,
        stream_view_type: Option<String>,
        global_secondary_indexes: Vec<GlobalSecondaryIndexDef>,
        local_secondary_indexes: Vec<LocalSecondaryIndexDef>,
    ) -> Result<&Table, DynamoDbError> {
        if self.tables.contains_key(name) {
            return Err(DynamoDbError::ResourceInUse(name.to_string()));
        }

        let billing = billing_mode.unwrap_or("PROVISIONED");

        // Extract key attrs (collect info before moving key_schema)
        let hash_key_name = key_schema
            .iter()
            .find(|k| k.key_type == "HASH")
            .map(|k| k.attribute_name.clone())
            .ok_or(DynamoDbError::NoHashKey)?;

        let range_key_name = key_schema
            .iter()
            .find(|k| k.key_type == "RANGE")
            .map(|k| k.attribute_name.clone());

        let hash_key_type = attribute_definitions
            .iter()
            .find(|a| a.attribute_name == hash_key_name)
            .map(|a| a.attribute_type.clone())
            .unwrap_or("S".to_string());

        let range_key_type = range_key_name.as_ref().and_then(|rk_name| {
            attribute_definitions
                .iter()
                .find(|a| a.attribute_name == *rk_name)
                .map(|a| a.attribute_type.clone())
        });

        let arn = format!("arn:aws:dynamodb:{region}:{account_id}:table/{name}");

        let (latest_stream_arn, latest_stream_label) = if stream_enabled {
            let label = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3f").to_string();
            let stream_arn =
                format!("arn:aws:dynamodb:{region}:{account_id}:table/{name}/stream/{label}");
            (Some(stream_arn), Some(label))
        } else {
            (None, None)
        };

        let table = Table {
            name: name.to_string(),
            arn,
            key_schema,
            attribute_definitions,
            billing_mode: billing.to_string(),
            provisioned_throughput,
            creation_date_time: Utc::now(),
            table_status: "ACTIVE".to_string(),
            item_count: 0,
            hash_key_attr: hash_key_name,
            hash_key_type,
            range_key_attr: range_key_name,
            range_key_type,
            stream_enabled,
            stream_view_type,
            latest_stream_arn,
            latest_stream_label,
            global_secondary_indexes,
            local_secondary_indexes,
        };

        self.tables.insert(
            name.to_string(),
            TableState {
                table,
                items: HashMap::new(),
                stream_records: Vec::new(),
                stream_sequence_counter: 0,
            },
        );

        Ok(&self.tables.get(name).unwrap().table)
    }

    pub fn delete_table(&mut self, name: &str) -> Result<Table, DynamoDbError> {
        match self.tables.remove(name) {
            Some(mut ts) => {
                ts.table.table_status = "DELETING".to_string();
                Ok(ts.table)
            }
            None => Err(resource_not_found(name)),
        }
    }

    pub fn describe_table(&self, name: &str) -> Result<&Table, DynamoDbError> {
        self.tables
            .get(name)
            .map(|ts| &ts.table)
            .ok_or_else(|| resource_not_found(name))
    }

    pub fn put_item(
        &mut self,
        table_name: &str,
        item: Item,
    ) -> Result<Option<Item>, DynamoDbError> {
        let ts = self
            .tables
            .get_mut(table_name)
            .ok_or_else(|| resource_not_found(table_name))?;

        let hash_key_value = serialize_key_value(
            item.get(&ts.table.hash_key_attr)
                .ok_or_else(|| DynamoDbError::MissingKey(ts.table.hash_key_attr.clone()))?,
        );

        let range_key_value = if let Some(ref rk_attr) = ts.table.range_key_attr {
            serialize_key_value(
                item.get(rk_attr)
                    .ok_or_else(|| DynamoDbError::MissingKey(rk_attr.clone()))?,
            )
        } else {
            String::new()
        };

        let hash_map = ts.items.entry(hash_key_value).or_default();
        let old = hash_map.insert(range_key_value, item.clone());

        // Update item count
        ts.table.item_count = ts.items.values().map(|m| m.len()).sum();

        // Capture stream record if streaming is enabled
        if ts.table.stream_enabled {
            let seq = ts.stream_sequence_counter;
            ts.stream_sequence_counter += 1;
            let event_name = if old.is_none() { "INSERT" } else { "MODIFY" };
            let keys = extract_key_item(&item, &ts.table);
            ts.stream_records.push(StreamChangeRecord {
                event_id: Uuid::new_v4().to_string(),
                event_name: event_name.to_string(),
                sequence_number: seq,
                approximate_creation_date_time: Utc::now().timestamp() as f64,
                keys,
                old_image: old.clone(),
                new_image: Some(item),
            });
        }

        Ok(old)
    }

    pub fn get_item(&self, table_name: &str, key: &Item) -> Result<Option<&Item>, DynamoDbError> {
        let ts = self
            .tables
            .get(table_name)
            .ok_or_else(|| resource_not_found(table_name))?;

        let hash_value = match key.get(&ts.table.hash_key_attr) {
            Some(v) => serialize_key_value(v),
            None => return Ok(None),
        };

        let range_value = if let Some(ref rk_attr) = ts.table.range_key_attr {
            match key.get(rk_attr) {
                Some(v) => serialize_key_value(v),
                None => return Ok(None),
            }
        } else {
            String::new()
        };

        Ok(ts.items.get(&hash_value).and_then(|m| m.get(&range_value)))
    }

    pub fn delete_item(
        &mut self,
        table_name: &str,
        key: &Item,
    ) -> Result<Option<Item>, DynamoDbError> {
        let ts = self
            .tables
            .get_mut(table_name)
            .ok_or_else(|| resource_not_found(table_name))?;

        let hash_value = match key.get(&ts.table.hash_key_attr) {
            Some(v) => serialize_key_value(v),
            None => return Ok(None),
        };

        let range_value = if let Some(ref rk_attr) = ts.table.range_key_attr {
            match key.get(rk_attr) {
                Some(v) => serialize_key_value(v),
                None => return Ok(None),
            }
        } else {
            String::new()
        };

        let result = ts
            .items
            .get_mut(&hash_value)
            .and_then(|m| m.remove(&range_value));

        // Clean up empty hash entries
        if let Some(m) = ts.items.get(&hash_value)
            && m.is_empty()
        {
            ts.items.remove(&hash_value);
        }

        ts.table.item_count = ts.items.values().map(|m| m.len()).sum();

        // Capture stream record if streaming is enabled and an item was actually deleted
        if ts.table.stream_enabled {
            if let Some(ref old_item) = result {
                let seq = ts.stream_sequence_counter;
                ts.stream_sequence_counter += 1;
                let keys = extract_key_item(old_item, &ts.table);
                ts.stream_records.push(StreamChangeRecord {
                    event_id: Uuid::new_v4().to_string(),
                    event_name: "REMOVE".to_string(),
                    sequence_number: seq,
                    approximate_creation_date_time: Utc::now().timestamp() as f64,
                    keys,
                    old_image: Some(old_item.clone()),
                    new_image: None,
                });
            }
        }

        Ok(result)
    }

    pub fn query(
        &self,
        table_name: &str,
        key_conditions: &Item,
        sort_key_condition: Option<&crate::types::SortKeyCondition>,
        limit: Option<usize>,
        scan_index_forward: bool,
        exclusive_start_key: Option<&Item>,
        index_name: Option<&str>,
    ) -> Result<QueryResult, DynamoDbError> {
        let ts = self
            .tables
            .get(table_name)
            .ok_or_else(|| resource_not_found(table_name))?;

        // Determine hash/range key attrs — either from the index or the table.
        let (hash_attr, range_attr) = if let Some(idx_name) = index_name {
            // Find index key schema from GSIs or LSIs
            let key_schema = ts
                .table
                .global_secondary_indexes
                .iter()
                .find(|g| g.index_name == idx_name)
                .map(|g| &g.key_schema)
                .or_else(|| {
                    ts.table
                        .local_secondary_indexes
                        .iter()
                        .find(|l| l.index_name == idx_name)
                        .map(|l| &l.key_schema)
                })
                .ok_or_else(|| {
                    DynamoDbError::InternalError(format!(
                        "The table does not have the specified index: {idx_name}"
                    ))
                })?;
            let h = key_schema
                .iter()
                .find(|k| k.key_type == "HASH")
                .map(|k| k.attribute_name.clone())
                .ok_or(DynamoDbError::NoHashKey)?;
            let r = key_schema
                .iter()
                .find(|k| k.key_type == "RANGE")
                .map(|k| k.attribute_name.clone());
            (h, r)
        } else {
            (
                ts.table.hash_key_attr.clone(),
                ts.table.range_key_attr.clone(),
            )
        };

        // Extract hash key value from key conditions
        let hash_value = match key_conditions.get(&hash_attr) {
            Some(v) => serialize_key_value(v),
            None => {
                return Err(DynamoDbError::QueryConditionMissedKey);
            }
        };

        if index_name.is_some() {
            // GSI/LSI query: scan all items and filter by the index's hash key,
            // then sort by the index's range key.
            let mut matching: Vec<Item> = Vec::new();
            for range_map in ts.items.values() {
                for item in range_map.values() {
                    if let Some(v) = item.get(&hash_attr) {
                        if serialize_key_value(v) == hash_value {
                            matching.push(item.clone());
                        }
                    }
                }
            }

            // Apply sort-key equality from `key_conditions` (when the request
            // used `sk = :v` on the index's range key).
            if let Some(rk_attr) = range_attr.as_ref()
                && let Some(rk_eq) = key_conditions.get(rk_attr)
            {
                matching.retain(|item| item.get(rk_attr) == Some(rk_eq));
            }

            // Apply sort-key range condition (`<`, `<=`, `>`, `>=`, BETWEEN,
            // begins_with) before sorting/limit.
            if let (Some(cond), Some(rk_attr)) = (sort_key_condition, range_attr.as_ref()) {
                matching.retain(|item| item.get(rk_attr).is_some_and(|v| cond.matches(v)));
            }

            // Sort by range key of the index
            if let Some(ref rk_attr) = range_attr {
                matching.sort_by(|a, b| {
                    let ak = a.get(rk_attr).map(serialize_key_value).unwrap_or_default();
                    let bk = b.get(rk_attr).map(serialize_key_value).unwrap_or_default();
                    if scan_index_forward {
                        ak.cmp(&bk)
                    } else {
                        bk.cmp(&ak)
                    }
                });
            }

            // Skip past exclusive start key
            if let Some(start_key) = exclusive_start_key {
                if let Some(ref rk_attr) = range_attr {
                    let start_range = start_key
                        .get(rk_attr)
                        .map(serialize_key_value)
                        .unwrap_or_default();
                    matching.retain(|item| {
                        let rk = item
                            .get(rk_attr)
                            .map(serialize_key_value)
                            .unwrap_or_default();
                        if scan_index_forward {
                            rk > start_range
                        } else {
                            rk < start_range
                        }
                    });
                }
            }

            let total = matching.len();
            let lim = limit.unwrap_or(total);
            let is_truncated = total > lim;
            let result_items: Vec<Item> = matching.into_iter().take(lim).collect();

            let last_evaluated_key = if is_truncated {
                result_items.last().map(|item| {
                    let mut key = Item::new();
                    // Include table primary key + index key in last_evaluated_key
                    key.insert(
                        ts.table.hash_key_attr.clone(),
                        item.get(&ts.table.hash_key_attr).cloned().unwrap(),
                    );
                    if let Some(ref rk_attr) = ts.table.range_key_attr {
                        if let Some(v) = item.get(rk_attr) {
                            key.insert(rk_attr.clone(), v.clone());
                        }
                    }
                    key.insert(hash_attr.clone(), item.get(&hash_attr).cloned().unwrap());
                    if let Some(ref rk_attr) = range_attr {
                        if let Some(v) = item.get(rk_attr) {
                            key.insert(rk_attr.clone(), v.clone());
                        }
                    }
                    key
                })
            } else {
                None
            };

            let count = result_items.len();
            return Ok(QueryResult {
                items: result_items,
                count,
                scanned_count: count,
                last_evaluated_key,
            });
        }

        // Primary table query (original path)

        // Get all items for this hash key
        let empty = HashMap::new();
        let range_items = ts.items.get(&hash_value).unwrap_or(&empty);

        let mut items: Vec<(&String, &Item)> = range_items.iter().collect();

        // Apply sort-key equality from `key_conditions` (when the request used
        // `sk = :v`, the equality lives in the regular Item alongside the
        // hash-key equality).
        if let Some(rk_attr) = ts.table.range_key_attr.as_ref()
            && let Some(rk_eq) = key_conditions.get(rk_attr)
        {
            items.retain(|&(_, item)| item.get(rk_attr) == Some(rk_eq));
        }

        // Apply sort-key range condition (`<`, `<=`, `>`, `>=`, BETWEEN,
        // begins_with) before sorting/limit.
        if let (Some(cond), Some(rk_attr)) = (sort_key_condition, ts.table.range_key_attr.as_ref())
        {
            items.retain(|&(_, item)| item.get(rk_attr).is_some_and(|v| cond.matches(v)));
        }

        // Sort by range key
        if scan_index_forward {
            items.sort_by(|a, b| a.0.cmp(b.0));
        } else {
            items.sort_by(|a, b| b.0.cmp(a.0));
        }

        // Skip past exclusive start key
        if let Some(start_key) = exclusive_start_key {
            let start_range = if let Some(ref rk_attr) = ts.table.range_key_attr {
                start_key
                    .get(rk_attr)
                    .map(serialize_key_value)
                    .unwrap_or_default()
            } else {
                String::new()
            };

            items.retain(|&(rk, _)| {
                if scan_index_forward {
                    rk > &start_range
                } else {
                    rk < &start_range
                }
            });
        }

        let total = items.len();
        let limit = limit.unwrap_or(total);
        let is_truncated = total > limit;

        let result_items: Vec<Item> = items
            .into_iter()
            .take(limit)
            .map(|(_, item)| item.clone())
            .collect();

        let last_evaluated_key = if is_truncated {
            result_items.last().map(|item| {
                let mut key = Item::new();
                key.insert(
                    ts.table.hash_key_attr.clone(),
                    item.get(&ts.table.hash_key_attr).cloned().unwrap(),
                );
                if let Some(ref rk_attr) = ts.table.range_key_attr
                    && let Some(v) = item.get(rk_attr)
                {
                    key.insert(rk_attr.clone(), v.clone());
                }
                key
            })
        } else {
            None
        };

        let count = result_items.len();
        Ok(QueryResult {
            items: result_items,
            count,
            scanned_count: count,
            last_evaluated_key,
        })
    }

    pub fn scan(
        &self,
        table_name: &str,
        limit: Option<usize>,
        exclusive_start_key: Option<&Item>,
    ) -> Result<QueryResult, DynamoDbError> {
        let ts = self
            .tables
            .get(table_name)
            .ok_or_else(|| resource_not_found(table_name))?;

        // Collect all items sorted by hash key then range key
        let mut all_items: Vec<(String, String, &Item)> = Vec::new();
        for (hk, range_map) in &ts.items {
            for (rk, item) in range_map {
                all_items.push((hk.clone(), rk.clone(), item));
            }
        }
        all_items.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));

        // Skip past exclusive start key
        if let Some(start_key) = exclusive_start_key {
            let start_hash = start_key
                .get(&ts.table.hash_key_attr)
                .map(serialize_key_value)
                .unwrap_or_default();
            let start_range = ts
                .table
                .range_key_attr
                .as_ref()
                .and_then(|rk_attr| start_key.get(rk_attr).map(serialize_key_value))
                .unwrap_or_default();

            all_items.retain(|(hk, rk, _)| (hk, rk) > (&start_hash, &start_range));
        }

        let total = all_items.len();
        let limit = limit.unwrap_or(total);
        let is_truncated = total > limit;

        let result_items: Vec<Item> = all_items
            .into_iter()
            .take(limit)
            .map(|(_, _, item)| item.clone())
            .collect();

        let last_evaluated_key = if is_truncated {
            result_items.last().map(|item| {
                let mut key = Item::new();
                key.insert(
                    ts.table.hash_key_attr.clone(),
                    item.get(&ts.table.hash_key_attr).cloned().unwrap(),
                );
                if let Some(ref rk_attr) = ts.table.range_key_attr
                    && let Some(v) = item.get(rk_attr)
                {
                    key.insert(rk_attr.clone(), v.clone());
                }
                key
            })
        } else {
            None
        };

        let count = result_items.len();
        Ok(QueryResult {
            items: result_items,
            count,
            scanned_count: count,
            last_evaluated_key,
        })
    }

    pub fn list_tables(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.tables.keys().map(|s| s.as_str()).collect();
        names.sort();
        names
    }

    pub fn update_item(
        &mut self,
        table_name: &str,
        key: &Item,
        actions: &[UpdateAction],
    ) -> Result<Option<Item>, DynamoDbError> {
        let ts = self
            .tables
            .get_mut(table_name)
            .ok_or_else(|| resource_not_found(table_name))?;

        let hash_key_value = serialize_key_value(
            key.get(&ts.table.hash_key_attr)
                .ok_or_else(|| DynamoDbError::MissingKey(ts.table.hash_key_attr.clone()))?,
        );

        let range_key_value = if let Some(ref rk_attr) = ts.table.range_key_attr {
            serialize_key_value(
                key.get(rk_attr)
                    .ok_or_else(|| DynamoDbError::MissingKey(rk_attr.clone()))?,
            )
        } else {
            String::new()
        };

        let hash_map = ts.items.entry(hash_key_value).or_default();

        // If item doesn't exist, create one with just the keys
        let is_new = !hash_map.contains_key(&range_key_value);
        if is_new {
            let mut new_item = Item::new();
            for (k, v) in key {
                new_item.insert(k.clone(), v.clone());
            }
            hash_map.insert(range_key_value.clone(), new_item);
        }

        // Snapshot old image before mutations (for stream records)
        let old_image = if ts.table.stream_enabled {
            Some(hash_map.get(&range_key_value).unwrap().clone())
        } else {
            None
        };

        {
            let item = hash_map.get_mut(&range_key_value).unwrap();
            crate::expr::apply_update_actions(item, actions);
        }

        let result = hash_map.get(&range_key_value).unwrap().clone();
        ts.table.item_count = ts
            .items
            .values()
            .map(|m: &HashMap<String, Item>| m.len())
            .sum::<usize>();

        // Capture stream record if streaming is enabled
        if ts.table.stream_enabled {
            let seq = ts.stream_sequence_counter;
            ts.stream_sequence_counter += 1;
            let event_name = if is_new { "INSERT" } else { "MODIFY" };
            let captured_keys = extract_key_item(&result, &ts.table);
            ts.stream_records.push(StreamChangeRecord {
                event_id: Uuid::new_v4().to_string(),
                event_name: event_name.to_string(),
                sequence_number: seq,
                approximate_creation_date_time: Utc::now().timestamp() as f64,
                keys: captured_keys,
                old_image: if is_new { None } else { old_image },
                new_image: Some(result.clone()),
            });
        }

        Ok(Some(result))
    }

    pub fn batch_get_item(
        &self,
        table_name: &str,
        keys: &[Item],
    ) -> Result<Vec<Item>, DynamoDbError> {
        let _ts = self
            .tables
            .get(table_name)
            .ok_or_else(|| resource_not_found(table_name))?;

        let mut results = Vec::new();
        for key in keys {
            if let Ok(Some(item)) = self.get_item(table_name, key) {
                results.push(item.clone());
            }
        }
        Ok(results)
    }

    pub fn batch_write_item(
        &mut self,
        table_name: &str,
        puts: Vec<Item>,
        deletes: Vec<Item>,
    ) -> Result<(), DynamoDbError> {
        // Verify table exists
        if !self.tables.contains_key(table_name) {
            return Err(resource_not_found(table_name));
        }

        for item in puts {
            self.put_item(table_name, item)?;
        }
        for key in &deletes {
            self.delete_item(table_name, key)?;
        }
        Ok(())
    }

    // --- Backup operations ---

    pub fn create_backup(
        &mut self,
        table_name: &str,
        backup_name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<Backup, DynamoDbError> {
        let ts = self
            .tables
            .get(table_name)
            .ok_or_else(|| table_not_found(table_name))?;

        self.backup_counter += 1;
        let backup_id = format!("{:016x}", self.backup_counter);
        let backup_arn =
            format!("arn:aws:dynamodb:{region}:{account_id}:table/{table_name}/backup/{backup_id}");

        let backup = Backup {
            backup_arn: backup_arn.clone(),
            backup_name: backup_name.to_string(),
            table_name: table_name.to_string(),
            table_arn: ts.table.arn.clone(),
            backup_status: "AVAILABLE".to_string(),
            backup_type: "USER".to_string(),
            backup_creation_date_time: Utc::now(),
            backup_size_bytes: 0,
            key_schema: ts.table.key_schema.clone(),
            provisioned_throughput: ts.table.provisioned_throughput.clone(),
            item_count: ts.table.item_count,
            table_id: backup_id.clone(),
            table_creation_date_time: ts.table.creation_date_time,
            billing_mode: ts.table.billing_mode.clone(),
            items: ts.items.clone(),
            table_snapshot: ts.table.clone(),
        };

        self.backups.insert(backup_arn, backup.clone());
        Ok(backup)
    }

    pub fn delete_backup(&mut self, backup_arn: &str) -> Result<Backup, DynamoDbError> {
        match self.backups.remove(backup_arn) {
            Some(mut backup) => {
                backup.backup_status = "DELETED".to_string();
                Ok(backup)
            }
            None => Err(DynamoDbError::BackupNotFound(backup_arn.to_string())),
        }
    }

    pub fn describe_backup(&self, backup_arn: &str) -> Result<&Backup, DynamoDbError> {
        self.backups
            .get(backup_arn)
            .ok_or_else(|| DynamoDbError::BackupNotFound(backup_arn.to_string()))
    }

    pub fn list_backups(&self, table_name: Option<&str>) -> Vec<&Backup> {
        let mut backups: Vec<&Backup> = self
            .backups
            .values()
            .filter(|b| table_name.map(|tn| b.table_name == tn).unwrap_or(true))
            .collect();
        backups.sort_by(|a, b| {
            a.backup_creation_date_time
                .cmp(&b.backup_creation_date_time)
        });
        backups
    }

    pub fn restore_table_from_backup(
        &mut self,
        target_table_name: &str,
        backup_arn: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&Table, DynamoDbError> {
        if self.tables.contains_key(target_table_name) {
            return Err(DynamoDbError::TableAlreadyExists(
                target_table_name.to_string(),
            ));
        }

        let backup = self
            .backups
            .get(backup_arn)
            .ok_or_else(|| DynamoDbError::BackupNotFound(backup_arn.to_string()))?;

        let arn = format!("arn:aws:dynamodb:{region}:{account_id}:table/{target_table_name}");
        let mut table = backup.table_snapshot.clone();
        table.name = target_table_name.to_string();
        table.arn = arn;
        table.table_status = "ACTIVE".to_string();
        table.creation_date_time = Utc::now();
        table.item_count = backup.items.values().map(|m| m.len()).sum();

        let items = backup.items.clone();

        self.tables.insert(
            target_table_name.to_string(),
            TableState {
                table,
                items,
                stream_records: Vec::new(),
                stream_sequence_counter: 0,
            },
        );

        Ok(&self.tables.get(target_table_name).unwrap().table)
    }

    pub fn restore_table_to_point_in_time(
        &mut self,
        source_table_name: &str,
        target_table_name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&Table, DynamoDbError> {
        if self.tables.contains_key(target_table_name) {
            return Err(DynamoDbError::TableAlreadyExists(
                target_table_name.to_string(),
            ));
        }

        let source = self
            .tables
            .get(source_table_name)
            .ok_or_else(|| DynamoDbError::SourceTableNotFound(source_table_name.to_string()))?;

        let arn = format!("arn:aws:dynamodb:{region}:{account_id}:table/{target_table_name}");
        let mut table = source.table.clone();
        table.name = target_table_name.to_string();
        table.arn = arn;
        table.table_status = "ACTIVE".to_string();
        table.creation_date_time = Utc::now();

        let items = source.items.clone();
        table.item_count = items.values().map(|m| m.len()).sum();

        self.tables.insert(
            target_table_name.to_string(),
            TableState {
                table,
                items,
                stream_records: Vec::new(),
                stream_sequence_counter: 0,
            },
        );

        Ok(&self.tables.get(target_table_name).unwrap().table)
    }

    // --- Tag operations ---

    pub fn tag_resource(&mut self, resource_arn: &str, tags: Vec<DynamoDbTag>) {
        let existing = self.tags.entry(resource_arn.to_string()).or_default();
        for tag in tags {
            // Replace existing tag with same key
            existing.retain(|t| t.key != tag.key);
            existing.push(tag);
        }
    }

    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(existing) = self.tags.get_mut(resource_arn) {
            existing.retain(|t| !tag_keys.contains(&t.key));
        }
    }

    pub fn list_tags_of_resource(&self, resource_arn: &str) -> Vec<&DynamoDbTag> {
        self.tags
            .get(resource_arn)
            .map(|tags| tags.iter().collect())
            .unwrap_or_default()
    }

    // --- TTL operations ---

    pub fn update_time_to_live(
        &mut self,
        table_name: &str,
        attribute_name: &str,
        enabled: bool,
    ) -> Result<TtlConfig, DynamoDbError> {
        if !self.tables.contains_key(table_name) {
            return Err(table_not_found(table_name));
        }

        let config = TtlConfig {
            attribute_name: attribute_name.to_string(),
            enabled,
        };
        self.ttl_configs
            .insert(table_name.to_string(), config.clone());
        Ok(config)
    }

    pub fn describe_time_to_live(
        &self,
        table_name: &str,
    ) -> Result<Option<&TtlConfig>, DynamoDbError> {
        if !self.tables.contains_key(table_name) {
            return Err(table_not_found(table_name));
        }
        Ok(self.ttl_configs.get(table_name))
    }

    // --- Continuous Backups / PITR ---

    pub fn update_continuous_backups(
        &mut self,
        table_name: &str,
        enabled: bool,
    ) -> Result<ContinuousBackupsConfig, DynamoDbError> {
        if !self.tables.contains_key(table_name) {
            return Err(table_not_found(table_name));
        }
        let config = ContinuousBackupsConfig {
            point_in_time_recovery_enabled: enabled,
        };
        self.continuous_backups
            .insert(table_name.to_string(), config.clone());
        Ok(config)
    }

    pub fn describe_continuous_backups(
        &self,
        table_name: &str,
    ) -> Result<Option<&ContinuousBackupsConfig>, DynamoDbError> {
        if !self.tables.contains_key(table_name) {
            return Err(table_not_found(table_name));
        }
        Ok(self.continuous_backups.get(table_name))
    }

    // --- Resource Policy ---

    pub fn put_resource_policy(
        &mut self,
        resource_arn: &str,
        policy: &str,
    ) -> Result<String, DynamoDbError> {
        let revision_id = uuid::Uuid::new_v4().to_string();
        self.resource_policies.insert(
            resource_arn.to_string(),
            ResourcePolicy {
                policy: policy.to_string(),
                revision_id: revision_id.clone(),
            },
        );
        Ok(revision_id)
    }

    pub fn get_resource_policy(
        &self,
        resource_arn: &str,
    ) -> Result<Option<&ResourcePolicy>, DynamoDbError> {
        // Check if the ARN corresponds to a known table
        Ok(self.resource_policies.get(resource_arn))
    }

    pub fn delete_resource_policy(&mut self, resource_arn: &str) -> Result<String, DynamoDbError> {
        self.resource_policies.remove(resource_arn);
        let revision_id = uuid::Uuid::new_v4().to_string();
        Ok(revision_id)
    }

    // --- Global Tables (v1 API) ---

    pub fn create_global_table(
        &mut self,
        global_table_name: &str,
        replication_group: Vec<String>,
        account_id: &str,
    ) -> Result<&GlobalTableInfo, DynamoDbError> {
        if self.global_tables.contains_key(global_table_name) {
            return Err(DynamoDbError::GlobalTableAlreadyExists(
                global_table_name.to_string(),
            ));
        }

        let arn = format!("arn:aws:dynamodb::{account_id}:global-table/{global_table_name}");
        let info = GlobalTableInfo {
            global_table_name: global_table_name.to_string(),
            global_table_arn: arn,
            global_table_status: "ACTIVE".to_string(),
            creation_date_time: Utc::now(),
            replication_group,
        };
        self.global_tables
            .insert(global_table_name.to_string(), info);
        Ok(self.global_tables.get(global_table_name).unwrap())
    }

    pub fn describe_global_table(
        &self,
        global_table_name: &str,
    ) -> Result<&GlobalTableInfo, DynamoDbError> {
        self.global_tables
            .get(global_table_name)
            .ok_or_else(|| DynamoDbError::GlobalTableNotFound(global_table_name.to_string()))
    }

    pub fn update_global_table(
        &mut self,
        global_table_name: &str,
        replicas_to_create: Vec<String>,
        replicas_to_delete: Vec<String>,
    ) -> Result<&GlobalTableInfo, DynamoDbError> {
        let gt = self
            .global_tables
            .get_mut(global_table_name)
            .ok_or_else(|| DynamoDbError::GlobalTableNotFound(global_table_name.to_string()))?;
        gt.replication_group
            .retain(|r| !replicas_to_delete.contains(r));
        gt.replication_group.extend(replicas_to_create);
        Ok(self.global_tables.get(global_table_name).unwrap())
    }

    pub fn list_global_tables(&self) -> Vec<&GlobalTableInfo> {
        let mut gts: Vec<&GlobalTableInfo> = self.global_tables.values().collect();
        gts.sort_by_key(|g| &g.global_table_name);
        gts
    }

    // --- Kinesis Streaming Destinations ---

    pub fn enable_kinesis_streaming_destination(
        &mut self,
        table_name: &str,
        stream_arn: &str,
        precision: Option<&str>,
    ) -> Result<KinesisStreamingDestination, DynamoDbError> {
        if !self.tables.contains_key(table_name) {
            return Err(table_not_found(table_name));
        }
        let dest = KinesisStreamingDestination {
            stream_arn: stream_arn.to_string(),
            destination_status: "ACTIVE".to_string(),
            approximate_creation_date_time_precision: precision.map(|s| s.to_string()),
        };
        self.kinesis_destinations
            .entry(table_name.to_string())
            .or_default()
            .insert(stream_arn.to_string(), dest.clone());
        Ok(dest)
    }

    pub fn disable_kinesis_streaming_destination(
        &mut self,
        table_name: &str,
        stream_arn: &str,
    ) -> Result<KinesisStreamingDestination, DynamoDbError> {
        if !self.tables.contains_key(table_name) {
            return Err(table_not_found(table_name));
        }
        let dests = self
            .kinesis_destinations
            .get_mut(table_name)
            .and_then(|m| m.get_mut(stream_arn));
        match dests {
            Some(d) => {
                d.destination_status = "DISABLED".to_string();
                Ok(d.clone())
            }
            None => Ok(KinesisStreamingDestination {
                stream_arn: stream_arn.to_string(),
                destination_status: "DISABLED".to_string(),
                approximate_creation_date_time_precision: None,
            }),
        }
    }

    pub fn describe_kinesis_streaming_destinations(
        &self,
        table_name: &str,
    ) -> Result<Vec<&KinesisStreamingDestination>, DynamoDbError> {
        if !self.tables.contains_key(table_name) {
            return Err(table_not_found(table_name));
        }
        Ok(self
            .kinesis_destinations
            .get(table_name)
            .map(|m| m.values().collect())
            .unwrap_or_default())
    }

    pub fn update_kinesis_streaming_destination(
        &mut self,
        table_name: &str,
        stream_arn: &str,
        precision: Option<&str>,
    ) -> Result<KinesisStreamingDestination, DynamoDbError> {
        if !self.tables.contains_key(table_name) {
            return Err(table_not_found(table_name));
        }
        let dest = self
            .kinesis_destinations
            .get_mut(table_name)
            .and_then(|m| m.get_mut(stream_arn));
        match dest {
            Some(d) => {
                if let Some(p) = precision {
                    d.approximate_creation_date_time_precision = Some(p.to_string());
                }
                Ok(d.clone())
            }
            None => Err(DynamoDbError::KinesisDestinationNotFound {
                table: table_name.to_string(),
                stream: stream_arn.to_string(),
            }),
        }
    }

    // --- Contributor Insights ---

    pub fn update_contributor_insights(
        &mut self,
        table_name: &str,
        index_name: Option<&str>,
        mode: &str,
    ) -> Result<ContributorInsightsConfig, DynamoDbError> {
        if !self.tables.contains_key(table_name) {
            return Err(table_not_found(table_name));
        }
        let key = match index_name {
            Some(idx) => format!("{table_name}/{idx}"),
            None => table_name.to_string(),
        };
        let status = if mode == "ENABLE" {
            "ENABLED"
        } else {
            "DISABLED"
        };
        let config = ContributorInsightsConfig {
            table_name: table_name.to_string(),
            index_name: index_name.map(|s| s.to_string()),
            status: status.to_string(),
            last_update_date_time: Utc::now(),
        };
        self.contributor_insights.insert(key, config.clone());
        Ok(config)
    }

    pub fn describe_contributor_insights(
        &self,
        table_name: &str,
        index_name: Option<&str>,
    ) -> Result<Option<&ContributorInsightsConfig>, DynamoDbError> {
        if !self.tables.contains_key(table_name) {
            return Err(table_not_found(table_name));
        }
        let key = match index_name {
            Some(idx) => format!("{table_name}/{idx}"),
            None => table_name.to_string(),
        };
        Ok(self.contributor_insights.get(&key))
    }

    pub fn list_contributor_insights(
        &self,
        table_name: Option<&str>,
    ) -> Vec<&ContributorInsightsConfig> {
        let mut configs: Vec<&ContributorInsightsConfig> = self
            .contributor_insights
            .values()
            .filter(|c| table_name.map(|tn| c.table_name == tn).unwrap_or(true))
            .collect();
        configs.sort_by_key(|c| (&c.table_name, &c.index_name));
        configs
    }

    // --- ExportTableToPointInTime ---

    pub fn export_table_to_point_in_time(
        &mut self,
        table_arn: &str,
        s3_bucket: Option<&str>,
        s3_prefix: Option<&str>,
        export_format: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<ExportInfo, DynamoDbError> {
        // Verify the table exists by checking if any table has this ARN
        let table_exists = self.tables.values().any(|ts| ts.table.arn == table_arn);
        if !table_exists {
            return Err(DynamoDbError::TableNotFoundByArn(table_arn.to_string()));
        }

        self.export_counter += 1;
        let export_id = format!("{:016x}", self.export_counter);
        let export_arn = format!("arn:aws:dynamodb:{region}:{account_id}:table/export/{export_id}");

        let info = ExportInfo {
            export_arn: export_arn.clone(),
            table_arn: table_arn.to_string(),
            export_status: "IN_PROGRESS".to_string(),
            start_time: Utc::now(),
            end_time: None,
            export_format: export_format.unwrap_or("DYNAMODB_JSON").to_string(),
            s3_bucket: s3_bucket.map(|s| s.to_string()),
            s3_prefix: s3_prefix.map(|s| s.to_string()),
        };

        self.exports.insert(export_arn, info.clone());
        Ok(info)
    }

    pub fn describe_export(&self, export_arn: &str) -> Result<&ExportInfo, DynamoDbError> {
        self.exports
            .get(export_arn)
            .ok_or_else(|| DynamoDbError::ExportNotFound(export_arn.to_string()))
    }

    pub fn list_exports(&self) -> Vec<&ExportInfo> {
        self.exports.values().collect()
    }

    pub fn import_table(
        &mut self,
        table_name: &str,
        account_id: &str,
        region: &str,
        s3_bucket_source: Option<&str>,
        input_format: Option<&str>,
    ) -> ImportInfo {
        self.import_counter += 1;
        let import_id = format!("{:016x}", self.import_counter);
        let table_arn = format!("arn:aws:dynamodb:{region}:{account_id}:table/{table_name}");
        let import_arn =
            format!("arn:aws:dynamodb:{region}:{account_id}:table/{table_name}/import/{import_id}");

        let info = ImportInfo {
            import_arn: import_arn.clone(),
            table_arn: Some(table_arn),
            import_status: "IN_PROGRESS".to_string(),
            start_time: Utc::now(),
            end_time: None,
            s3_bucket_source: s3_bucket_source.map(|s| s.to_string()),
            input_format: input_format.map(|s| s.to_string()),
        };

        self.imports.insert(import_arn, info.clone());
        info
    }

    pub fn describe_import(&self, import_arn: &str) -> Result<&ImportInfo, DynamoDbError> {
        self.imports
            .get(import_arn)
            .ok_or_else(|| DynamoDbError::ImportNotFound(import_arn.to_string()))
    }

    pub fn list_imports(&self) -> Vec<&ImportInfo> {
        self.imports.values().collect()
    }

    // --- UpdateTable ---

    pub fn update_table(
        &mut self,
        table_name: &str,
        billing_mode: Option<&str>,
        provisioned_throughput: Option<ProvisionedThroughput>,
    ) -> Result<&Table, DynamoDbError> {
        let ts = self
            .tables
            .get_mut(table_name)
            .ok_or_else(|| resource_not_found(table_name))?;

        if let Some(bm) = billing_mode {
            ts.table.billing_mode = bm.to_string();
        }
        if let Some(pt) = provisioned_throughput {
            ts.table.provisioned_throughput = Some(pt);
        }

        Ok(&ts.table)
    }

    /// Enable or disable DynamoDB Streams on an existing table.
    pub fn update_stream_specification(
        &mut self,
        table_name: &str,
        account_id: &str,
        region: &str,
        stream_enabled: bool,
        stream_view_type: Option<String>,
    ) -> Result<&Table, DynamoDbError> {
        let ts = self
            .tables
            .get_mut(table_name)
            .ok_or_else(|| resource_not_found(table_name))?;

        ts.table.stream_enabled = stream_enabled;
        ts.table.stream_view_type = stream_view_type;
        if stream_enabled && ts.table.latest_stream_arn.is_none() {
            let label = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3f").to_string();
            ts.table.latest_stream_arn = Some(format!(
                "arn:aws:dynamodb:{region}:{account_id}:table/{table_name}/stream/{label}"
            ));
            ts.table.latest_stream_label = Some(label);
        }

        Ok(&ts.table)
    }

    /// List all streams, optionally filtered by table name.
    pub fn list_streams(&self, table_name_filter: Option<&str>) -> Vec<StreamSummary> {
        self.tables
            .values()
            .filter(|ts| {
                ts.table.stream_enabled
                    && ts.table.latest_stream_arn.is_some()
                    && table_name_filter.is_none_or(|tn| ts.table.name == tn)
            })
            .map(|ts| StreamSummary {
                stream_arn: ts.table.latest_stream_arn.clone().unwrap(),
                stream_label: ts.table.latest_stream_label.clone().unwrap_or_default(),
                table_name: ts.table.name.clone(),
                key_schema: ts.table.key_schema.clone(),
                stream_view_type: ts
                    .table
                    .stream_view_type
                    .clone()
                    .unwrap_or_else(|| "NEW_IMAGE".to_string()),
            })
            .collect()
    }

    /// Look up a stream by its ARN.
    pub fn describe_stream_by_arn(&self, stream_arn: &str) -> Option<StreamSummary> {
        self.tables.values().find_map(|ts| {
            if ts.table.latest_stream_arn.as_deref() == Some(stream_arn) {
                Some(StreamSummary {
                    stream_arn: stream_arn.to_string(),
                    stream_label: ts.table.latest_stream_label.clone().unwrap_or_default(),
                    table_name: ts.table.name.clone(),
                    key_schema: ts.table.key_schema.clone(),
                    stream_view_type: ts
                        .table
                        .stream_view_type
                        .clone()
                        .unwrap_or_else(|| "NEW_IMAGE".to_string()),
                })
            } else {
                None
            }
        })
    }

    // --- Transact operations ---

    pub fn transact_get_items(
        &self,
        keys: &[(String, Item)],
    ) -> Result<Vec<Option<Item>>, DynamoDbError> {
        let mut results = Vec::new();
        for (table_name, key) in keys {
            match self.get_item(table_name, key) {
                Ok(Some(item)) => results.push(Some(item.clone())),
                Ok(None) => results.push(None),
                Err(e) => return Err(e),
            }
        }
        Ok(results)
    }

    pub fn transact_write_items(
        &mut self,
        puts: Vec<(String, Item)>,
        deletes: Vec<(String, Item)>,
        updates: Vec<(String, Item, Vec<UpdateAction>)>,
    ) -> Result<(), DynamoDbError> {
        // Verify all tables exist first
        for (table_name, _) in &puts {
            if !self.tables.contains_key(table_name.as_str()) {
                return Err(resource_not_found(table_name));
            }
        }
        for (table_name, _) in &deletes {
            if !self.tables.contains_key(table_name.as_str()) {
                return Err(resource_not_found(table_name));
            }
        }
        for (table_name, _, _) in &updates {
            if !self.tables.contains_key(table_name.as_str()) {
                return Err(resource_not_found(table_name));
            }
        }

        for (table_name, item) in puts {
            self.put_item(&table_name, item)?;
        }
        for (table_name, key) in &deletes {
            self.delete_item(table_name, key)?;
        }
        for (table_name, key, actions) in &updates {
            self.update_item(table_name, key, actions)?;
        }
        Ok(())
    }

    // --- Stream record retrieval ---

    /// Return stream records for the table that owns the given `stream_arn`,
    /// with sequence numbers >= `next_sequence_number`.
    /// Returns `(records, stream_summary)` so callers can validate the stream.
    pub fn get_stream_records(
        &self,
        stream_arn: &str,
        next_sequence_number: u64,
        limit: Option<usize>,
    ) -> Result<(&[StreamChangeRecord], StreamSummary), DynamoDbError> {
        let summary = self
            .describe_stream_by_arn(stream_arn)
            .ok_or_else(|| DynamoDbError::StreamNotFound(stream_arn.to_string()))?;

        let ts = self
            .tables
            .get(&summary.table_name)
            .ok_or_else(|| resource_not_found(&summary.table_name))?;

        let all = &ts.stream_records;
        // Records are appended in order; find the first with seq >= next_sequence_number
        let start = all.partition_point(|r| r.sequence_number < next_sequence_number);
        let slice = &all[start..];
        let slice = if let Some(n) = limit {
            &slice[..n.min(slice.len())]
        } else {
            slice
        };
        Ok((slice, summary))
    }
}

pub(crate) fn resource_not_found(table_name: &str) -> DynamoDbError {
    DynamoDbError::ResourceNotFound(table_name.to_string())
}

fn table_not_found(table_name: &str) -> DynamoDbError {
    DynamoDbError::TableNotFound(table_name.to_string())
}

/// Extract only the key attributes from an item, for use in stream records.
fn extract_key_item(item: &Item, table: &Table) -> Item {
    let mut keys = Item::new();
    if let Some(v) = item.get(&table.hash_key_attr) {
        keys.insert(table.hash_key_attr.clone(), v.clone());
    }
    if let Some(ref rk_attr) = table.range_key_attr {
        if let Some(v) = item.get(rk_attr) {
            keys.insert(rk_attr.clone(), v.clone());
        }
    }
    keys
}

/// Serialize a DynamoDB-typed AttributeValue to a string key for HashMap storage.
fn serialize_key_value(value: &AttributeValue) -> String {
    match value {
        AttributeValue::S(s) => format!("S:{s}"),
        AttributeValue::N(n) => format!("N:{n}"),
        AttributeValue::B(b) => format!("B:{b}"),
        _ => format!("{value:?}"),
    }
}
