//! Pluggable storage backend for the DynamoDB mock service.
//!
//! The [`DynamoDbBackend`] trait abstracts all table and item operations so that
//! alternative implementations (e.g. Redis-backed) can be swapped in without
//! touching the protocol layer.
//!
//! The built-in [`InMemoryDynamoDbBackend`] is the default and stores everything in
//! `HashMap`s protected by an `RwLock`, partitioned by account ID and region
//! via [`BackendState`].
//!
//! # Object safety and async
//!
//! The same `Pin<Box<dyn Future>>` pattern used elsewhere in this codebase is
//! used here so that `Arc<dyn DynamoDbBackend>` is object-safe without requiring
//! the `async-trait` crate. All string parameters are passed as owned `String`s
//! so that futures can be `'static` without lifetime annotations.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{BackendState, StateViewError};

use crate::expr::Expr;
use crate::state::{DynamoDbError, DynamoDbState, QueryResult};
use crate::types::*;
use crate::views::DynamodbStateView;

/// Evaluate a condition the way DynamoDB does for a possibly absent item: an
/// absent item is an empty one.
fn evaluate_on(condition: &Expr, item: Option<&Item>) -> bool {
    match item {
        Some(item) => crate::expr::evaluate(condition, item),
        None => crate::expr::evaluate(condition, &Item::new()),
    }
}

// ---------------------------------------------------------------------------
// Trait
// ---------------------------------------------------------------------------

/// Pluggable backend for DynamoDB table and item storage.
///
/// Implementations are responsible for partitioning data by `account_id` and
/// `region` internally. All parameters are owned so the returned futures are
/// `'static`.
pub trait DynamoDbBackend: Send + Sync {
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
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>>;

    fn delete_table(
        &self,
        account_id: String,
        region: String,
        name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>>;

    fn describe_table(
        &self,
        account_id: String,
        region: String,
        name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>>;

    fn list_tables(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, DynamoDbError>> + Send>>;

    fn update_table(
        &self,
        account_id: String,
        region: String,
        name: String,
        billing_mode: Option<String>,
        provisioned_throughput: Option<ProvisionedThroughput>,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>>;

    fn update_stream_specification(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        stream_enabled: bool,
        stream_view_type: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>>;

    /// Return the settings stored beside a table ( deletion protection and
    /// server-side encryption ).
    ///
    /// The provided implementation is for backends that do not persist table
    /// settings: it reports the defaults. A backend that overrides
    /// [`Self::update_table_settings`] must override this as well, reject
    /// `delete_table` with [`crate::state::deletion_protected_error`] while
    /// deletion protection is enabled, drop the settings when the table is
    /// deleted, and carry them through `snapshot` / `restore` / `merge` as
    /// [`DynamodbStateView::table_settings`].
    fn describe_table_settings(
        &self,
        _account_id: String,
        _region: String,
        _name: String,
    ) -> Pin<Box<dyn Future<Output = Result<TableSettings, DynamoDbError>> + Send>> {
        Box::pin(async { Ok(TableSettings::default()) })
    }

    /// Apply a partial update to a table's settings and return the result.
    ///
    /// The provided implementation is for backends that do not persist table
    /// settings: it stores nothing and reports the defaults, so the requested
    /// setting is not honoured. See [`Self::describe_table_settings`].
    fn update_table_settings(
        &self,
        _account_id: String,
        _region: String,
        _name: String,
        _update: TableSettingsUpdate,
    ) -> Pin<Box<dyn Future<Output = Result<TableSettings, DynamoDbError>> + Send>> {
        Box::pin(async { Ok(TableSettings::default()) })
    }

    fn list_streams(
        &self,
        account_id: String,
        region: String,
        table_name: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<StreamSummary>, DynamoDbError>> + Send>>;

    fn describe_stream_by_arn(
        &self,
        account_id: String,
        region: String,
        stream_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<StreamSummary, DynamoDbError>> + Send>>;

    // --- Item operations ---

    fn put_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        item: Item,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Item>, DynamoDbError>> + Send>>;

    fn get_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        key: Item,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Item>, DynamoDbError>> + Send>>;

    fn delete_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        key: Item,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Item>, DynamoDbError>> + Send>>;

    fn update_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        key: Item,
        actions: Vec<UpdateAction>,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Item>, DynamoDbError>> + Send>>;

    fn batch_get_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        keys: Vec<Item>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Item>, DynamoDbError>> + Send>>;

    fn batch_write_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        puts: Vec<Item>,
        deletes: Vec<Item>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DynamoDbError>> + Send>>;

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
    ) -> Pin<Box<dyn Future<Output = Result<QueryResult, DynamoDbError>> + Send>>;

    fn scan(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        limit: Option<usize>,
        exclusive_start_key: Option<Item>,
    ) -> Pin<Box<dyn Future<Output = Result<QueryResult, DynamoDbError>> + Send>>;

    /// Scan a secondary index (`Scan` with `IndexName`).
    ///
    /// Must return only the items that belong to the sparse index (those
    /// carrying every index key attribute), unprojected; the protocol layer
    /// applies the index projection. `LastEvaluatedKey` should carry the
    /// table primary key plus the index key attributes.
    ///
    /// The default implementation is a best-effort fallback for backends
    /// that predate this method: it delegates to [`DynamoDbBackend::scan`]
    /// and drops the items absent from the index afterwards, so `Limit` is
    /// applied before the sparse filter. Backends should override it.
    fn scan_index(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        index_name: String,
        limit: Option<usize>,
        exclusive_start_key: Option<Item>,
    ) -> Pin<Box<dyn Future<Output = Result<QueryResult, DynamoDbError>> + Send>> {
        let describe = self.describe_table(account_id.clone(), region.clone(), table_name.clone());
        let scan = self.scan(account_id, region, table_name, limit, exclusive_start_key);
        Box::pin(async move {
            let table = describe.await?;
            let (hash_attr, range_attr) = crate::state::index_key_attrs(&table, &index_name)?;
            let mut result = scan.await?;
            result.items.retain(|item| {
                item.contains_key(&hash_attr)
                    && range_attr.as_ref().is_none_or(|r| item.contains_key(r))
            });
            result.count = result.items.len();
            Ok(result)
        })
    }

    fn transact_get_items(
        &self,
        account_id: String,
        region: String,
        keys: Vec<(String, Item)>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Option<Item>>, DynamoDbError>> + Send>>;

    /// Apply a transaction's actions in the caller's order. Implementations
    /// must reject two actions targeting the same item before writing
    /// anything.
    fn transact_write_items(
        &self,
        account_id: String,
        region: String,
        ops: Vec<TransactOp>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DynamoDbError>> + Send>>;

    // --- Conditional ( atomic ) writes ---
    //
    // The four methods below take the parsed `ConditionExpression` so that a
    // backend can evaluate it and apply the write as ONE atomic step: no
    // other write to the item may land between the evaluation and the write
    // ( issue #61 ). The images they return come out of that same step.
    //
    // They have provided implementations so that a backend written against
    // an earlier version of this trait keeps compiling. Those fall back to a
    // read through `get_item` followed by a separate write, which is NOT
    // atomic: two concurrent conditional writes can both pass the check.
    // They also assume, like `scan_index`, that the futures returned by the
    // other trait methods do nothing until polled. Every backend that can be
    // written to concurrently should override all four; both backends in
    // this repository do.

    /// `PutItem` with an optional condition, evaluated against the stored
    /// item ( an absent item is evaluated as an empty one ) atomically with
    /// the write. Table and key validation come before the condition.
    fn put_item_conditional(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        item: Item,
        condition: Option<Expr>,
    ) -> Pin<Box<dyn Future<Output = Result<ConditionalWrite, DynamoDbError>> + Send>> {
        let read = condition.as_ref().map(|_| {
            self.get_item(
                account_id.clone(),
                region.clone(),
                table_name.clone(),
                item.clone(),
            )
        });
        let new_item = Some(item.clone());
        let write = self.put_item(account_id, region, table_name, item);
        Box::pin(async move {
            if let (Some(condition), Some(read)) = (condition, read) {
                let existing = read.await?;
                if !evaluate_on(&condition, existing.as_ref()) {
                    return Ok(ConditionalWrite::ConditionFailed { item: existing });
                }
            }
            let old_item = write.await?;
            Ok(ConditionalWrite::Applied { old_item, new_item })
        })
    }

    /// `DeleteItem` with an optional condition. See
    /// [`Self::put_item_conditional`].
    fn delete_item_conditional(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        key: Item,
        condition: Option<Expr>,
    ) -> Pin<Box<dyn Future<Output = Result<ConditionalWrite, DynamoDbError>> + Send>> {
        let read = condition.as_ref().map(|_| {
            self.get_item(
                account_id.clone(),
                region.clone(),
                table_name.clone(),
                key.clone(),
            )
        });
        let write = self.delete_item(account_id, region, table_name, key);
        Box::pin(async move {
            if let (Some(condition), Some(read)) = (condition, read) {
                let existing = read.await?;
                if !evaluate_on(&condition, existing.as_ref()) {
                    return Ok(ConditionalWrite::ConditionFailed { item: existing });
                }
            }
            let old_item = write.await?;
            Ok(ConditionalWrite::Applied {
                old_item,
                new_item: None,
            })
        })
    }

    /// `UpdateItem` with an optional condition. See
    /// [`Self::put_item_conditional`]. The condition comes before the
    /// evaluation of the update actions.
    fn update_item_conditional(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        key: Item,
        actions: Vec<UpdateAction>,
        condition: Option<Expr>,
    ) -> Pin<Box<dyn Future<Output = Result<ConditionalWrite, DynamoDbError>> + Send>> {
        // Read even without a condition: `update_item` reports the new image
        // only.
        let read = self.get_item(
            account_id.clone(),
            region.clone(),
            table_name.clone(),
            key.clone(),
        );
        let write = self.update_item(account_id, region, table_name, key, actions);
        Box::pin(async move {
            let old_item = read.await?;
            if let Some(condition) = condition
                && !evaluate_on(&condition, old_item.as_ref())
            {
                return Ok(ConditionalWrite::ConditionFailed { item: old_item });
            }
            let new_item = write.await?;
            Ok(ConditionalWrite::Applied { old_item, new_item })
        })
    }

    /// `TransactWriteItems`: evaluate the conditions of all actions
    /// ( `ConditionCheck` included ) and, if every one holds, apply the
    /// writes in the caller's order, all in one critical section.
    ///
    /// A failed condition is not an error: the result is
    /// [`TransactWriteOutcome::Cancelled`] with one reason per action and
    /// nothing is written. As with [`Self::transact_write_items`], two
    /// actions targeting the same item must be rejected before anything is
    /// evaluated or written; here that includes a `ConditionCheck`.
    fn transact_write_items_conditional(
        &self,
        account_id: String,
        region: String,
        actions: Vec<TransactWriteAction>,
    ) -> Pin<Box<dyn Future<Output = Result<TransactWriteOutcome, DynamoDbError>> + Send>> {
        // `None` for an action without a condition.
        let reads: Vec<_> = actions
            .iter()
            .map(|action| {
                action.condition().map(|condition| {
                    (
                        condition.clone(),
                        self.get_item(
                            account_id.clone(),
                            region.clone(),
                            action.table_name().to_string(),
                            action.target().clone(),
                        ),
                    )
                })
            })
            .collect();
        let ops = actions
            .into_iter()
            .filter_map(TransactWriteAction::into_op)
            .collect();
        let write = self.transact_write_items(account_id, region, ops);
        Box::pin(async move {
            let mut reasons = Vec::with_capacity(reads.len());
            let mut cancelled = false;
            for read in reads {
                let mut reason = TransactCancellationReason::None;
                if let Some((condition, read)) = read {
                    let existing = read.await?;
                    if !evaluate_on(&condition, existing.as_ref()) {
                        cancelled = true;
                        reason =
                            TransactCancellationReason::ConditionalCheckFailed { item: existing };
                    }
                }
                reasons.push(reason);
            }
            if cancelled {
                return Ok(TransactWriteOutcome::Cancelled(reasons));
            }
            write.await?;
            Ok(TransactWriteOutcome::Committed)
        })
    }

    // --- Backup operations ---

    fn create_backup(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        backup_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Backup, DynamoDbError>> + Send>>;

    fn delete_backup(
        &self,
        account_id: String,
        region: String,
        backup_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Backup, DynamoDbError>> + Send>>;

    fn describe_backup(
        &self,
        account_id: String,
        region: String,
        backup_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Backup, DynamoDbError>> + Send>>;

    fn list_backups(
        &self,
        account_id: String,
        region: String,
        table_name: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Backup>, DynamoDbError>> + Send>>;

    fn restore_table_from_backup(
        &self,
        account_id: String,
        region: String,
        backup_arn: String,
        target_table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>>;

    fn restore_table_to_point_in_time(
        &self,
        account_id: String,
        region: String,
        source_table_name: String,
        target_table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>>;

    // --- Tag operations ---

    fn tag_resource(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
        tags: Vec<DynamoDbTag>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DynamoDbError>> + Send>>;

    fn untag_resource(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
        tag_keys: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DynamoDbError>> + Send>>;

    fn list_tags_of_resource(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DynamoDbTag>, DynamoDbError>> + Send>>;

    // --- TTL ---

    fn update_time_to_live(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        attribute_name: String,
        enabled: bool,
    ) -> Pin<Box<dyn Future<Output = Result<TtlConfig, DynamoDbError>> + Send>>;

    fn describe_time_to_live(
        &self,
        account_id: String,
        region: String,
        table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<TtlConfig>, DynamoDbError>> + Send>>;

    // --- Continuous Backups / PITR ---

    fn update_continuous_backups(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        enabled: bool,
    ) -> Pin<Box<dyn Future<Output = Result<ContinuousBackupsConfig, DynamoDbError>> + Send>>;

    fn describe_continuous_backups(
        &self,
        account_id: String,
        region: String,
        table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<ContinuousBackupsConfig>, DynamoDbError>> + Send>>;

    // --- Resource Policy ---

    fn put_resource_policy(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
        policy: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, DynamoDbError>> + Send>>;

    fn get_resource_policy(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<ResourcePolicy>, DynamoDbError>> + Send>>;

    fn delete_resource_policy(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, DynamoDbError>> + Send>>;

    // --- Global Tables (v1 API) ---

    fn create_global_table(
        &self,
        account_id: String,
        region: String,
        global_table_name: String,
        replication_group: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<GlobalTableInfo, DynamoDbError>> + Send>>;

    fn describe_global_table(
        &self,
        account_id: String,
        region: String,
        global_table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<GlobalTableInfo, DynamoDbError>> + Send>>;

    fn update_global_table(
        &self,
        account_id: String,
        region: String,
        global_table_name: String,
        replicas_to_create: Vec<String>,
        replicas_to_delete: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<GlobalTableInfo, DynamoDbError>> + Send>>;

    fn list_global_tables(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<GlobalTableInfo>, DynamoDbError>> + Send>>;

    // --- Kinesis Streaming Destinations ---

    fn enable_kinesis_streaming_destination(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        stream_arn: String,
        precision: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<KinesisStreamingDestination, DynamoDbError>> + Send>>;

    fn disable_kinesis_streaming_destination(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        stream_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<KinesisStreamingDestination, DynamoDbError>> + Send>>;

    fn describe_kinesis_streaming_destinations(
        &self,
        account_id: String,
        region: String,
        table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<KinesisStreamingDestination>, DynamoDbError>> + Send>>;

    fn update_kinesis_streaming_destination(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        stream_arn: String,
        precision: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<KinesisStreamingDestination, DynamoDbError>> + Send>>;

    // --- Contributor Insights ---

    fn update_contributor_insights(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        index_name: Option<String>,
        mode: String,
    ) -> Pin<Box<dyn Future<Output = Result<ContributorInsightsConfig, DynamoDbError>> + Send>>;

    fn describe_contributor_insights(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        index_name: Option<String>,
    ) -> Pin<
        Box<dyn Future<Output = Result<Option<ContributorInsightsConfig>, DynamoDbError>> + Send>,
    >;

    fn list_contributor_insights(
        &self,
        account_id: String,
        region: String,
        table_name: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ContributorInsightsConfig>, DynamoDbError>> + Send>>;

    // --- Export ---

    fn export_table_to_point_in_time(
        &self,
        account_id: String,
        region: String,
        table_arn: String,
        s3_bucket: Option<String>,
        s3_prefix: Option<String>,
        export_format: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<ExportInfo, DynamoDbError>> + Send>>;

    fn describe_export(
        &self,
        account_id: String,
        region: String,
        export_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<ExportInfo, DynamoDbError>> + Send>>;

    fn list_exports(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Vec<ExportInfo>> + Send>>;

    fn import_table(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        s3_bucket_source: Option<String>,
        input_format: Option<String>,
    ) -> Pin<Box<dyn Future<Output = ImportInfo> + Send>>;

    fn describe_import(
        &self,
        account_id: String,
        region: String,
        import_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<ImportInfo, DynamoDbError>> + Send>>;

    fn list_imports(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Vec<ImportInfo>> + Send>>;

    // --- Stream record retrieval ---

    /// Return stream records for the given `stream_arn` with sequence numbers
    /// >= `next_sequence_number`.
    ///
    /// Returns `(records, new_next_sequence_number)` where
    /// `new_next_sequence_number` is the sequence number to pass on the next
    /// call to continue reading.  If no records are returned the caller's
    /// current `next_sequence_number` is echoed back unchanged.
    fn get_stream_records(
        &self,
        account_id: String,
        region: String,
        stream_arn: String,
        next_sequence_number: u64,
        limit: Option<usize>,
    ) -> Pin<Box<dyn Future<Output = Result<(Vec<StreamChangeRecord>, u64), DynamoDbError>> + Send>>;

    // --- Scope discovery ---

    /// Return all `(account_id, region)` pairs that currently have state.
    fn scopes_with_state(&self) -> Vec<(String, String)>;

    // --- State snapshot / restore / merge ---

    /// Take a snapshot of the state for the given account/region.
    fn snapshot(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = DynamodbStateView> + Send>>;

    /// Replace state for the given account/region from a view.
    fn restore(
        &self,
        account_id: String,
        region: String,
        view: DynamodbStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>>;

    /// Merge a partial view into existing state (additive).
    fn merge(
        &self,
        account_id: String,
        region: String,
        view: DynamodbStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>>;
}

// ---------------------------------------------------------------------------
// In-memory implementation
// ---------------------------------------------------------------------------

/// In-memory [`DynamoDbBackend`] backed by [`DynamoDbState`], partitioned by
/// account ID and region via [`BackendState`].
pub struct InMemoryDynamoDbBackend {
    pub(crate) state: Arc<BackendState<DynamoDbState>>,
}

impl InMemoryDynamoDbBackend {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
        }
    }
}

impl Default for InMemoryDynamoDbBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamoDbBackend for InMemoryDynamoDbBackend {
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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.create_table(
                &name,
                &account_id,
                &region,
                key_schema,
                attribute_definitions,
                billing_mode.as_deref(),
                provisioned_throughput,
                stream_enabled,
                stream_view_type,
                global_secondary_indexes,
                local_secondary_indexes,
            )
            .cloned()
        })
    }

    fn delete_table(
        &self,
        account_id: String,
        region: String,
        name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.delete_table(&name)
        })
    }

    fn describe_table(
        &self,
        account_id: String,
        region: String,
        name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.describe_table(&name).cloned()
        })
    }

    fn list_tables(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.list_tables().iter().map(|n| n.to_string()).collect())
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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.update_table(&name, billing_mode.as_deref(), provisioned_throughput)
                .cloned()
        })
    }

    fn describe_table_settings(
        &self,
        account_id: String,
        region: String,
        name: String,
    ) -> Pin<Box<dyn Future<Output = Result<TableSettings, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.describe_table_settings(&name)
        })
    }

    fn update_table_settings(
        &self,
        account_id: String,
        region: String,
        name: String,
        update: TableSettingsUpdate,
    ) -> Pin<Box<dyn Future<Output = Result<TableSettings, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.update_table_settings(&name, update)
        })
    }

    fn update_stream_specification(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        stream_enabled: bool,
        stream_view_type: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.update_stream_specification(
                &table_name,
                &account_id,
                &region,
                stream_enabled,
                stream_view_type,
            )
            .cloned()
        })
    }

    fn list_streams(
        &self,
        account_id: String,
        region: String,
        table_name: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<StreamSummary>, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.list_streams(table_name.as_deref()))
        })
    }

    fn describe_stream_by_arn(
        &self,
        account_id: String,
        region: String,
        stream_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<StreamSummary, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.describe_stream_by_arn(&stream_arn)
                .ok_or_else(|| DynamoDbError::StreamNotFound(stream_arn.clone()))
        })
    }

    fn put_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        item: Item,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Item>, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.put_item(&table_name, item)
        })
    }

    fn get_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        key: Item,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Item>, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.get_item(&table_name, &key).map(|r| r.cloned())
        })
    }

    fn delete_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        key: Item,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Item>, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.delete_item(&table_name, &key)
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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.update_item(&table_name, &key, &actions)
        })
    }

    fn batch_get_item(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        keys: Vec<Item>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Item>, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.batch_get_item(&table_name, &keys)
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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.batch_write_item(&table_name, puts, deletes)
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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.query(
                &table_name,
                &key_conditions,
                sort_key_condition.as_ref(),
                limit,
                scan_index_forward,
                exclusive_start_key.as_ref(),
                index_name.as_deref(),
            )
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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.scan(&table_name, limit, exclusive_start_key.as_ref())
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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.scan_index(
                &table_name,
                &index_name,
                limit,
                exclusive_start_key.as_ref(),
            )
        })
    }

    fn transact_get_items(
        &self,
        account_id: String,
        region: String,
        keys: Vec<(String, Item)>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Option<Item>>, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.transact_get_items(&keys)
        })
    }

    fn transact_write_items(
        &self,
        account_id: String,
        region: String,
        ops: Vec<TransactOp>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.transact_write_items(ops)
        })
    }

    fn put_item_conditional(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        item: Item,
        condition: Option<Expr>,
    ) -> Pin<Box<dyn Future<Output = Result<ConditionalWrite, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            // One write lock around evaluate + apply.
            let mut s = lock.write().await;
            s.put_item_conditional(&table_name, item, condition.as_ref())
        })
    }

    fn delete_item_conditional(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        key: Item,
        condition: Option<Expr>,
    ) -> Pin<Box<dyn Future<Output = Result<ConditionalWrite, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.delete_item_conditional(&table_name, &key, condition.as_ref())
        })
    }

    fn update_item_conditional(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        key: Item,
        actions: Vec<UpdateAction>,
        condition: Option<Expr>,
    ) -> Pin<Box<dyn Future<Output = Result<ConditionalWrite, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.update_item_conditional(&table_name, &key, &actions, condition.as_ref())
        })
    }

    fn transact_write_items_conditional(
        &self,
        account_id: String,
        region: String,
        actions: Vec<TransactWriteAction>,
    ) -> Pin<Box<dyn Future<Output = Result<TransactWriteOutcome, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.transact_write_items_conditional(actions)
        })
    }

    fn create_backup(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        backup_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Backup, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.create_backup(&table_name, &backup_name, &account_id, &region)
        })
    }

    fn delete_backup(
        &self,
        account_id: String,
        region: String,
        backup_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Backup, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.delete_backup(&backup_arn)
        })
    }

    fn describe_backup(
        &self,
        account_id: String,
        region: String,
        backup_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Backup, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.describe_backup(&backup_arn).cloned()
        })
    }

    fn list_backups(
        &self,
        account_id: String,
        region: String,
        table_name: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Backup>, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.list_backups(table_name.as_deref())
                .into_iter()
                .cloned()
                .collect())
        })
    }

    fn restore_table_from_backup(
        &self,
        account_id: String,
        region: String,
        backup_arn: String,
        target_table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.restore_table_from_backup(&target_table_name, &backup_arn, &account_id, &region)
                .cloned()
        })
    }

    fn restore_table_to_point_in_time(
        &self,
        account_id: String,
        region: String,
        source_table_name: String,
        target_table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Table, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.restore_table_to_point_in_time(
                &source_table_name,
                &target_table_name,
                &account_id,
                &region,
            )
            .cloned()
        })
    }

    fn tag_resource(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
        tags: Vec<DynamoDbTag>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.tag_resource(&resource_arn, tags);
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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.untag_resource(&resource_arn, &tag_keys);
            Ok(())
        })
    }

    fn list_tags_of_resource(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DynamoDbTag>, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.list_tags_of_resource(&resource_arn)
                .into_iter()
                .cloned()
                .collect())
        })
    }

    fn update_time_to_live(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        attribute_name: String,
        enabled: bool,
    ) -> Pin<Box<dyn Future<Output = Result<TtlConfig, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.update_time_to_live(&table_name, &attribute_name, enabled)
        })
    }

    fn describe_time_to_live(
        &self,
        account_id: String,
        region: String,
        table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<TtlConfig>, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.describe_time_to_live(&table_name).map(|r| r.cloned())
        })
    }

    fn update_continuous_backups(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        enabled: bool,
    ) -> Pin<Box<dyn Future<Output = Result<ContinuousBackupsConfig, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.update_continuous_backups(&table_name, enabled)
        })
    }

    fn describe_continuous_backups(
        &self,
        account_id: String,
        region: String,
        table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<ContinuousBackupsConfig>, DynamoDbError>> + Send>>
    {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.describe_continuous_backups(&table_name)
                .map(|r| r.cloned())
        })
    }

    fn put_resource_policy(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
        policy: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.put_resource_policy(&resource_arn, &policy)
        })
    }

    fn get_resource_policy(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<ResourcePolicy>, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.get_resource_policy(&resource_arn).map(|r| r.cloned())
        })
    }

    fn delete_resource_policy(
        &self,
        account_id: String,
        region: String,
        resource_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.delete_resource_policy(&resource_arn)
        })
    }

    fn create_global_table(
        &self,
        account_id: String,
        region: String,
        global_table_name: String,
        replication_group: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<GlobalTableInfo, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.create_global_table(&global_table_name, replication_group, &account_id)
                .cloned()
        })
    }

    fn describe_global_table(
        &self,
        account_id: String,
        region: String,
        global_table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<GlobalTableInfo, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.describe_global_table(&global_table_name).cloned()
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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.update_global_table(&global_table_name, replicas_to_create, replicas_to_delete)
                .cloned()
        })
    }

    fn list_global_tables(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<GlobalTableInfo>, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.list_global_tables().into_iter().cloned().collect())
        })
    }

    fn enable_kinesis_streaming_destination(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        stream_arn: String,
        precision: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<KinesisStreamingDestination, DynamoDbError>> + Send>>
    {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.enable_kinesis_streaming_destination(&table_name, &stream_arn, precision.as_deref())
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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.disable_kinesis_streaming_destination(&table_name, &stream_arn)
        })
    }

    fn describe_kinesis_streaming_destinations(
        &self,
        account_id: String,
        region: String,
        table_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<KinesisStreamingDestination>, DynamoDbError>> + Send>>
    {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.describe_kinesis_streaming_destinations(&table_name)
                .map(|v| v.into_iter().cloned().collect())
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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.update_kinesis_streaming_destination(&table_name, &stream_arn, precision.as_deref())
        })
    }

    fn update_contributor_insights(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        index_name: Option<String>,
        mode: String,
    ) -> Pin<Box<dyn Future<Output = Result<ContributorInsightsConfig, DynamoDbError>> + Send>>
    {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.update_contributor_insights(&table_name, index_name.as_deref(), &mode)
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
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.describe_contributor_insights(&table_name, index_name.as_deref())
                .map(|r| r.cloned())
        })
    }

    fn list_contributor_insights(
        &self,
        account_id: String,
        region: String,
        table_name: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ContributorInsightsConfig>, DynamoDbError>> + Send>>
    {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            Ok(s.list_contributor_insights(table_name.as_deref())
                .into_iter()
                .cloned()
                .collect())
        })
    }

    fn export_table_to_point_in_time(
        &self,
        account_id: String,
        region: String,
        table_arn: String,
        s3_bucket: Option<String>,
        s3_prefix: Option<String>,
        export_format: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<ExportInfo, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.export_table_to_point_in_time(
                &table_arn,
                s3_bucket.as_deref(),
                s3_prefix.as_deref(),
                export_format.as_deref(),
                &account_id,
                &region,
            )
        })
    }

    fn describe_export(
        &self,
        account_id: String,
        region: String,
        export_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<ExportInfo, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.describe_export(&export_arn).cloned()
        })
    }

    fn list_exports(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Vec<ExportInfo>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.list_exports().into_iter().cloned().collect()
        })
    }

    fn import_table(
        &self,
        account_id: String,
        region: String,
        table_name: String,
        s3_bucket_source: Option<String>,
        input_format: Option<String>,
    ) -> Pin<Box<dyn Future<Output = ImportInfo> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.import_table(
                &table_name,
                &account_id,
                &region,
                s3_bucket_source.as_deref(),
                input_format.as_deref(),
            )
        })
    }

    fn describe_import(
        &self,
        account_id: String,
        region: String,
        import_arn: String,
    ) -> Pin<Box<dyn Future<Output = Result<ImportInfo, DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.describe_import(&import_arn).cloned()
        })
    }

    fn list_imports(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = Vec<ImportInfo>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            s.list_imports().into_iter().cloned().collect()
        })
    }

    fn get_stream_records(
        &self,
        account_id: String,
        region: String,
        stream_arn: String,
        next_sequence_number: u64,
        limit: Option<usize>,
    ) -> Pin<Box<dyn Future<Output = Result<(Vec<StreamChangeRecord>, u64), DynamoDbError>> + Send>>
    {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let s = lock.read().await;
            let (records, _summary) =
                s.get_stream_records(&stream_arn, next_sequence_number, limit)?;
            let new_next = records
                .last()
                .map(|r| r.sequence_number + 1)
                .unwrap_or(next_sequence_number);
            Ok((records.to_vec(), new_next))
        })
    }

    fn scopes_with_state(&self) -> Vec<(String, String)> {
        self.state.scopes_with_state()
    }

    fn snapshot(
        &self,
        account_id: String,
        region: String,
    ) -> Pin<Box<dyn Future<Output = DynamodbStateView> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let guard = lock.read().await;
            DynamodbStateView::from(&*guard)
        })
    }

    fn restore(
        &self,
        account_id: String,
        region: String,
        view: DynamodbStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut guard = lock.write().await;
            *guard = DynamoDbState::from(view);
            Ok(())
        })
    }

    fn merge(
        &self,
        account_id: String,
        region: String,
        view: DynamodbStateView,
    ) -> Pin<Box<dyn Future<Output = Result<(), StateViewError>> + Send>> {
        use crate::state::TableState;
        use crate::types::{
            ContinuousBackupsConfig, ContributorInsightsConfig, GlobalTableInfo,
            KinesisStreamingDestination, ResourcePolicy,
        };
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut guard = lock.write().await;
            for (name, table_view) in view.tables {
                guard.tables.insert(name, TableState::from(table_view));
            }
            for (arn, tags) in view.tags {
                guard
                    .tags
                    .insert(arn, tags.into_iter().map(DynamoDbTag::from).collect());
            }
            for (name, settings) in view.table_settings {
                let settings = TableSettings::from(settings);
                if settings.is_default() {
                    guard.table_settings.remove(&name);
                } else {
                    guard.table_settings.insert(name, settings);
                }
            }
            for (name, ttl) in view.ttl_configs {
                guard.ttl_configs.insert(name, TtlConfig::from(ttl));
            }
            for (name, cb) in view.continuous_backups {
                guard
                    .continuous_backups
                    .insert(name, ContinuousBackupsConfig::from(cb));
            }
            for (arn, p) in view.resource_policies {
                guard.resource_policies.insert(arn, ResourcePolicy::from(p));
            }
            for (name, gt) in view.global_tables {
                guard.global_tables.insert(name, GlobalTableInfo::from(gt));
            }
            for (table_name, dests) in view.kinesis_destinations {
                let entry = guard.kinesis_destinations.entry(table_name).or_default();
                for (stream_arn, d) in dests {
                    entry.insert(stream_arn, KinesisStreamingDestination::from(d));
                }
            }
            for (key, ci) in view.contributor_insights {
                guard
                    .contributor_insights
                    .insert(key, ContributorInsightsConfig::from(ci));
            }
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    const ACCOUNT: &str = "123456789012";
    const REGION: &str = "us-east-1";
    const TABLE: &str = "t";

    fn item(rev: i64) -> Item {
        Item::from([
            ("pk".to_string(), AttributeValue::S("k".to_string())),
            ("rev".to_string(), AttributeValue::N(rev.to_string())),
        ])
    }

    fn key() -> Item {
        Item::from([("pk".to_string(), AttributeValue::S("k".to_string()))])
    }

    /// `rev = 0`
    fn rev_is_zero() -> Expr {
        let values = HashMap::from([(":zero".to_string(), AttributeValue::N("0".to_string()))]);
        crate::expr::parse_expression("rev = :zero", &HashMap::new(), &values).unwrap()
    }

    /// `SET rev = 1`
    fn set_rev_to_one() -> Vec<UpdateAction> {
        let values = HashMap::from([(":one".to_string(), AttributeValue::N("1".to_string()))]);
        crate::expr::parse_update_expression("SET rev = :one", &HashMap::new(), &values).unwrap()
    }

    async fn backend_with_item() -> InMemoryDynamoDbBackend {
        backend_with_item_and_stream(false).await
    }

    async fn backend_with_item_and_stream(stream_enabled: bool) -> InMemoryDynamoDbBackend {
        let backend = InMemoryDynamoDbBackend::new();
        backend
            .create_table(
                ACCOUNT.to_string(),
                REGION.to_string(),
                TABLE.to_string(),
                vec![KeySchemaElement {
                    attribute_name: "pk".to_string(),
                    key_type: "HASH".to_string(),
                }],
                vec![AttributeDefinition {
                    attribute_name: "pk".to_string(),
                    attribute_type: "S".to_string(),
                }],
                Some("PAY_PER_REQUEST".to_string()),
                None,
                stream_enabled,
                stream_enabled.then(|| "NEW_AND_OLD_IMAGES".to_string()),
                vec![],
                vec![],
            )
            .await
            .unwrap();
        backend
            .put_item(
                ACCOUNT.to_string(),
                REGION.to_string(),
                TABLE.to_string(),
                item(0),
            )
            .await
            .unwrap();
        backend
    }

    /// Let every spawned task run until it is parked.
    async fn settle() {
        for _ in 0..16 {
            tokio::task::yield_now().await;
        }
    }

    /// Atomicity without relying on timing ( issue #61 ).
    ///
    /// The state lock is the seam. `tokio::sync::RwLock` serves its waiters
    /// in FIFO order, so on a single-threaded runtime the interleaving below
    /// is fixed: while the test holds the lock, a conditional write `A`
    /// ( condition `rev = 0` ) queues first and a competing unconditional
    /// `Put` of `rev = 5` queues second.
    ///
    /// An atomic `A` evaluates its condition and writes in its one turn, the
    /// competing put then runs after it, and the item ends as the put left
    /// it. A check-then-act `A` only reads in its turn; its write queues
    /// behind the competing put and is then applied to an item whose `rev`
    /// is 5, although the condition `rev = 0` no longer holds — the final
    /// item is `A`'s.
    #[tokio::test]
    async fn competing_write_cannot_land_between_condition_and_write() {
        type Write = Pin<Box<dyn Future<Output = bool> + Send>>;
        let args = || (ACCOUNT.to_string(), REGION.to_string(), TABLE.to_string());
        let conditional_writes: [(&str, fn(&InMemoryDynamoDbBackend) -> Write); 4] = [
            ("UpdateItem", |b| {
                let (a, r, t) = (ACCOUNT.to_string(), REGION.to_string(), TABLE.to_string());
                let w = b.update_item_conditional(
                    a,
                    r,
                    t,
                    key(),
                    set_rev_to_one(),
                    Some(rev_is_zero()),
                );
                Box::pin(async move { matches!(w.await, Ok(ConditionalWrite::Applied { .. })) })
            }),
            ("PutItem", |b| {
                let (a, r, t) = (ACCOUNT.to_string(), REGION.to_string(), TABLE.to_string());
                let w = b.put_item_conditional(a, r, t, item(1), Some(rev_is_zero()));
                Box::pin(async move { matches!(w.await, Ok(ConditionalWrite::Applied { .. })) })
            }),
            ("DeleteItem", |b| {
                let (a, r, t) = (ACCOUNT.to_string(), REGION.to_string(), TABLE.to_string());
                let w = b.delete_item_conditional(a, r, t, key(), Some(rev_is_zero()));
                Box::pin(async move { matches!(w.await, Ok(ConditionalWrite::Applied { .. })) })
            }),
            ("TransactWriteItems", |b| {
                let w = b.transact_write_items_conditional(
                    ACCOUNT.to_string(),
                    REGION.to_string(),
                    vec![TransactWriteAction::Update {
                        table_name: TABLE.to_string(),
                        key: key(),
                        actions: set_rev_to_one(),
                        condition: Some(rev_is_zero()),
                    }],
                );
                Box::pin(async move { matches!(w.await, Ok(TransactWriteOutcome::Committed)) })
            }),
        ];
        let mut interleaved = Vec::new();
        for (name, conditional_write) in conditional_writes {
            let backend = backend_with_item().await;
            let lock = backend.state.get(ACCOUNT, REGION);
            let gate = lock.write().await;

            let first = tokio::spawn(conditional_write(&backend));
            settle().await;
            let (a, r, t) = args();
            let second = tokio::spawn(backend.put_item(a, r, t, item(5)));
            settle().await;
            drop(gate);

            assert!(
                first.await.unwrap(),
                "{name}: the condition held when its turn came"
            );
            second.await.unwrap().unwrap();

            let (a, r, t) = args();
            let stored = backend.get_item(a, r, t, key()).await.unwrap();
            // The competing put must come after the whole conditional write.
            if stored != Some(item(5)) {
                interleaved.push((name, stored));
            }
        }
        assert!(
            interleaved.is_empty(),
            "a competing put landed between the condition check and the write of: {interleaved:?}"
        );
    }

    /// A stream record is emitted once per applied write and never for a
    /// write whose condition failed or whose transaction was cancelled.
    #[tokio::test]
    async fn stream_records_follow_applied_writes_only() {
        let backend = backend_with_item_and_stream(true).await;
        let args = || (ACCOUNT.to_string(), REGION.to_string(), TABLE.to_string());
        let events = || async {
            let lock = backend.state.get(ACCOUNT, REGION);
            let state = lock.read().await;
            state.tables[TABLE]
                .stream_records
                .iter()
                .map(|r| r.event_name.clone())
                .collect::<Vec<_>>()
        };
        // The seeding put.
        assert_eq!(events().await, ["INSERT"]);

        // Applied: `rev = 0` holds.
        let (a, r, t) = args();
        let applied = backend
            .update_item_conditional(a, r, t, key(), set_rev_to_one(), Some(rev_is_zero()))
            .await
            .unwrap();
        assert_eq!(
            applied,
            ConditionalWrite::Applied {
                old_item: Some(item(0)),
                new_item: Some(item(1)),
            }
        );
        assert_eq!(events().await, ["INSERT", "MODIFY"]);

        // Rejected from here on: `rev` is 1 now.
        let rejected = ConditionalWrite::ConditionFailed {
            item: Some(item(1)),
        };
        let (a, r, t) = args();
        let update = backend
            .update_item_conditional(a, r, t, key(), set_rev_to_one(), Some(rev_is_zero()))
            .await
            .unwrap();
        let (a, r, t) = args();
        let put = backend
            .put_item_conditional(a, r, t, item(7), Some(rev_is_zero()))
            .await
            .unwrap();
        let (a, r, t) = args();
        let delete = backend
            .delete_item_conditional(a, r, t, key(), Some(rev_is_zero()))
            .await
            .unwrap();
        assert_eq!([&update, &put, &delete], [&rejected; 3]);

        // A cancelled transaction writes none of its actions.
        let other = Item::from([("pk".to_string(), AttributeValue::S("other".to_string()))]);
        let outcome = backend
            .transact_write_items_conditional(
                ACCOUNT.to_string(),
                REGION.to_string(),
                vec![
                    TransactWriteAction::Put {
                        table_name: TABLE.to_string(),
                        item: other.clone(),
                        condition: None,
                    },
                    TransactWriteAction::ConditionCheck {
                        table_name: TABLE.to_string(),
                        key: key(),
                        condition: rev_is_zero(),
                    },
                ],
            )
            .await
            .unwrap();
        assert_eq!(
            outcome,
            TransactWriteOutcome::Cancelled(vec![
                TransactCancellationReason::None,
                TransactCancellationReason::ConditionalCheckFailed {
                    item: Some(item(1)),
                },
            ])
        );
        let (a, r, t) = args();
        assert_eq!(backend.get_item(a, r, t, other).await.unwrap(), None);
        assert_eq!(events().await, ["INSERT", "MODIFY"]);
    }
}
