//! Serde-compatible view types for Keyspaces state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::KeyspacesService;
use crate::state::KeyspacesState;
use crate::types::{
    ClusteringKey, ColumnDefinition, FieldDefinition, Keyspace, SchemaDefinition, Table,
    UserDefinedType,
};

/// Serializable view of the entire Keyspaces state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyspacesStateView {
    #[serde(default)]
    pub keyspaces: HashMap<String, KeyspaceView>,
    /// Key: "keyspace_name/table_name"
    #[serde(default)]
    pub tables: HashMap<String, TableView>,
    /// Key: "keyspace_name/type_name"
    #[serde(default)]
    pub types: HashMap<String, UserDefinedTypeView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyspaceView {
    pub name: String,
    pub arn: String,
    pub replication_strategy: String,
    #[serde(default)]
    pub replication_regions: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub creation_timestamp: Option<String>,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableView {
    pub keyspace_name: String,
    pub table_name: String,
    pub arn: String,
    pub schema_definition: SchemaDefinitionView,
    pub capacity_mode: String,
    pub read_capacity_units: Option<i64>,
    pub write_capacity_units: Option<i64>,
    pub encryption_type: String,
    pub kms_key_identifier: Option<String>,
    pub point_in_time_recovery_enabled: bool,
    pub ttl_status: String,
    pub default_time_to_live: Option<i32>,
    #[serde(default)]
    pub comment: String,
    pub client_side_timestamps_enabled: bool,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub creation_timestamp: Option<String>,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDefinitionView {
    pub all_columns: Vec<ColumnDefinitionView>,
    pub partition_keys: Vec<String>,
    #[serde(default)]
    pub clustering_keys: Vec<ClusteringKeyView>,
    #[serde(default)]
    pub static_columns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDefinitionView {
    pub name: String,
    pub column_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusteringKeyView {
    pub name: String,
    pub order_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDefinedTypeView {
    pub keyspace_name: String,
    pub type_name: String,
    pub field_definitions: Vec<FieldDefinitionView>,
    pub creation_timestamp: Option<String>,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinitionView {
    pub name: String,
    pub field_type: String,
}

// ---- From conversions ----

impl From<&Keyspace> for KeyspaceView {
    fn from(ks: &Keyspace) -> Self {
        Self {
            name: ks.name.clone(),
            arn: ks.arn.clone(),
            replication_strategy: ks.replication_strategy.clone(),
            replication_regions: ks.replication_regions.clone(),
            tags: ks.tags.clone(),
            creation_timestamp: Some(ks.creation_timestamp.to_rfc3339()),
            status: ks.status.clone(),
        }
    }
}

impl From<&Table> for TableView {
    fn from(t: &Table) -> Self {
        Self {
            keyspace_name: t.keyspace_name.clone(),
            table_name: t.table_name.clone(),
            arn: t.arn.clone(),
            schema_definition: SchemaDefinitionView::from(&t.schema_definition),
            capacity_mode: t.capacity_mode.clone(),
            read_capacity_units: t.read_capacity_units,
            write_capacity_units: t.write_capacity_units,
            encryption_type: t.encryption_type.clone(),
            kms_key_identifier: t.kms_key_identifier.clone(),
            point_in_time_recovery_enabled: t.point_in_time_recovery_enabled,
            ttl_status: t.ttl_status.clone(),
            default_time_to_live: t.default_time_to_live,
            comment: t.comment.clone(),
            client_side_timestamps_enabled: t.client_side_timestamps_enabled,
            tags: t.tags.clone(),
            creation_timestamp: Some(t.creation_timestamp.to_rfc3339()),
            status: t.status.clone(),
        }
    }
}

impl From<&SchemaDefinition> for SchemaDefinitionView {
    fn from(s: &SchemaDefinition) -> Self {
        Self {
            all_columns: s
                .all_columns
                .iter()
                .map(|c| ColumnDefinitionView {
                    name: c.name.clone(),
                    column_type: c.column_type.clone(),
                })
                .collect(),
            partition_keys: s.partition_keys.clone(),
            clustering_keys: s
                .clustering_keys
                .iter()
                .map(|c| ClusteringKeyView {
                    name: c.name.clone(),
                    order_by: c.order_by.clone(),
                })
                .collect(),
            static_columns: s.static_columns.clone(),
        }
    }
}

impl From<&UserDefinedType> for UserDefinedTypeView {
    fn from(t: &UserDefinedType) -> Self {
        Self {
            keyspace_name: t.keyspace_name.clone(),
            type_name: t.type_name.clone(),
            field_definitions: t
                .field_definitions
                .iter()
                .map(|f| FieldDefinitionView {
                    name: f.name.clone(),
                    field_type: f.field_type.clone(),
                })
                .collect(),
            creation_timestamp: Some(t.creation_timestamp.to_rfc3339()),
            status: t.status.clone(),
        }
    }
}

impl From<KeyspacesStateView> for KeyspacesState {
    fn from(view: KeyspacesStateView) -> Self {
        let keyspaces = view
            .keyspaces
            .into_iter()
            .map(|(k, v)| {
                let ks = Keyspace {
                    name: v.name,
                    arn: v.arn,
                    replication_strategy: v.replication_strategy,
                    replication_regions: v.replication_regions,
                    tags: v.tags,
                    creation_timestamp: v
                        .creation_timestamp
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(chrono::Utc::now),
                    status: v.status,
                };
                (k, ks)
            })
            .collect();

        let tables = view
            .tables
            .into_values()
            .map(|v| {
                let table = Table {
                    keyspace_name: v.keyspace_name.clone(),
                    table_name: v.table_name.clone(),
                    arn: v.arn,
                    schema_definition: SchemaDefinition {
                        all_columns: v
                            .schema_definition
                            .all_columns
                            .into_iter()
                            .map(|c| ColumnDefinition {
                                name: c.name,
                                column_type: c.column_type,
                            })
                            .collect(),
                        partition_keys: v.schema_definition.partition_keys,
                        clustering_keys: v
                            .schema_definition
                            .clustering_keys
                            .into_iter()
                            .map(|c| ClusteringKey {
                                name: c.name,
                                order_by: c.order_by,
                            })
                            .collect(),
                        static_columns: v.schema_definition.static_columns,
                    },
                    capacity_mode: v.capacity_mode,
                    read_capacity_units: v.read_capacity_units,
                    write_capacity_units: v.write_capacity_units,
                    encryption_type: v.encryption_type,
                    kms_key_identifier: v.kms_key_identifier,
                    point_in_time_recovery_enabled: v.point_in_time_recovery_enabled,
                    ttl_status: v.ttl_status,
                    default_time_to_live: v.default_time_to_live,
                    comment: v.comment,
                    client_side_timestamps_enabled: v.client_side_timestamps_enabled,
                    tags: v.tags,
                    creation_timestamp: v
                        .creation_timestamp
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(chrono::Utc::now),
                    status: v.status,
                };
                (
                    (table.keyspace_name.clone(), table.table_name.clone()),
                    table,
                )
            })
            .collect();

        let types = view
            .types
            .into_values()
            .map(|v| {
                let udt = UserDefinedType {
                    keyspace_name: v.keyspace_name.clone(),
                    type_name: v.type_name.clone(),
                    field_definitions: v
                        .field_definitions
                        .into_iter()
                        .map(|f| FieldDefinition {
                            name: f.name,
                            field_type: f.field_type,
                        })
                        .collect(),
                    creation_timestamp: v
                        .creation_timestamp
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(chrono::Utc::now),
                    status: v.status,
                };
                ((udt.keyspace_name.clone(), udt.type_name.clone()), udt)
            })
            .collect();

        KeyspacesState {
            keyspaces,
            tables,
            types,
        }
    }
}

impl From<&KeyspacesState> for KeyspacesStateView {
    fn from(state: &KeyspacesState) -> Self {
        Self {
            keyspaces: state
                .keyspaces
                .iter()
                .map(|(k, v)| (k.clone(), KeyspaceView::from(v)))
                .collect(),
            tables: state
                .tables
                .iter()
                .map(|((ks, tn), v)| (format!("{ks}/{tn}"), TableView::from(v)))
                .collect(),
            types: state
                .types
                .iter()
                .map(|((ks, tn), v)| (format!("{ks}/{tn}"), UserDefinedTypeView::from(v)))
                .collect(),
        }
    }
}

impl StatefulService for KeyspacesService {
    type StateView = KeyspacesStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        KeyspacesStateView::from(&*guard)
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
            *guard = KeyspacesState::from(view);
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
            // Merge keyspaces.
            for (name, ks_view) in view.keyspaces {
                let ks = Keyspace {
                    name: ks_view.name,
                    arn: ks_view.arn,
                    replication_strategy: ks_view.replication_strategy,
                    replication_regions: ks_view.replication_regions,
                    tags: ks_view.tags,
                    creation_timestamp: ks_view
                        .creation_timestamp
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(chrono::Utc::now),
                    status: ks_view.status,
                };
                guard.keyspaces.insert(name, ks);
            }
            // Merge tables.
            for tv in view.tables.into_values() {
                let table = Table {
                    keyspace_name: tv.keyspace_name.clone(),
                    table_name: tv.table_name.clone(),
                    arn: tv.arn,
                    schema_definition: SchemaDefinition {
                        all_columns: tv
                            .schema_definition
                            .all_columns
                            .into_iter()
                            .map(|c| ColumnDefinition {
                                name: c.name,
                                column_type: c.column_type,
                            })
                            .collect(),
                        partition_keys: tv.schema_definition.partition_keys,
                        clustering_keys: tv
                            .schema_definition
                            .clustering_keys
                            .into_iter()
                            .map(|c| ClusteringKey {
                                name: c.name,
                                order_by: c.order_by,
                            })
                            .collect(),
                        static_columns: tv.schema_definition.static_columns,
                    },
                    capacity_mode: tv.capacity_mode,
                    read_capacity_units: tv.read_capacity_units,
                    write_capacity_units: tv.write_capacity_units,
                    encryption_type: tv.encryption_type,
                    kms_key_identifier: tv.kms_key_identifier,
                    point_in_time_recovery_enabled: tv.point_in_time_recovery_enabled,
                    ttl_status: tv.ttl_status,
                    default_time_to_live: tv.default_time_to_live,
                    comment: tv.comment,
                    client_side_timestamps_enabled: tv.client_side_timestamps_enabled,
                    tags: tv.tags,
                    creation_timestamp: tv
                        .creation_timestamp
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(chrono::Utc::now),
                    status: tv.status,
                };
                guard.tables.insert(
                    (table.keyspace_name.clone(), table.table_name.clone()),
                    table,
                );
            }
            // Merge types.
            for tv in view.types.into_values() {
                let udt = UserDefinedType {
                    keyspace_name: tv.keyspace_name.clone(),
                    type_name: tv.type_name.clone(),
                    field_definitions: tv
                        .field_definitions
                        .into_iter()
                        .map(|f| FieldDefinition {
                            name: f.name,
                            field_type: f.field_type,
                        })
                        .collect(),
                    creation_timestamp: tv
                        .creation_timestamp
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(chrono::Utc::now),
                    status: tv.status,
                };
                guard
                    .types
                    .insert((udt.keyspace_name.clone(), udt.type_name.clone()), udt);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
