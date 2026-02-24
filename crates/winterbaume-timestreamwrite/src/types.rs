use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::Value;

/// A Timestream database.
#[derive(Debug, Clone)]
pub struct Database {
    pub database_name: String,
    pub arn: String,
    pub kms_key_id: Option<String>,
    pub table_count: i64,
    pub creation_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

/// A Timestream table.
#[derive(Debug, Clone)]
pub struct Table {
    pub table_name: String,
    pub database_name: String,
    pub arn: String,
    pub table_status: String,
    pub retention_properties: RetentionProperties,
    pub magnetic_store_write_properties: MagneticStoreWriteProperties,
    pub creation_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
    pub tags: HashMap<String, String>,
    pub records: Vec<Record>,
}

/// Retention properties for a Timestream table.
#[derive(Debug, Clone)]
pub struct RetentionProperties {
    pub memory_store_retention_period_in_hours: i64,
    pub magnetic_store_retention_period_in_days: i64,
}

impl Default for RetentionProperties {
    fn default() -> Self {
        Self {
            memory_store_retention_period_in_hours: 6,
            magnetic_store_retention_period_in_days: 73000,
        }
    }
}

/// Magnetic store write properties for a Timestream table.
#[derive(Debug, Clone, Default)]
pub struct MagneticStoreWriteProperties {
    pub enable_magnetic_store_writes: bool,
}

/// A record written to a Timestream table.
#[derive(Debug, Clone)]
pub struct Record {
    pub dimensions: Vec<Dimension>,
    pub measure_name: Option<String>,
    pub measure_value: Option<String>,
    pub measure_value_type: Option<String>,
    pub time: Option<String>,
    pub time_unit: Option<String>,
}

/// A dimension in a record.
#[derive(Debug, Clone)]
pub struct Dimension {
    pub name: String,
    pub value: String,
    pub dimension_value_type: Option<String>,
}

/// A batch load task that loads data from S3 into a Timestream table.
#[derive(Debug, Clone)]
pub struct BatchLoadTask {
    pub task_id: String,
    pub task_status: String,
    pub database_name: String,
    pub table_name: String,
    /// Raw JSON value of DataSourceConfiguration as provided by the caller.
    pub data_source_configuration: Value,
    /// Raw JSON value of ReportConfiguration as provided by the caller.
    pub report_configuration: Value,
    /// Raw JSON value of DataModelConfiguration as provided by the caller (optional).
    pub data_model_configuration: Option<Value>,
    pub creation_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
    pub error_message: Option<String>,
}
