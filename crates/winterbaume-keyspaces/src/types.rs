use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// In-memory representation of a keyspace.
#[derive(Debug, Clone)]
pub struct Keyspace {
    pub name: String,
    pub arn: String,
    pub replication_strategy: String,
    pub replication_regions: Vec<String>,
    pub tags: HashMap<String, String>,
    pub creation_timestamp: DateTime<Utc>,
    pub status: String,
}

/// In-memory representation of a table within a keyspace.
#[derive(Debug, Clone)]
pub struct Table {
    pub keyspace_name: String,
    pub table_name: String,
    pub arn: String,
    pub schema_definition: SchemaDefinition,
    pub capacity_mode: String,
    pub read_capacity_units: Option<i64>,
    pub write_capacity_units: Option<i64>,
    pub encryption_type: String,
    pub kms_key_identifier: Option<String>,
    pub point_in_time_recovery_enabled: bool,
    pub ttl_status: String,
    pub default_time_to_live: Option<i32>,
    pub comment: String,
    pub client_side_timestamps_enabled: bool,
    pub tags: HashMap<String, String>,
    pub creation_timestamp: DateTime<Utc>,
    pub status: String,
}

/// In-memory representation of a user-defined type.
#[derive(Debug, Clone)]
pub struct UserDefinedType {
    pub keyspace_name: String,
    pub type_name: String,
    pub field_definitions: Vec<FieldDefinition>,
    pub creation_timestamp: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct SchemaDefinition {
    pub all_columns: Vec<ColumnDefinition>,
    pub partition_keys: Vec<String>,
    pub clustering_keys: Vec<ClusteringKey>,
    pub static_columns: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    pub name: String,
    pub column_type: String,
}

#[derive(Debug, Clone)]
pub struct ClusteringKey {
    pub name: String,
    pub order_by: String,
}

#[derive(Debug, Clone)]
pub struct FieldDefinition {
    pub name: String,
    pub field_type: String,
}
