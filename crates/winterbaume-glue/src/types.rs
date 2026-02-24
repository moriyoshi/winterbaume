use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A Glue Data Catalog database.
#[derive(Debug, Clone)]
pub struct Database {
    pub name: String,
    pub description: String,
    pub location_uri: String,
    pub parameters: HashMap<String, String>,
    pub create_time: DateTime<Utc>,
    pub catalog_id: String,
}

/// A Glue table within a database.
#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub database_name: String,
    pub catalog_id: String,
    pub description: String,
    pub owner: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
    pub table_type: String,
    pub parameters: HashMap<String, String>,
    pub storage_descriptor: Option<serde_json::Value>,
    pub partition_keys: Option<serde_json::Value>,
    pub retention: i32,
    /// Table versions stored as list of (version_id, snapshot).
    pub versions: Vec<TableVersionEntry>,
    pub version_id: String,
}

#[derive(Debug, Clone)]
pub struct TableVersionEntry {
    pub version_id: String,
    pub table_snapshot: serde_json::Value,
}

/// A partition within a table.
#[derive(Debug, Clone)]
pub struct Partition {
    pub values: Vec<String>,
    pub database_name: String,
    pub table_name: String,
    pub catalog_id: String,
    pub creation_time: DateTime<Utc>,
    pub last_access_time: Option<DateTime<Utc>>,
    pub parameters: HashMap<String, String>,
    pub storage_descriptor: Option<serde_json::Value>,
}

/// A Glue connection.
#[derive(Debug, Clone)]
pub struct Connection {
    pub name: String,
    pub connection_type: String,
    pub connection_properties: HashMap<String, String>,
    pub description: String,
    pub creation_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
    pub match_criteria: Vec<String>,
    pub physical_connection_requirements: Option<serde_json::Value>,
}

/// A Glue crawler.
#[derive(Debug, Clone)]
pub struct Crawler {
    pub name: String,
    pub role: String,
    pub database_name: String,
    pub description: String,
    pub targets: Option<serde_json::Value>,
    pub schedule: Option<String>,
    pub classifiers: Vec<String>,
    pub table_prefix: String,
    pub configuration: String,
    pub state: String,
    pub creation_time: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub version: i64,
}

/// A record of a crawler run (crawl).
#[derive(Debug, Clone)]
pub struct CrawlRecord {
    pub crawl_id: String,
    pub crawler_name: String,
    pub state: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

/// A Glue job.
#[derive(Debug, Clone)]
pub struct Job {
    pub name: String,
    pub description: String,
    pub role: String,
    pub command: Option<serde_json::Value>,
    pub default_arguments: HashMap<String, String>,
    pub max_retries: i32,
    pub timeout: i32,
    pub max_capacity: Option<f64>,
    pub number_of_workers: Option<i32>,
    pub worker_type: Option<String>,
    pub glue_version: Option<String>,
    pub created_on: DateTime<Utc>,
    pub last_modified_on: DateTime<Utc>,
}

/// A Glue job run.
#[derive(Debug, Clone)]
pub struct JobRun {
    pub id: String,
    pub job_name: String,
    pub started_on: DateTime<Utc>,
    pub completed_on: Option<DateTime<Utc>>,
    pub job_run_state: String,
    pub arguments: HashMap<String, String>,
    pub timeout: i32,
    pub max_capacity: Option<f64>,
    pub number_of_workers: Option<i32>,
    pub worker_type: Option<String>,
}

/// A Glue trigger.
#[derive(Debug, Clone)]
pub struct Trigger {
    pub name: String,
    pub trigger_type: String,
    pub state: String,
    pub description: String,
    pub schedule: Option<String>,
    pub actions: Option<serde_json::Value>,
    pub predicate: Option<serde_json::Value>,
    pub workflow_name: Option<String>,
}

/// A Glue workflow.
#[derive(Debug, Clone)]
pub struct Workflow {
    pub name: String,
    pub description: String,
    pub default_run_properties: HashMap<String, String>,
    pub created_on: DateTime<Utc>,
    pub last_modified_on: DateTime<Utc>,
    pub max_concurrent_runs: Option<i32>,
    pub runs: Vec<WorkflowRun>,
}

/// A single workflow run.
#[derive(Debug, Clone)]
pub struct WorkflowRun {
    pub workflow_run_id: String,
    pub name: String,
    pub started_on: DateTime<Utc>,
    pub completed_on: Option<DateTime<Utc>>,
    pub status: String,
    pub run_properties: HashMap<String, String>,
}

/// A Glue dev endpoint.
#[derive(Debug, Clone)]
pub struct DevEndpoint {
    pub endpoint_name: String,
    pub role_arn: String,
    pub security_group_ids: Vec<String>,
    pub subnet_id: String,
    pub number_of_nodes: i32,
    pub number_of_workers: Option<i32>,
    pub worker_type: Option<String>,
    pub glue_version: Option<String>,
    pub status: String,
    pub created_timestamp: DateTime<Utc>,
    pub last_modified_timestamp: DateTime<Utc>,
    pub public_key: Option<String>,
    pub public_keys: Vec<String>,
    pub extra_python_libs_s3_path: Option<String>,
    pub extra_jars_s3_path: Option<String>,
    pub arguments: HashMap<String, String>,
}

/// A Glue security configuration.
#[derive(Debug, Clone)]
pub struct SecurityConfiguration {
    pub name: String,
    pub created_time_stamp: DateTime<Utc>,
    pub encryption_configuration: Option<serde_json::Value>,
}

/// A Glue interactive session.
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub description: String,
    pub role: String,
    pub command: Option<serde_json::Value>,
    pub status: String,
    pub created_on: DateTime<Utc>,
    pub glue_version: Option<String>,
    pub max_capacity: Option<f64>,
    pub number_of_workers: Option<i32>,
    pub worker_type: Option<String>,
    pub idle_timeout: Option<i32>,
    pub default_arguments: HashMap<String, String>,
    pub security_configuration: Option<String>,
}

/// A schema registry.
#[derive(Debug, Clone)]
pub struct Registry {
    pub registry_name: String,
    pub registry_arn: String,
    pub description: String,
    pub created_time: DateTime<Utc>,
    pub updated_time: DateTime<Utc>,
    pub status: String,
    pub tags: HashMap<String, String>,
}

/// A schema in the schema registry.
#[derive(Debug, Clone)]
pub struct Schema {
    pub schema_name: String,
    pub schema_arn: String,
    pub registry_name: String,
    pub registry_arn: String,
    pub data_format: String,
    pub compatibility: String,
    pub description: String,
    pub schema_status: String,
    pub created_time: DateTime<Utc>,
    pub updated_time: DateTime<Utc>,
    pub latest_schema_version: i64,
    pub next_schema_version: i64,
    pub schema_checkpoint: i64,
    pub tags: HashMap<String, String>,
    pub versions: Vec<SchemaVersion>,
}

/// A version of a schema.
#[derive(Debug, Clone)]
pub struct SchemaVersion {
    pub schema_version_id: String,
    pub version_number: i64,
    pub schema_definition: String,
    pub status: String,
    pub created_time: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

/// Resource policy.
#[derive(Debug, Clone)]
pub struct ResourcePolicy {
    pub policy_in_json: String,
    pub policy_hash: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

/// Data catalog encryption settings.
#[derive(Debug, Clone)]
pub struct DataCatalogEncryptionSettings {
    pub encryption_at_rest: Option<serde_json::Value>,
    pub connection_password_encryption: Option<serde_json::Value>,
}

/// A Glue ML Transform.
#[derive(Debug, Clone)]
pub struct MLTransform {
    pub transform_id: String,
    pub name: String,
    pub description: String,
    pub role: String,
    pub glue_version: Option<String>,
    pub max_capacity: Option<f64>,
    pub max_retries: Option<i32>,
    pub timeout: Option<i32>,
    pub number_of_workers: Option<i32>,
    pub worker_type: Option<String>,
    pub parameters: Option<serde_json::Value>,
    pub input_record_tables: Vec<serde_json::Value>,
    pub status: String,
    pub created_on: DateTime<Utc>,
    pub last_modified_on: DateTime<Utc>,
}

/// Tags store keyed by resource ARN.
pub type TagStore = HashMap<String, HashMap<String, String>>;

/// Crawler metrics returned by GetCrawlerMetrics.
#[derive(Debug, Clone)]
pub struct CrawlerMetrics {
    pub crawler_name: String,
    pub time_left_seconds: f64,
    pub still_estimating: bool,
    pub last_runtime_seconds: f64,
    pub median_runtime_seconds: f64,
    pub tables_created: i32,
    pub tables_updated: i32,
    pub tables_deleted: i32,
}
