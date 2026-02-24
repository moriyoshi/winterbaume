//! Serde-compatible view types for Glue state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::GlueService;
use crate::state::GlueState;
use crate::types::*;

// ─── View types ───

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlueStateView {
    #[serde(default)]
    pub databases: HashMap<String, DatabaseView>,
    #[serde(default)]
    pub tables: HashMap<String, TableView>,
    #[serde(default)]
    pub connections: HashMap<String, ConnectionView>,
    #[serde(default)]
    pub crawlers: HashMap<String, CrawlerView>,
    #[serde(default)]
    pub crawl_records: Vec<CrawlRecordView>,
    #[serde(default)]
    pub jobs: HashMap<String, JobView>,
    /// Job runs keyed by `"{job_name}/{run_id}"`.
    #[serde(default)]
    pub job_runs: HashMap<String, JobRunView>,
    #[serde(default)]
    pub triggers: HashMap<String, TriggerView>,
    #[serde(default)]
    pub workflows: HashMap<String, WorkflowView>,
    #[serde(default)]
    pub dev_endpoints: HashMap<String, DevEndpointView>,
    #[serde(default)]
    pub security_configurations: HashMap<String, SecurityConfigurationView>,
    #[serde(default)]
    pub registries: HashMap<String, RegistryView>,
    #[serde(default)]
    pub schemas: HashMap<String, SchemaView>,
    #[serde(default)]
    pub partitions: HashMap<String, PartitionView>,
    #[serde(default)]
    pub ml_transforms: HashMap<String, MLTransformView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub resource_policy: Option<ResourcePolicyView>,
    #[serde(default)]
    pub data_catalog_encryption_settings: Option<DataCatalogEncryptionSettingsView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePolicyView {
    pub policy_in_json: String,
    pub policy_hash: String,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCatalogEncryptionSettingsView {
    pub encryption_at_rest: Option<Value>,
    pub connection_password_encryption: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseView {
    pub name: String,
    pub description: String,
    pub location_uri: String,
    pub parameters: HashMap<String, String>,
    pub create_time: String,
    pub catalog_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableVersionEntryView {
    pub version_id: String,
    pub table_snapshot: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableView {
    pub name: String,
    pub database_name: String,
    pub catalog_id: String,
    pub description: String,
    pub owner: String,
    pub create_time: String,
    pub update_time: String,
    pub table_type: String,
    pub parameters: HashMap<String, String>,
    pub storage_descriptor: Option<Value>,
    pub partition_keys: Option<Value>,
    pub retention: i32,
    pub versions: Vec<TableVersionEntryView>,
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionView {
    pub values: Vec<String>,
    pub database_name: String,
    pub table_name: String,
    pub catalog_id: String,
    pub creation_time: String,
    pub last_access_time: Option<String>,
    pub parameters: HashMap<String, String>,
    pub storage_descriptor: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionView {
    pub name: String,
    pub connection_type: String,
    pub connection_properties: HashMap<String, String>,
    pub description: String,
    pub creation_time: String,
    pub last_updated_time: String,
    pub match_criteria: Vec<String>,
    pub physical_connection_requirements: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlerView {
    pub name: String,
    pub role: String,
    pub database_name: String,
    pub description: String,
    pub targets: Option<Value>,
    pub schedule: Option<String>,
    pub classifiers: Vec<String>,
    pub table_prefix: String,
    pub configuration: String,
    pub state: String,
    pub creation_time: String,
    pub last_updated: String,
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlRecordView {
    pub crawl_id: String,
    pub crawler_name: String,
    pub state: String,
    pub start_time: String,
    pub end_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobView {
    pub name: String,
    pub description: String,
    pub role: String,
    pub command: Option<Value>,
    pub default_arguments: HashMap<String, String>,
    pub max_retries: i32,
    pub timeout: i32,
    pub max_capacity: Option<f64>,
    pub number_of_workers: Option<i32>,
    pub worker_type: Option<String>,
    pub glue_version: Option<String>,
    pub created_on: String,
    pub last_modified_on: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobRunView {
    pub id: String,
    pub job_name: String,
    pub started_on: String,
    pub completed_on: Option<String>,
    pub job_run_state: String,
    #[serde(default)]
    pub arguments: HashMap<String, String>,
    pub timeout: i32,
    pub max_capacity: Option<f64>,
    pub number_of_workers: Option<i32>,
    pub worker_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerView {
    pub name: String,
    pub trigger_type: String,
    pub state: String,
    pub description: String,
    pub schedule: Option<String>,
    pub actions: Option<Value>,
    pub predicate: Option<Value>,
    pub workflow_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRunView {
    pub workflow_run_id: String,
    pub name: String,
    pub started_on: String,
    pub completed_on: Option<String>,
    pub status: String,
    pub run_properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowView {
    pub name: String,
    pub description: String,
    pub default_run_properties: HashMap<String, String>,
    pub created_on: String,
    pub last_modified_on: String,
    pub max_concurrent_runs: Option<i32>,
    pub runs: Vec<WorkflowRunView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevEndpointView {
    pub endpoint_name: String,
    pub role_arn: String,
    pub security_group_ids: Vec<String>,
    pub subnet_id: String,
    pub number_of_nodes: i32,
    pub number_of_workers: Option<i32>,
    pub worker_type: Option<String>,
    pub glue_version: Option<String>,
    pub status: String,
    pub created_timestamp: String,
    pub last_modified_timestamp: String,
    pub public_key: Option<String>,
    pub public_keys: Vec<String>,
    pub extra_python_libs_s3_path: Option<String>,
    pub extra_jars_s3_path: Option<String>,
    pub arguments: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfigurationView {
    pub name: String,
    pub created_time_stamp: String,
    pub encryption_configuration: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryView {
    pub registry_name: String,
    pub registry_arn: String,
    pub description: String,
    pub created_time: String,
    pub updated_time: String,
    pub status: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaVersionView {
    pub schema_version_id: String,
    pub version_number: i64,
    pub schema_definition: String,
    pub status: String,
    pub created_time: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaView {
    pub schema_name: String,
    pub schema_arn: String,
    pub registry_name: String,
    pub registry_arn: String,
    pub data_format: String,
    pub compatibility: String,
    pub description: String,
    pub schema_status: String,
    pub created_time: String,
    pub updated_time: String,
    pub latest_schema_version: i64,
    pub next_schema_version: i64,
    pub schema_checkpoint: i64,
    pub tags: HashMap<String, String>,
    pub versions: Vec<SchemaVersionView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLTransformView {
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
    pub parameters: Option<Value>,
    pub input_record_tables: Vec<Value>,
    pub status: String,
    pub created_on: String,
    pub last_modified_on: String,
}

// ─── Helper to encode a composite key ───

fn table_key(db: &str, tbl: &str) -> String {
    format!("{}\x00{}", db, tbl)
}

fn partition_key_view(db: &str, tbl: &str, values_key: &str) -> String {
    format!("{}\x00{}\x00{}", db, tbl, values_key)
}

fn job_run_key(job_name: &str, run_id: &str) -> String {
    format!("{job_name}/{run_id}")
}

// ─── From internal → view ───

impl From<&GlueState> for GlueStateView {
    fn from(s: &GlueState) -> Self {
        GlueStateView {
            databases: s
                .databases
                .iter()
                .map(|(k, v)| (k.clone(), DatabaseView::from(v)))
                .collect(),
            tables: s
                .tables
                .iter()
                .map(|((db, tbl), v)| (table_key(db, tbl), TableView::from(v)))
                .collect(),
            connections: s
                .connections
                .iter()
                .map(|(k, v)| (k.clone(), ConnectionView::from(v)))
                .collect(),
            crawlers: s
                .crawlers
                .iter()
                .map(|(k, v)| (k.clone(), CrawlerView::from(v)))
                .collect(),
            crawl_records: s.crawl_records.iter().map(CrawlRecordView::from).collect(),
            jobs: s
                .jobs
                .iter()
                .map(|(k, v)| (k.clone(), JobView::from(v)))
                .collect(),
            job_runs: s
                .job_runs
                .iter()
                .map(|((job_name, run_id), v)| (job_run_key(job_name, run_id), JobRunView::from(v)))
                .collect(),
            triggers: s
                .triggers
                .iter()
                .map(|(k, v)| (k.clone(), TriggerView::from(v)))
                .collect(),
            workflows: s
                .workflows
                .iter()
                .map(|(k, v)| (k.clone(), WorkflowView::from(v)))
                .collect(),
            dev_endpoints: s
                .dev_endpoints
                .iter()
                .map(|(k, v)| (k.clone(), DevEndpointView::from(v)))
                .collect(),
            security_configurations: s
                .security_configurations
                .iter()
                .map(|(k, v)| (k.clone(), SecurityConfigurationView::from(v)))
                .collect(),
            registries: s
                .registries
                .iter()
                .map(|(k, v)| (k.clone(), RegistryView::from(v)))
                .collect(),
            schemas: s
                .schemas
                .iter()
                .map(|(k, v)| (k.clone(), SchemaView::from(v)))
                .collect(),
            partitions: s
                .partitions
                .iter()
                .map(|((db, tbl, vk), v)| (partition_key_view(db, tbl, vk), PartitionView::from(v)))
                .collect(),
            ml_transforms: s
                .ml_transforms
                .iter()
                .map(|(k, v)| (k.clone(), MLTransformView::from(v)))
                .collect(),
            tags: s.tags.clone(),
            resource_policy: s.resource_policy.as_ref().map(ResourcePolicyView::from),
            data_catalog_encryption_settings: s
                .data_catalog_encryption_settings
                .as_ref()
                .map(DataCatalogEncryptionSettingsView::from),
        }
    }
}

impl From<&Database> for DatabaseView {
    fn from(v: &Database) -> Self {
        DatabaseView {
            name: v.name.clone(),
            description: v.description.clone(),
            location_uri: v.location_uri.clone(),
            parameters: v.parameters.clone(),
            create_time: v.create_time.to_rfc3339(),
            catalog_id: v.catalog_id.clone(),
        }
    }
}

impl From<&TableVersionEntry> for TableVersionEntryView {
    fn from(v: &TableVersionEntry) -> Self {
        TableVersionEntryView {
            version_id: v.version_id.clone(),
            table_snapshot: v.table_snapshot.clone(),
        }
    }
}

impl From<&Table> for TableView {
    fn from(v: &Table) -> Self {
        TableView {
            name: v.name.clone(),
            database_name: v.database_name.clone(),
            catalog_id: v.catalog_id.clone(),
            description: v.description.clone(),
            owner: v.owner.clone(),
            create_time: v.create_time.to_rfc3339(),
            update_time: v.update_time.to_rfc3339(),
            table_type: v.table_type.clone(),
            parameters: v.parameters.clone(),
            storage_descriptor: v.storage_descriptor.clone(),
            partition_keys: v.partition_keys.clone(),
            retention: v.retention,
            versions: v.versions.iter().map(TableVersionEntryView::from).collect(),
            version_id: v.version_id.clone(),
        }
    }
}

impl From<&Partition> for PartitionView {
    fn from(v: &Partition) -> Self {
        PartitionView {
            values: v.values.clone(),
            database_name: v.database_name.clone(),
            table_name: v.table_name.clone(),
            catalog_id: v.catalog_id.clone(),
            creation_time: v.creation_time.to_rfc3339(),
            last_access_time: v.last_access_time.map(|t| t.to_rfc3339()),
            parameters: v.parameters.clone(),
            storage_descriptor: v.storage_descriptor.clone(),
        }
    }
}

impl From<&Connection> for ConnectionView {
    fn from(v: &Connection) -> Self {
        ConnectionView {
            name: v.name.clone(),
            connection_type: v.connection_type.clone(),
            connection_properties: v.connection_properties.clone(),
            description: v.description.clone(),
            creation_time: v.creation_time.to_rfc3339(),
            last_updated_time: v.last_updated_time.to_rfc3339(),
            match_criteria: v.match_criteria.clone(),
            physical_connection_requirements: v.physical_connection_requirements.clone(),
        }
    }
}

impl From<&Crawler> for CrawlerView {
    fn from(v: &Crawler) -> Self {
        CrawlerView {
            name: v.name.clone(),
            role: v.role.clone(),
            database_name: v.database_name.clone(),
            description: v.description.clone(),
            targets: v.targets.clone(),
            schedule: v.schedule.clone(),
            classifiers: v.classifiers.clone(),
            table_prefix: v.table_prefix.clone(),
            configuration: v.configuration.clone(),
            state: v.state.clone(),
            creation_time: v.creation_time.to_rfc3339(),
            last_updated: v.last_updated.to_rfc3339(),
            version: v.version,
        }
    }
}

impl From<&CrawlRecord> for CrawlRecordView {
    fn from(v: &CrawlRecord) -> Self {
        CrawlRecordView {
            crawl_id: v.crawl_id.clone(),
            crawler_name: v.crawler_name.clone(),
            state: v.state.clone(),
            start_time: v.start_time.to_rfc3339(),
            end_time: v.end_time.map(|t| t.to_rfc3339()),
        }
    }
}

impl From<&Job> for JobView {
    fn from(v: &Job) -> Self {
        JobView {
            name: v.name.clone(),
            description: v.description.clone(),
            role: v.role.clone(),
            command: v.command.clone(),
            default_arguments: v.default_arguments.clone(),
            max_retries: v.max_retries,
            timeout: v.timeout,
            max_capacity: v.max_capacity,
            number_of_workers: v.number_of_workers,
            worker_type: v.worker_type.clone(),
            glue_version: v.glue_version.clone(),
            created_on: v.created_on.to_rfc3339(),
            last_modified_on: v.last_modified_on.to_rfc3339(),
        }
    }
}

impl From<&JobRun> for JobRunView {
    fn from(v: &JobRun) -> Self {
        JobRunView {
            id: v.id.clone(),
            job_name: v.job_name.clone(),
            started_on: v.started_on.to_rfc3339(),
            completed_on: v.completed_on.map(|t| t.to_rfc3339()),
            job_run_state: v.job_run_state.clone(),
            arguments: v.arguments.clone(),
            timeout: v.timeout,
            max_capacity: v.max_capacity,
            number_of_workers: v.number_of_workers,
            worker_type: v.worker_type.clone(),
        }
    }
}

impl From<&Trigger> for TriggerView {
    fn from(v: &Trigger) -> Self {
        TriggerView {
            name: v.name.clone(),
            trigger_type: v.trigger_type.clone(),
            state: v.state.clone(),
            description: v.description.clone(),
            schedule: v.schedule.clone(),
            actions: v.actions.clone(),
            predicate: v.predicate.clone(),
            workflow_name: v.workflow_name.clone(),
        }
    }
}

impl From<&WorkflowRun> for WorkflowRunView {
    fn from(v: &WorkflowRun) -> Self {
        WorkflowRunView {
            workflow_run_id: v.workflow_run_id.clone(),
            name: v.name.clone(),
            started_on: v.started_on.to_rfc3339(),
            completed_on: v.completed_on.map(|t| t.to_rfc3339()),
            status: v.status.clone(),
            run_properties: v.run_properties.clone(),
        }
    }
}

impl From<&Workflow> for WorkflowView {
    fn from(v: &Workflow) -> Self {
        WorkflowView {
            name: v.name.clone(),
            description: v.description.clone(),
            default_run_properties: v.default_run_properties.clone(),
            created_on: v.created_on.to_rfc3339(),
            last_modified_on: v.last_modified_on.to_rfc3339(),
            max_concurrent_runs: v.max_concurrent_runs,
            runs: v.runs.iter().map(WorkflowRunView::from).collect(),
        }
    }
}

impl From<&DevEndpoint> for DevEndpointView {
    fn from(v: &DevEndpoint) -> Self {
        DevEndpointView {
            endpoint_name: v.endpoint_name.clone(),
            role_arn: v.role_arn.clone(),
            security_group_ids: v.security_group_ids.clone(),
            subnet_id: v.subnet_id.clone(),
            number_of_nodes: v.number_of_nodes,
            number_of_workers: v.number_of_workers,
            worker_type: v.worker_type.clone(),
            glue_version: v.glue_version.clone(),
            status: v.status.clone(),
            created_timestamp: v.created_timestamp.to_rfc3339(),
            last_modified_timestamp: v.last_modified_timestamp.to_rfc3339(),
            public_key: v.public_key.clone(),
            public_keys: v.public_keys.clone(),
            extra_python_libs_s3_path: v.extra_python_libs_s3_path.clone(),
            extra_jars_s3_path: v.extra_jars_s3_path.clone(),
            arguments: v.arguments.clone(),
        }
    }
}

impl From<&SecurityConfiguration> for SecurityConfigurationView {
    fn from(v: &SecurityConfiguration) -> Self {
        SecurityConfigurationView {
            name: v.name.clone(),
            created_time_stamp: v.created_time_stamp.to_rfc3339(),
            encryption_configuration: v.encryption_configuration.clone(),
        }
    }
}

impl From<&Registry> for RegistryView {
    fn from(v: &Registry) -> Self {
        RegistryView {
            registry_name: v.registry_name.clone(),
            registry_arn: v.registry_arn.clone(),
            description: v.description.clone(),
            created_time: v.created_time.to_rfc3339(),
            updated_time: v.updated_time.to_rfc3339(),
            status: v.status.clone(),
            tags: v.tags.clone(),
        }
    }
}

impl From<&SchemaVersion> for SchemaVersionView {
    fn from(v: &SchemaVersion) -> Self {
        SchemaVersionView {
            schema_version_id: v.schema_version_id.clone(),
            version_number: v.version_number,
            schema_definition: v.schema_definition.clone(),
            status: v.status.clone(),
            created_time: v.created_time.to_rfc3339(),
            metadata: v.metadata.clone(),
        }
    }
}

impl From<&Schema> for SchemaView {
    fn from(v: &Schema) -> Self {
        SchemaView {
            schema_name: v.schema_name.clone(),
            schema_arn: v.schema_arn.clone(),
            registry_name: v.registry_name.clone(),
            registry_arn: v.registry_arn.clone(),
            data_format: v.data_format.clone(),
            compatibility: v.compatibility.clone(),
            description: v.description.clone(),
            schema_status: v.schema_status.clone(),
            created_time: v.created_time.to_rfc3339(),
            updated_time: v.updated_time.to_rfc3339(),
            latest_schema_version: v.latest_schema_version,
            next_schema_version: v.next_schema_version,
            schema_checkpoint: v.schema_checkpoint,
            tags: v.tags.clone(),
            versions: v.versions.iter().map(SchemaVersionView::from).collect(),
        }
    }
}

impl From<&ResourcePolicy> for ResourcePolicyView {
    fn from(v: &ResourcePolicy) -> Self {
        ResourcePolicyView {
            policy_in_json: v.policy_in_json.clone(),
            policy_hash: v.policy_hash.clone(),
            create_time: v.create_time.to_rfc3339(),
            update_time: v.update_time.to_rfc3339(),
        }
    }
}

impl From<&DataCatalogEncryptionSettings> for DataCatalogEncryptionSettingsView {
    fn from(v: &DataCatalogEncryptionSettings) -> Self {
        DataCatalogEncryptionSettingsView {
            encryption_at_rest: v.encryption_at_rest.clone(),
            connection_password_encryption: v.connection_password_encryption.clone(),
        }
    }
}

// ─── From view → internal ───

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

impl From<GlueStateView> for GlueState {
    fn from(v: GlueStateView) -> Self {
        let tables = v
            .tables
            .into_values()
            .map(|tv| {
                let k = (tv.database_name.clone(), tv.name.clone());
                (k, Table::from(tv))
            })
            .collect();

        let partitions = v
            .partitions
            .into_values()
            .map(|pv| {
                let k = (
                    pv.database_name.clone(),
                    pv.table_name.clone(),
                    pv.values.join("#"),
                );
                (k, Partition::from(pv))
            })
            .collect();

        let job_runs = v
            .job_runs
            .into_iter()
            .map(|(key, jrv)| {
                // Prefer parsing the composite key, but fall back to the embedded field values.
                let (job_name, run_id) = match key.split_once('/') {
                    Some((job, id)) => (job.to_string(), id.to_string()),
                    None => (jrv.job_name.clone(), jrv.id.clone()),
                };
                ((job_name, run_id), JobRun::from(jrv))
            })
            .collect();

        GlueState {
            databases: v
                .databases
                .into_values()
                .map(|dv| (dv.name.clone(), Database::from(dv)))
                .collect(),
            tables,
            partitions,
            connections: v
                .connections
                .into_values()
                .map(|cv| (cv.name.clone(), Connection::from(cv)))
                .collect(),
            crawlers: v
                .crawlers
                .into_values()
                .map(|cv| (cv.name.clone(), Crawler::from(cv)))
                .collect(),
            crawl_records: v.crawl_records.into_iter().map(CrawlRecord::from).collect(),
            jobs: v
                .jobs
                .into_values()
                .map(|jv| (jv.name.clone(), Job::from(jv)))
                .collect(),
            job_runs,
            triggers: v
                .triggers
                .into_values()
                .map(|tv| (tv.name.clone(), Trigger::from(tv)))
                .collect(),
            workflows: v
                .workflows
                .into_values()
                .map(|wv| (wv.name.clone(), Workflow::from(wv)))
                .collect(),
            dev_endpoints: v
                .dev_endpoints
                .into_values()
                .map(|dv| (dv.endpoint_name.clone(), DevEndpoint::from(dv)))
                .collect(),
            security_configurations: v
                .security_configurations
                .into_values()
                .map(|sv| (sv.name.clone(), SecurityConfiguration::from(sv)))
                .collect(),
            sessions: HashMap::new(),
            registries: v
                .registries
                .into_values()
                .map(|rv| (rv.registry_name.clone(), Registry::from(rv)))
                .collect(),
            schemas: v
                .schemas
                .into_values()
                .map(|sv| (sv.schema_name.clone(), Schema::from(sv)))
                .collect(),
            ml_transforms: v
                .ml_transforms
                .into_values()
                .map(|mv| (mv.transform_id.clone(), MLTransform::from(mv)))
                .collect(),
            resource_policy: v.resource_policy.map(ResourcePolicy::from),
            data_catalog_encryption_settings: v
                .data_catalog_encryption_settings
                .map(DataCatalogEncryptionSettings::from),
            tags: v.tags,
        }
    }
}

impl From<DatabaseView> for Database {
    fn from(v: DatabaseView) -> Self {
        Database {
            name: v.name,
            description: v.description,
            location_uri: v.location_uri,
            parameters: v.parameters,
            create_time: parse_dt(&v.create_time),
            catalog_id: v.catalog_id,
        }
    }
}

impl From<TableVersionEntryView> for TableVersionEntry {
    fn from(v: TableVersionEntryView) -> Self {
        TableVersionEntry {
            version_id: v.version_id,
            table_snapshot: v.table_snapshot,
        }
    }
}

impl From<TableView> for Table {
    fn from(v: TableView) -> Self {
        Table {
            name: v.name,
            database_name: v.database_name,
            catalog_id: v.catalog_id,
            description: v.description,
            owner: v.owner,
            create_time: parse_dt(&v.create_time),
            update_time: parse_dt(&v.update_time),
            table_type: v.table_type,
            parameters: v.parameters,
            storage_descriptor: v.storage_descriptor,
            partition_keys: v.partition_keys,
            retention: v.retention,
            versions: v
                .versions
                .into_iter()
                .map(TableVersionEntry::from)
                .collect(),
            version_id: v.version_id,
        }
    }
}

impl From<PartitionView> for Partition {
    fn from(v: PartitionView) -> Self {
        Partition {
            values: v.values,
            database_name: v.database_name,
            table_name: v.table_name,
            catalog_id: v.catalog_id,
            creation_time: parse_dt(&v.creation_time),
            last_access_time: v.last_access_time.as_deref().map(parse_dt),
            parameters: v.parameters,
            storage_descriptor: v.storage_descriptor,
        }
    }
}

impl From<ConnectionView> for Connection {
    fn from(v: ConnectionView) -> Self {
        Connection {
            name: v.name,
            connection_type: v.connection_type,
            connection_properties: v.connection_properties,
            description: v.description,
            creation_time: parse_dt(&v.creation_time),
            last_updated_time: parse_dt(&v.last_updated_time),
            match_criteria: v.match_criteria,
            physical_connection_requirements: v.physical_connection_requirements,
        }
    }
}

impl From<CrawlerView> for Crawler {
    fn from(v: CrawlerView) -> Self {
        Crawler {
            name: v.name,
            role: v.role,
            database_name: v.database_name,
            description: v.description,
            targets: v.targets,
            schedule: v.schedule,
            classifiers: v.classifiers,
            table_prefix: v.table_prefix,
            configuration: v.configuration,
            state: v.state,
            creation_time: parse_dt(&v.creation_time),
            last_updated: parse_dt(&v.last_updated),
            version: v.version,
        }
    }
}

impl From<CrawlRecordView> for CrawlRecord {
    fn from(v: CrawlRecordView) -> Self {
        CrawlRecord {
            crawl_id: v.crawl_id,
            crawler_name: v.crawler_name,
            state: v.state,
            start_time: parse_dt(&v.start_time),
            end_time: v.end_time.as_deref().map(parse_dt),
        }
    }
}

impl From<JobView> for Job {
    fn from(v: JobView) -> Self {
        Job {
            name: v.name,
            description: v.description,
            role: v.role,
            command: v.command,
            default_arguments: v.default_arguments,
            max_retries: v.max_retries,
            timeout: v.timeout,
            max_capacity: v.max_capacity,
            number_of_workers: v.number_of_workers,
            worker_type: v.worker_type,
            glue_version: v.glue_version,
            created_on: parse_dt(&v.created_on),
            last_modified_on: parse_dt(&v.last_modified_on),
        }
    }
}

impl From<JobRunView> for JobRun {
    fn from(v: JobRunView) -> Self {
        JobRun {
            id: v.id,
            job_name: v.job_name,
            started_on: parse_dt(&v.started_on),
            completed_on: v.completed_on.as_deref().map(parse_dt),
            job_run_state: v.job_run_state,
            arguments: v.arguments,
            timeout: v.timeout,
            max_capacity: v.max_capacity,
            number_of_workers: v.number_of_workers,
            worker_type: v.worker_type,
        }
    }
}

impl From<TriggerView> for Trigger {
    fn from(v: TriggerView) -> Self {
        Trigger {
            name: v.name,
            trigger_type: v.trigger_type,
            state: v.state,
            description: v.description,
            schedule: v.schedule,
            actions: v.actions,
            predicate: v.predicate,
            workflow_name: v.workflow_name,
        }
    }
}

impl From<WorkflowRunView> for WorkflowRun {
    fn from(v: WorkflowRunView) -> Self {
        WorkflowRun {
            workflow_run_id: v.workflow_run_id,
            name: v.name,
            started_on: parse_dt(&v.started_on),
            completed_on: v.completed_on.as_deref().map(parse_dt),
            status: v.status,
            run_properties: v.run_properties,
        }
    }
}

impl From<WorkflowView> for Workflow {
    fn from(v: WorkflowView) -> Self {
        Workflow {
            name: v.name,
            description: v.description,
            default_run_properties: v.default_run_properties,
            created_on: parse_dt(&v.created_on),
            last_modified_on: parse_dt(&v.last_modified_on),
            max_concurrent_runs: v.max_concurrent_runs,
            runs: v.runs.into_iter().map(WorkflowRun::from).collect(),
        }
    }
}

impl From<DevEndpointView> for DevEndpoint {
    fn from(v: DevEndpointView) -> Self {
        DevEndpoint {
            endpoint_name: v.endpoint_name,
            role_arn: v.role_arn,
            security_group_ids: v.security_group_ids,
            subnet_id: v.subnet_id,
            number_of_nodes: v.number_of_nodes,
            number_of_workers: v.number_of_workers,
            worker_type: v.worker_type,
            glue_version: v.glue_version,
            status: v.status,
            created_timestamp: parse_dt(&v.created_timestamp),
            last_modified_timestamp: parse_dt(&v.last_modified_timestamp),
            public_key: v.public_key,
            public_keys: v.public_keys,
            extra_python_libs_s3_path: v.extra_python_libs_s3_path,
            extra_jars_s3_path: v.extra_jars_s3_path,
            arguments: v.arguments,
        }
    }
}

impl From<SecurityConfigurationView> for SecurityConfiguration {
    fn from(v: SecurityConfigurationView) -> Self {
        SecurityConfiguration {
            name: v.name,
            created_time_stamp: parse_dt(&v.created_time_stamp),
            encryption_configuration: v.encryption_configuration,
        }
    }
}

impl From<RegistryView> for Registry {
    fn from(v: RegistryView) -> Self {
        Registry {
            registry_name: v.registry_name,
            registry_arn: v.registry_arn,
            description: v.description,
            created_time: parse_dt(&v.created_time),
            updated_time: parse_dt(&v.updated_time),
            status: v.status,
            tags: v.tags,
        }
    }
}

impl From<SchemaVersionView> for SchemaVersion {
    fn from(v: SchemaVersionView) -> Self {
        SchemaVersion {
            schema_version_id: v.schema_version_id,
            version_number: v.version_number,
            schema_definition: v.schema_definition,
            status: v.status,
            created_time: parse_dt(&v.created_time),
            metadata: v.metadata,
        }
    }
}

impl From<SchemaView> for Schema {
    fn from(v: SchemaView) -> Self {
        Schema {
            schema_name: v.schema_name,
            schema_arn: v.schema_arn,
            registry_name: v.registry_name,
            registry_arn: v.registry_arn,
            data_format: v.data_format,
            compatibility: v.compatibility,
            description: v.description,
            schema_status: v.schema_status,
            created_time: parse_dt(&v.created_time),
            updated_time: parse_dt(&v.updated_time),
            latest_schema_version: v.latest_schema_version,
            next_schema_version: v.next_schema_version,
            schema_checkpoint: v.schema_checkpoint,
            tags: v.tags,
            versions: v.versions.into_iter().map(SchemaVersion::from).collect(),
        }
    }
}

impl From<&MLTransform> for MLTransformView {
    fn from(v: &MLTransform) -> Self {
        MLTransformView {
            transform_id: v.transform_id.clone(),
            name: v.name.clone(),
            description: v.description.clone(),
            role: v.role.clone(),
            glue_version: v.glue_version.clone(),
            max_capacity: v.max_capacity,
            max_retries: v.max_retries,
            timeout: v.timeout,
            number_of_workers: v.number_of_workers,
            worker_type: v.worker_type.clone(),
            parameters: v.parameters.clone(),
            input_record_tables: v.input_record_tables.clone(),
            status: v.status.clone(),
            created_on: v.created_on.to_rfc3339(),
            last_modified_on: v.last_modified_on.to_rfc3339(),
        }
    }
}

impl From<MLTransformView> for MLTransform {
    fn from(v: MLTransformView) -> Self {
        MLTransform {
            transform_id: v.transform_id,
            name: v.name,
            description: v.description,
            role: v.role,
            glue_version: v.glue_version,
            max_capacity: v.max_capacity,
            max_retries: v.max_retries,
            timeout: v.timeout,
            number_of_workers: v.number_of_workers,
            worker_type: v.worker_type,
            parameters: v.parameters,
            input_record_tables: v.input_record_tables,
            status: v.status,
            created_on: parse_dt(&v.created_on),
            last_modified_on: parse_dt(&v.last_modified_on),
        }
    }
}

impl From<ResourcePolicyView> for ResourcePolicy {
    fn from(v: ResourcePolicyView) -> Self {
        ResourcePolicy {
            policy_in_json: v.policy_in_json,
            policy_hash: v.policy_hash,
            create_time: parse_dt(&v.create_time),
            update_time: parse_dt(&v.update_time),
        }
    }
}

impl From<DataCatalogEncryptionSettingsView> for DataCatalogEncryptionSettings {
    fn from(v: DataCatalogEncryptionSettingsView) -> Self {
        DataCatalogEncryptionSettings {
            encryption_at_rest: v.encryption_at_rest,
            connection_password_encryption: v.connection_password_encryption,
        }
    }
}

// ─── StatefulService implementation ───

impl StatefulService for GlueService {
    type StateView = GlueStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        GlueStateView::from(&*guard)
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
            *guard = GlueState::from(view);
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
            let merged = GlueState::from(view);
            for (k, v) in merged.databases {
                guard.databases.insert(k, v);
            }
            for (k, v) in merged.tables {
                guard.tables.insert(k, v);
            }
            for (k, v) in merged.partitions {
                guard.partitions.insert(k, v);
            }
            for (k, v) in merged.connections {
                guard.connections.insert(k, v);
            }
            for (k, v) in merged.crawlers {
                guard.crawlers.insert(k, v);
            }
            guard.crawl_records.extend(merged.crawl_records);
            for (k, v) in merged.jobs {
                guard.jobs.insert(k, v);
            }
            for (k, v) in merged.job_runs {
                guard.job_runs.insert(k, v);
            }
            for (k, v) in merged.triggers {
                guard.triggers.insert(k, v);
            }
            for (k, v) in merged.workflows {
                guard.workflows.insert(k, v);
            }
            for (k, v) in merged.dev_endpoints {
                guard.dev_endpoints.insert(k, v);
            }
            for (k, v) in merged.security_configurations {
                guard.security_configurations.insert(k, v);
            }
            for (k, v) in merged.registries {
                guard.registries.insert(k, v);
            }
            for (k, v) in merged.schemas {
                guard.schemas.insert(k, v);
            }
            for (k, v) in merged.ml_transforms {
                guard.ml_transforms.insert(k, v);
            }
            for (k, v) in merged.tags {
                guard.tags.insert(k, v);
            }
            if merged.resource_policy.is_some() {
                guard.resource_policy = merged.resource_policy;
            }
            if merged.data_catalog_encryption_settings.is_some() {
                guard.data_catalog_encryption_settings = merged.data_catalog_encryption_settings;
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
