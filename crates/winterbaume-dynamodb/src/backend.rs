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

use crate::state::{DynamoDbError, DynamoDbState, QueryResult};
use crate::types::*;
use crate::views::DynamodbStateView;

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

    fn transact_get_items(
        &self,
        account_id: String,
        region: String,
        keys: Vec<(String, Item)>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Option<Item>>, DynamoDbError>> + Send>>;

    fn transact_write_items(
        &self,
        account_id: String,
        region: String,
        puts: Vec<(String, Item)>,
        deletes: Vec<(String, Item)>,
        updates: Vec<(String, Item, Vec<UpdateAction>)>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DynamoDbError>> + Send>>;

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
        puts: Vec<(String, Item)>,
        deletes: Vec<(String, Item)>,
        updates: Vec<(String, Item, Vec<UpdateAction>)>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DynamoDbError>> + Send>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let lock = state.get(&account_id, &region);
            let mut s = lock.write().await;
            s.transact_write_items(puts, deletes, updates)
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
