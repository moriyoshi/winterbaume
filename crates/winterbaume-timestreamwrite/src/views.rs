//! Serde-compatible view types for Timestream Write state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::TimestreamWriteService;
use crate::state::TimestreamWriteState;
use crate::types::{
    BatchLoadTask, Database, MagneticStoreWriteProperties, RetentionProperties, Table,
};

/// Serializable view of the entire Timestream Write state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimestreamWriteStateView {
    #[serde(default)]
    pub databases: HashMap<String, DatabaseView>,
    /// Tables keyed by "database_name\x1ftable_name".
    #[serde(default)]
    pub tables: HashMap<String, TableView>,
    /// Batch load tasks keyed by task_id.
    #[serde(default)]
    pub batch_load_tasks: HashMap<String, BatchLoadTaskView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseView {
    pub database_name: String,
    pub arn: String,
    pub kms_key_id: Option<String>,
    pub table_count: i64,
    /// Creation time in RFC 3339 format.
    pub creation_time: String,
    /// Last updated time in RFC 3339 format.
    pub last_updated_time: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableView {
    pub table_name: String,
    pub database_name: String,
    pub arn: String,
    pub table_status: String,
    pub memory_store_retention_period_in_hours: i64,
    pub magnetic_store_retention_period_in_days: i64,
    pub enable_magnetic_store_writes: bool,
    /// Creation time in RFC 3339 format.
    pub creation_time: String,
    /// Last updated time in RFC 3339 format.
    pub last_updated_time: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    // records are transient and not persisted
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchLoadTaskView {
    pub task_id: String,
    pub task_status: String,
    pub database_name: String,
    pub table_name: String,
    pub data_source_configuration: Value,
    pub report_configuration: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_model_configuration: Option<Value>,
    /// Creation time in RFC 3339 format.
    pub creation_time: String,
    /// Last updated time in RFC 3339 format.
    pub last_updated_time: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

fn tuple2_key(a: &str, b: &str) -> String {
    format!("{}\x1f{}", a, b)
}

fn parse_tuple2(key: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = key.splitn(2, '\x1f').collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

fn parse_datetime(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

// --- From internal types to view types ---

impl From<&TimestreamWriteState> for TimestreamWriteStateView {
    fn from(state: &TimestreamWriteState) -> Self {
        TimestreamWriteStateView {
            databases: state
                .databases
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DatabaseView {
                            database_name: v.database_name.clone(),
                            arn: v.arn.clone(),
                            kms_key_id: v.kms_key_id.clone(),
                            table_count: v.table_count,
                            creation_time: v.creation_time.to_rfc3339(),
                            last_updated_time: v.last_updated_time.to_rfc3339(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            tables: state
                .tables
                .iter()
                .map(|((db_name, tbl_name), v)| {
                    (
                        tuple2_key(db_name, tbl_name),
                        TableView {
                            table_name: v.table_name.clone(),
                            database_name: v.database_name.clone(),
                            arn: v.arn.clone(),
                            table_status: v.table_status.clone(),
                            memory_store_retention_period_in_hours: v
                                .retention_properties
                                .memory_store_retention_period_in_hours,
                            magnetic_store_retention_period_in_days: v
                                .retention_properties
                                .magnetic_store_retention_period_in_days,
                            enable_magnetic_store_writes: v
                                .magnetic_store_write_properties
                                .enable_magnetic_store_writes,
                            creation_time: v.creation_time.to_rfc3339(),
                            last_updated_time: v.last_updated_time.to_rfc3339(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            batch_load_tasks: state
                .batch_load_tasks
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        BatchLoadTaskView {
                            task_id: v.task_id.clone(),
                            task_status: v.task_status.clone(),
                            database_name: v.database_name.clone(),
                            table_name: v.table_name.clone(),
                            data_source_configuration: v.data_source_configuration.clone(),
                            report_configuration: v.report_configuration.clone(),
                            data_model_configuration: v.data_model_configuration.clone(),
                            creation_time: v.creation_time.to_rfc3339(),
                            last_updated_time: v.last_updated_time.to_rfc3339(),
                            error_message: v.error_message.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<TimestreamWriteStateView> for TimestreamWriteState {
    fn from(view: TimestreamWriteStateView) -> Self {
        let databases = view
            .databases
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    Database {
                        database_name: v.database_name,
                        arn: v.arn,
                        kms_key_id: v.kms_key_id,
                        table_count: v.table_count,
                        creation_time: parse_datetime(&v.creation_time),
                        last_updated_time: parse_datetime(&v.last_updated_time),
                        tags: v.tags,
                    },
                )
            })
            .collect();

        let tables = view
            .tables
            .into_iter()
            .filter_map(|(k, v)| {
                parse_tuple2(&k).map(|(db_name, tbl_name)| {
                    (
                        (db_name, tbl_name),
                        Table {
                            table_name: v.table_name,
                            database_name: v.database_name,
                            arn: v.arn,
                            table_status: v.table_status,
                            retention_properties: RetentionProperties {
                                memory_store_retention_period_in_hours: v
                                    .memory_store_retention_period_in_hours,
                                magnetic_store_retention_period_in_days: v
                                    .magnetic_store_retention_period_in_days,
                            },
                            magnetic_store_write_properties: MagneticStoreWriteProperties {
                                enable_magnetic_store_writes: v.enable_magnetic_store_writes,
                            },
                            creation_time: parse_datetime(&v.creation_time),
                            last_updated_time: parse_datetime(&v.last_updated_time),
                            tags: v.tags,
                            records: Vec::new(),
                        },
                    )
                })
            })
            .collect();

        let batch_load_tasks = view
            .batch_load_tasks
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    BatchLoadTask {
                        task_id: v.task_id,
                        task_status: v.task_status,
                        database_name: v.database_name,
                        table_name: v.table_name,
                        data_source_configuration: v.data_source_configuration,
                        report_configuration: v.report_configuration,
                        data_model_configuration: v.data_model_configuration,
                        creation_time: parse_datetime(&v.creation_time),
                        last_updated_time: parse_datetime(&v.last_updated_time),
                        error_message: v.error_message,
                    },
                )
            })
            .collect();

        TimestreamWriteState {
            databases,
            tables,
            batch_load_tasks,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for TimestreamWriteService {
    type StateView = TimestreamWriteStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        TimestreamWriteStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = TimestreamWriteState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            let new_state = TimestreamWriteState::from(view);
            guard.databases.extend(new_state.databases);
            guard.tables.extend(new_state.tables);
            guard.batch_load_tasks.extend(new_state.batch_load_tasks);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
