use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

use crate::types::*;

/// In-memory state for the Glue service.
#[derive(Debug, Default)]
pub struct GlueState {
    pub databases: HashMap<String, Database>,
    /// Tables keyed by (database_name, table_name).
    pub tables: HashMap<(String, String), Table>,
    /// Partitions keyed by (database_name, table_name, values_key).
    pub partitions: HashMap<(String, String, String), Partition>,
    pub connections: HashMap<String, Connection>,
    pub crawlers: HashMap<String, Crawler>,
    pub crawl_records: Vec<CrawlRecord>,
    pub jobs: HashMap<String, Job>,
    /// Job runs keyed by (job_name, run_id).
    pub job_runs: HashMap<(String, String), JobRun>,
    pub triggers: HashMap<String, Trigger>,
    pub workflows: HashMap<String, Workflow>,
    pub dev_endpoints: HashMap<String, DevEndpoint>,
    pub security_configurations: HashMap<String, SecurityConfiguration>,
    pub sessions: HashMap<String, Session>,
    pub registries: HashMap<String, Registry>,
    /// Schemas keyed by schema_name.
    pub schemas: HashMap<String, Schema>,
    /// ML Transforms keyed by transform_id.
    pub ml_transforms: HashMap<String, MLTransform>,
    pub resource_policy: Option<ResourcePolicy>,
    pub data_catalog_encryption_settings: Option<DataCatalogEncryptionSettings>,
    pub tags: TagStore,
}

/// Error type for Glue operations.
#[derive(Debug, thiserror::Error)]
pub enum GlueError {
    #[error("{0}")]
    EntityNotFoundException(String),
    #[error("{0}")]
    AlreadyExistsException(String),
    #[error("{0}")]
    InvalidInputException(String),
}

fn not_found(msg: impl Into<String>) -> GlueError {
    GlueError::EntityNotFoundException(msg.into())
}

fn already_exists(msg: impl Into<String>) -> GlueError {
    GlueError::AlreadyExistsException(msg.into())
}

fn invalid_input(msg: impl Into<String>) -> GlueError {
    GlueError::InvalidInputException(msg.into())
}

fn partition_key(values: &[String]) -> String {
    values.join("#")
}

impl GlueState {
    // ─── Database operations ───

    pub fn create_database(
        &mut self,
        name: &str,
        description: &str,
        location_uri: &str,
        parameters: HashMap<String, String>,
        catalog_id: &str,
    ) -> Result<&Database, GlueError> {
        if self.databases.contains_key(name) {
            return Err(already_exists(format!("Database already exists: {name}")));
        }
        let db = Database {
            name: name.to_string(),
            description: description.to_string(),
            location_uri: location_uri.to_string(),
            parameters,
            create_time: Utc::now(),
            catalog_id: catalog_id.to_string(),
        };
        self.databases.insert(name.to_string(), db);
        Ok(self.databases.get(name).unwrap())
    }

    pub fn get_database(&self, name: &str) -> Result<&Database, GlueError> {
        self.databases
            .get(name)
            .ok_or_else(|| not_found(format!("Database {name} not found.")))
    }

    pub fn delete_database(&mut self, name: &str) -> Result<(), GlueError> {
        self.databases
            .remove(name)
            .ok_or_else(|| not_found(format!("Database {name} not found.")))?;
        Ok(())
    }

    pub fn get_databases(&self) -> Vec<&Database> {
        self.databases.values().collect()
    }

    pub fn update_database(
        &mut self,
        name: &str,
        description: Option<&str>,
        location_uri: Option<&str>,
        parameters: Option<HashMap<String, String>>,
    ) -> Result<(), GlueError> {
        let db = self
            .databases
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Database {name} not found.")))?;
        if let Some(d) = description {
            db.description = d.to_string();
        }
        if let Some(l) = location_uri {
            db.location_uri = l.to_string();
        }
        if let Some(p) = parameters {
            db.parameters = p;
        }
        Ok(())
    }

    // ─── Table operations ───

    pub fn create_table(
        &mut self,
        database_name: &str,
        name: &str,
        catalog_id: &str,
        description: &str,
        owner: &str,
        table_type: &str,
        parameters: HashMap<String, String>,
        storage_descriptor: Option<serde_json::Value>,
        partition_keys: Option<serde_json::Value>,
        retention: i32,
    ) -> Result<(), GlueError> {
        if !self.databases.contains_key(database_name) {
            return Err(not_found(format!("Database {database_name} not found.")));
        }
        let key = (database_name.to_string(), name.to_string());
        if self.tables.contains_key(&key) {
            return Err(already_exists(format!(
                "Table {name} already exists in database {database_name}"
            )));
        }
        let now = Utc::now();
        let tbl = Table {
            name: name.to_string(),
            database_name: database_name.to_string(),
            catalog_id: catalog_id.to_string(),
            description: description.to_string(),
            owner: owner.to_string(),
            create_time: now,
            update_time: now,
            table_type: table_type.to_string(),
            parameters,
            storage_descriptor,
            partition_keys,
            retention,
            versions: Vec::new(),
            version_id: "0".to_string(),
        };
        self.tables.insert(key, tbl);
        Ok(())
    }

    pub fn get_table(&self, database_name: &str, name: &str) -> Result<&Table, GlueError> {
        let key = (database_name.to_string(), name.to_string());
        self.tables
            .get(&key)
            .ok_or_else(|| not_found(format!("Table {name} not found.")))
    }

    pub fn get_tables(&self, database_name: &str) -> Vec<&Table> {
        self.tables
            .values()
            .filter(|t| t.database_name == database_name)
            .collect()
    }

    pub fn delete_table(&mut self, database_name: &str, name: &str) -> Result<(), GlueError> {
        let key = (database_name.to_string(), name.to_string());
        self.tables
            .remove(&key)
            .ok_or_else(|| not_found(format!("Table {name} not found.")))?;
        // Also remove partitions for this table.
        self.partitions
            .retain(|k, _| !(k.0 == database_name && k.1 == name));
        Ok(())
    }

    pub fn batch_delete_table(
        &mut self,
        database_name: &str,
        table_names: &[String],
    ) -> Vec<(String, Option<GlueError>)> {
        table_names
            .iter()
            .map(|name| {
                let result = self.delete_table(database_name, name);
                (name.clone(), result.err())
            })
            .collect()
    }

    pub fn update_table(
        &mut self,
        database_name: &str,
        name: &str,
        description: Option<&str>,
        parameters: Option<HashMap<String, String>>,
        storage_descriptor: Option<serde_json::Value>,
        retention: Option<i32>,
    ) -> Result<(), GlueError> {
        let key = (database_name.to_string(), name.to_string());
        let tbl = self
            .tables
            .get_mut(&key)
            .ok_or_else(|| not_found(format!("Table {name} not found.")))?;

        // Save current version
        let ver_id = tbl.version_id.clone();
        let snapshot = table_to_json_snapshot(tbl);
        tbl.versions.push(TableVersionEntry {
            version_id: ver_id,
            table_snapshot: snapshot,
        });

        if let Some(d) = description {
            tbl.description = d.to_string();
        }
        if let Some(p) = parameters {
            tbl.parameters = p;
        }
        if let Some(sd) = storage_descriptor {
            tbl.storage_descriptor = Some(sd);
        }
        if let Some(r) = retention {
            tbl.retention = r;
        }
        let new_version: i64 = tbl.version_id.parse::<i64>().unwrap_or(0) + 1;
        tbl.version_id = new_version.to_string();
        tbl.update_time = Utc::now();
        Ok(())
    }

    pub fn get_table_version(
        &self,
        database_name: &str,
        table_name: &str,
        version_id: &str,
    ) -> Result<serde_json::Value, GlueError> {
        let tbl = self.get_table(database_name, table_name)?;
        // Current version
        if version_id == tbl.version_id {
            return Ok(table_to_json_snapshot(tbl));
        }
        for v in &tbl.versions {
            if v.version_id == version_id {
                return Ok(v.table_snapshot.clone());
            }
        }
        Err(not_found(format!("Version {version_id} not found.")))
    }

    pub fn get_table_versions(
        &self,
        database_name: &str,
        table_name: &str,
    ) -> Result<Vec<(String, serde_json::Value)>, GlueError> {
        let tbl = self.get_table(database_name, table_name)?;
        let mut result: Vec<(String, serde_json::Value)> = tbl
            .versions
            .iter()
            .map(|v| (v.version_id.clone(), v.table_snapshot.clone()))
            .collect();
        result.push((tbl.version_id.clone(), table_to_json_snapshot(tbl)));
        Ok(result)
    }

    pub fn delete_table_version(
        &mut self,
        database_name: &str,
        table_name: &str,
        version_id: &str,
    ) -> Result<(), GlueError> {
        let key = (database_name.to_string(), table_name.to_string());
        let tbl = self
            .tables
            .get_mut(&key)
            .ok_or_else(|| not_found(format!("Table {table_name} not found.")))?;
        let before = tbl.versions.len();
        tbl.versions.retain(|v| v.version_id != version_id);
        if tbl.versions.len() == before {
            return Err(not_found(format!("Version {version_id} not found.")));
        }
        Ok(())
    }

    // ─── Partition operations ───

    pub fn create_partition(
        &mut self,
        database_name: &str,
        table_name: &str,
        catalog_id: &str,
        values: Vec<String>,
        parameters: HashMap<String, String>,
        storage_descriptor: Option<serde_json::Value>,
    ) -> Result<(), GlueError> {
        let tbl_key = (database_name.to_string(), table_name.to_string());
        if !self.tables.contains_key(&tbl_key) {
            return Err(not_found(format!("Table {table_name} not found.")));
        }
        let pk = partition_key(&values);
        let key = (database_name.to_string(), table_name.to_string(), pk);
        if self.partitions.contains_key(&key) {
            return Err(already_exists("Partition already exists.".to_string()));
        }
        let p = Partition {
            values,
            database_name: database_name.to_string(),
            table_name: table_name.to_string(),
            catalog_id: catalog_id.to_string(),
            creation_time: Utc::now(),
            last_access_time: None,
            parameters,
            storage_descriptor,
        };
        self.partitions.insert(key, p);
        Ok(())
    }

    pub fn batch_create_partition(
        &mut self,
        database_name: &str,
        table_name: &str,
        catalog_id: &str,
        partition_inputs: Vec<(
            Vec<String>,
            HashMap<String, String>,
            Option<serde_json::Value>,
        )>,
    ) -> Vec<Option<GlueError>> {
        partition_inputs
            .into_iter()
            .map(|(values, parameters, sd)| {
                self.create_partition(
                    database_name,
                    table_name,
                    catalog_id,
                    values,
                    parameters,
                    sd,
                )
                .err()
            })
            .collect()
    }

    pub fn get_partition(
        &self,
        database_name: &str,
        table_name: &str,
        values: &[String],
    ) -> Result<&Partition, GlueError> {
        let pk = partition_key(values);
        let key = (database_name.to_string(), table_name.to_string(), pk);
        self.partitions
            .get(&key)
            .ok_or_else(|| not_found("Partition not found.".to_string()))
    }

    pub fn batch_get_partition(
        &self,
        database_name: &str,
        table_name: &str,
        partitions_to_get: &[Vec<String>],
    ) -> Vec<Option<&Partition>> {
        partitions_to_get
            .iter()
            .map(|values| {
                let pk = partition_key(values);
                let key = (database_name.to_string(), table_name.to_string(), pk);
                self.partitions.get(&key)
            })
            .collect()
    }

    pub fn get_partitions(&self, database_name: &str, table_name: &str) -> Vec<&Partition> {
        self.partitions
            .values()
            .filter(|p| p.database_name == database_name && p.table_name == table_name)
            .collect()
    }

    pub fn delete_partition(
        &mut self,
        database_name: &str,
        table_name: &str,
        values: &[String],
    ) -> Result<(), GlueError> {
        let pk = partition_key(values);
        let key = (database_name.to_string(), table_name.to_string(), pk);
        self.partitions
            .remove(&key)
            .ok_or_else(|| not_found("Partition not found.".to_string()))?;
        Ok(())
    }

    pub fn batch_delete_partition(
        &mut self,
        database_name: &str,
        table_name: &str,
        partitions_to_delete: &[Vec<String>],
    ) -> Vec<Option<GlueError>> {
        partitions_to_delete
            .iter()
            .map(|values| {
                self.delete_partition(database_name, table_name, values)
                    .err()
            })
            .collect()
    }

    pub fn update_partition(
        &mut self,
        database_name: &str,
        table_name: &str,
        values: &[String],
        parameters: Option<HashMap<String, String>>,
        storage_descriptor: Option<serde_json::Value>,
    ) -> Result<(), GlueError> {
        let pk = partition_key(values);
        let key = (database_name.to_string(), table_name.to_string(), pk);
        let p = self
            .partitions
            .get_mut(&key)
            .ok_or_else(|| not_found("Partition not found.".to_string()))?;
        if let Some(params) = parameters {
            p.parameters = params;
        }
        if let Some(sd) = storage_descriptor {
            p.storage_descriptor = Some(sd);
        }
        Ok(())
    }

    pub fn batch_update_partition(
        &mut self,
        database_name: &str,
        table_name: &str,
        entries: Vec<(
            Vec<String>,
            HashMap<String, String>,
            Option<serde_json::Value>,
        )>,
    ) -> Vec<Option<GlueError>> {
        entries
            .into_iter()
            .map(|(values, params, sd)| {
                self.update_partition(database_name, table_name, &values, Some(params), sd)
                    .err()
            })
            .collect()
    }

    // ─── Connection operations ───

    pub fn create_connection(
        &mut self,
        name: &str,
        connection_type: &str,
        connection_properties: HashMap<String, String>,
        description: &str,
        match_criteria: Vec<String>,
        physical_connection_requirements: Option<serde_json::Value>,
    ) -> Result<(), GlueError> {
        if self.connections.contains_key(name) {
            return Err(already_exists(format!("Connection {name} already exists.")));
        }
        let now = Utc::now();
        let conn = Connection {
            name: name.to_string(),
            connection_type: connection_type.to_string(),
            connection_properties,
            description: description.to_string(),
            creation_time: now,
            last_updated_time: now,
            match_criteria,
            physical_connection_requirements,
        };
        self.connections.insert(name.to_string(), conn);
        Ok(())
    }

    pub fn get_connection(&self, name: &str) -> Result<&Connection, GlueError> {
        self.connections
            .get(name)
            .ok_or_else(|| not_found(format!("Connection {name} not found.")))
    }

    pub fn get_connections(&self) -> Vec<&Connection> {
        self.connections.values().collect()
    }

    pub fn delete_connection(&mut self, name: &str) -> Result<(), GlueError> {
        self.connections
            .remove(name)
            .ok_or_else(|| not_found(format!("Connection {name} not found.")))?;
        Ok(())
    }

    pub fn update_connection(
        &mut self,
        name: &str,
        connection_type: Option<&str>,
        connection_properties: Option<HashMap<String, String>>,
        description: Option<&str>,
        match_criteria: Option<Vec<String>>,
        physical_connection_requirements: Option<serde_json::Value>,
    ) -> Result<(), GlueError> {
        let conn = self
            .connections
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Connection {name} not found.")))?;
        if let Some(ct) = connection_type {
            conn.connection_type = ct.to_string();
        }
        if let Some(cp) = connection_properties {
            conn.connection_properties = cp;
        }
        if let Some(d) = description {
            conn.description = d.to_string();
        }
        if let Some(mc) = match_criteria {
            conn.match_criteria = mc;
        }
        if let Some(pcr) = physical_connection_requirements {
            conn.physical_connection_requirements = Some(pcr);
        }
        conn.last_updated_time = Utc::now();
        Ok(())
    }

    pub fn batch_delete_connection(
        &mut self,
        names: &[String],
    ) -> (Vec<String>, HashMap<String, GlueError>) {
        let mut succeeded = Vec::new();
        let mut errors = HashMap::new();
        for name in names {
            match self.delete_connection(name) {
                Ok(()) => succeeded.push(name.clone()),
                Err(e) => {
                    errors.insert(name.clone(), e);
                }
            }
        }
        (succeeded, errors)
    }

    // ─── Crawler operations ───

    pub fn create_crawler(
        &mut self,
        name: &str,
        role: &str,
        database_name: &str,
        description: &str,
        targets: Option<serde_json::Value>,
        schedule: Option<&str>,
        classifiers: Vec<String>,
        table_prefix: &str,
        configuration: &str,
    ) -> Result<(), GlueError> {
        if self.crawlers.contains_key(name) {
            return Err(already_exists(format!("Crawler {name} already exists.")));
        }
        let now = Utc::now();
        let c = Crawler {
            name: name.to_string(),
            role: role.to_string(),
            database_name: database_name.to_string(),
            description: description.to_string(),
            targets,
            schedule: schedule.map(|s| s.to_string()),
            classifiers,
            table_prefix: table_prefix.to_string(),
            configuration: configuration.to_string(),
            state: "READY".to_string(),
            creation_time: now,
            last_updated: now,
            version: 1,
        };
        self.crawlers.insert(name.to_string(), c);
        Ok(())
    }

    pub fn get_crawler(&self, name: &str) -> Result<&Crawler, GlueError> {
        self.crawlers
            .get(name)
            .ok_or_else(|| not_found(format!("Crawler {name} not found.")))
    }

    pub fn get_crawlers(&self) -> Vec<&Crawler> {
        self.crawlers.values().collect()
    }

    pub fn batch_get_crawlers(&self, names: &[String]) -> (Vec<&Crawler>, Vec<String>) {
        let mut found = Vec::new();
        let mut not_found_names = Vec::new();
        for name in names {
            if let Some(c) = self.crawlers.get(name) {
                found.push(c);
            } else {
                not_found_names.push(name.clone());
            }
        }
        (found, not_found_names)
    }

    pub fn list_crawlers(&self) -> Vec<String> {
        self.crawlers.keys().cloned().collect()
    }

    pub fn delete_crawler(&mut self, name: &str) -> Result<(), GlueError> {
        self.crawlers
            .remove(name)
            .ok_or_else(|| not_found(format!("Crawler {name} not found.")))?;
        Ok(())
    }

    pub fn start_crawler(&mut self, name: &str) -> Result<(), GlueError> {
        let c = self
            .crawlers
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Crawler {name} not found.")))?;
        c.state = "RUNNING".to_string();
        // Create a crawl record
        let crawl_id = Uuid::new_v4().to_string();
        self.crawl_records.push(CrawlRecord {
            crawl_id,
            crawler_name: name.to_string(),
            state: "RUNNING".to_string(),
            start_time: Utc::now(),
            end_time: None,
        });
        Ok(())
    }

    pub fn stop_crawler(&mut self, name: &str) -> Result<(), GlueError> {
        let c = self
            .crawlers
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Crawler {name} not found.")))?;
        if c.state != "RUNNING" {
            return Err(invalid_input(format!("Crawler {name} is not running.")));
        }
        c.state = "STOPPING".to_string();
        Ok(())
    }

    pub fn list_crawls(&self, crawler_name: &str) -> Result<Vec<&CrawlRecord>, GlueError> {
        // Check crawler exists
        self.get_crawler(crawler_name)?;
        let records: Vec<&CrawlRecord> = self
            .crawl_records
            .iter()
            .filter(|r| r.crawler_name == crawler_name)
            .collect();
        Ok(records)
    }

    pub fn update_crawler(
        &mut self,
        name: &str,
        role: Option<&str>,
        database_name: Option<&str>,
        description: Option<&str>,
        targets: Option<serde_json::Value>,
        schedule: Option<Option<&str>>,
        classifiers: Option<Vec<String>>,
        table_prefix: Option<&str>,
        configuration: Option<&str>,
    ) -> Result<(), GlueError> {
        let c = self
            .crawlers
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Crawler {name} not found.")))?;
        if let Some(r) = role {
            c.role = r.to_string();
        }
        if let Some(d) = database_name {
            c.database_name = d.to_string();
        }
        if let Some(d) = description {
            c.description = d.to_string();
        }
        if let Some(t) = targets {
            c.targets = Some(t);
        }
        if let Some(s) = schedule {
            c.schedule = s.map(|v| v.to_string());
        }
        if let Some(cls) = classifiers {
            c.classifiers = cls;
        }
        if let Some(p) = table_prefix {
            c.table_prefix = p.to_string();
        }
        if let Some(cfg) = configuration {
            c.configuration = cfg.to_string();
        }
        c.last_updated = Utc::now();
        c.version += 1;
        Ok(())
    }

    pub fn update_crawler_schedule(
        &mut self,
        name: &str,
        schedule_expression: Option<&str>,
    ) -> Result<(), GlueError> {
        let c = self
            .crawlers
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Crawler {name} not found.")))?;
        c.schedule = schedule_expression.map(|s| s.to_string());
        Ok(())
    }

    pub fn start_crawler_schedule(&mut self, name: &str) -> Result<(), GlueError> {
        // Ensure crawler exists; schedule state is implicit
        self.get_crawler(name)?;
        Ok(())
    }

    pub fn stop_crawler_schedule(&mut self, name: &str) -> Result<(), GlueError> {
        // Ensure crawler exists; schedule state is implicit
        self.get_crawler(name)?;
        Ok(())
    }

    pub fn get_crawler_metrics(&self) -> Vec<CrawlerMetrics> {
        self.crawlers
            .values()
            .map(|c| CrawlerMetrics {
                crawler_name: c.name.clone(),
                time_left_seconds: 0.0,
                still_estimating: false,
                last_runtime_seconds: 0.0,
                median_runtime_seconds: 0.0,
                tables_created: 0,
                tables_updated: 0,
                tables_deleted: 0,
            })
            .collect()
    }

    // ─── Job operations ───

    pub fn create_job(
        &mut self,
        name: &str,
        description: &str,
        role: &str,
        command: Option<serde_json::Value>,
        default_arguments: HashMap<String, String>,
        max_retries: i32,
        timeout: i32,
        max_capacity: Option<f64>,
        number_of_workers: Option<i32>,
        worker_type: Option<String>,
        glue_version: Option<String>,
    ) -> Result<String, GlueError> {
        if self.jobs.contains_key(name) {
            return Err(already_exists(format!("Job {name} already exists.")));
        }
        let now = Utc::now();
        let j = Job {
            name: name.to_string(),
            description: description.to_string(),
            role: role.to_string(),
            command,
            default_arguments,
            max_retries,
            timeout,
            max_capacity,
            number_of_workers,
            worker_type,
            glue_version,
            created_on: now,
            last_modified_on: now,
        };
        self.jobs.insert(name.to_string(), j);
        Ok(name.to_string())
    }

    pub fn get_job(&self, name: &str) -> Result<&Job, GlueError> {
        self.jobs
            .get(name)
            .ok_or_else(|| not_found(format!("Job {name} not found.")))
    }

    pub fn get_jobs(&self) -> Vec<&Job> {
        self.jobs.values().collect()
    }

    pub fn batch_get_jobs(&self, names: &[String]) -> (Vec<&Job>, Vec<String>) {
        let mut found = Vec::new();
        let mut not_found_names = Vec::new();
        for name in names {
            if let Some(j) = self.jobs.get(name) {
                found.push(j);
            } else {
                not_found_names.push(name.clone());
            }
        }
        (found, not_found_names)
    }

    pub fn list_jobs(&self) -> Vec<String> {
        self.jobs.keys().cloned().collect()
    }

    pub fn delete_job(&mut self, name: &str) -> Result<String, GlueError> {
        self.jobs
            .remove(name)
            .ok_or_else(|| not_found(format!("Job {name} not found.")))?;
        Ok(name.to_string())
    }

    pub fn start_job_run(
        &mut self,
        job_name: &str,
        arguments: HashMap<String, String>,
        timeout: Option<i32>,
        max_capacity: Option<f64>,
        number_of_workers: Option<i32>,
        worker_type: Option<String>,
    ) -> Result<String, GlueError> {
        let job = self
            .jobs
            .get(job_name)
            .ok_or_else(|| not_found(format!("Job {job_name} not found.")))?;
        let run_id = format!("jr_{}", Uuid::new_v4());
        let jr = JobRun {
            id: run_id.clone(),
            job_name: job_name.to_string(),
            started_on: Utc::now(),
            completed_on: None,
            job_run_state: "RUNNING".to_string(),
            arguments,
            timeout: timeout.unwrap_or(job.timeout),
            max_capacity: max_capacity.or(job.max_capacity),
            number_of_workers: number_of_workers.or(job.number_of_workers),
            worker_type: worker_type.or_else(|| job.worker_type.clone()),
        };
        self.job_runs
            .insert((job_name.to_string(), run_id.clone()), jr);
        Ok(run_id)
    }

    pub fn get_job_run(&self, job_name: &str, run_id: &str) -> Result<&JobRun, GlueError> {
        let key = (job_name.to_string(), run_id.to_string());
        self.job_runs
            .get(&key)
            .ok_or_else(|| not_found(format!("JobRun {run_id} not found.")))
    }

    pub fn get_job_runs(&self, job_name: &str) -> Vec<&JobRun> {
        self.job_runs
            .values()
            .filter(|jr| jr.job_name == job_name)
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_job(
        &mut self,
        name: &str,
        description: Option<&str>,
        role: Option<&str>,
        command: Option<serde_json::Value>,
        default_arguments: Option<HashMap<String, String>>,
        max_retries: Option<i32>,
        timeout: Option<i32>,
        max_capacity: Option<f64>,
        number_of_workers: Option<i32>,
        worker_type: Option<String>,
        glue_version: Option<String>,
    ) -> Result<String, GlueError> {
        let j = self
            .jobs
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Job {name} not found.")))?;
        if let Some(d) = description {
            j.description = d.to_string();
        }
        if let Some(r) = role {
            j.role = r.to_string();
        }
        if let Some(cmd) = command {
            j.command = Some(cmd);
        }
        if let Some(da) = default_arguments {
            j.default_arguments = da;
        }
        if let Some(mr) = max_retries {
            j.max_retries = mr;
        }
        if let Some(t) = timeout {
            j.timeout = t;
        }
        if let Some(mc) = max_capacity {
            j.max_capacity = Some(mc);
        }
        if let Some(nw) = number_of_workers {
            j.number_of_workers = Some(nw);
        }
        if let Some(wt) = worker_type {
            j.worker_type = Some(wt);
        }
        if let Some(gv) = glue_version {
            j.glue_version = Some(gv);
        }
        j.last_modified_on = Utc::now();
        Ok(name.to_string())
    }

    pub fn batch_stop_job_run(
        &mut self,
        job_name: &str,
        job_run_ids: &[String],
    ) -> (Vec<(String, String)>, Vec<(String, String, GlueError)>) {
        let mut succeeded = Vec::new();
        let mut errors = Vec::new();
        for run_id in job_run_ids {
            let key = (job_name.to_string(), run_id.clone());
            if let Some(jr) = self.job_runs.get_mut(&key) {
                jr.job_run_state = "STOPPED".to_string();
                jr.completed_on = Some(Utc::now());
                succeeded.push((job_name.to_string(), run_id.clone()));
            } else {
                errors.push((
                    job_name.to_string(),
                    run_id.clone(),
                    not_found(format!("JobRun {run_id} not found.")),
                ));
            }
        }
        (succeeded, errors)
    }

    pub fn get_job_bookmark(&self, job_name: &str) -> Result<(), GlueError> {
        // Just verify the job exists; bookmark state is not maintained in depth.
        self.get_job(job_name)?;
        Ok(())
    }

    pub fn reset_job_bookmark(&mut self, job_name: &str) -> Result<(), GlueError> {
        // Verify the job exists; no actual bookmark state to reset.
        self.get_job(job_name)?;
        Ok(())
    }

    // ─── Trigger operations ───

    pub fn create_trigger(
        &mut self,
        name: &str,
        trigger_type: &str,
        description: &str,
        schedule: Option<&str>,
        actions: Option<serde_json::Value>,
        predicate: Option<serde_json::Value>,
        workflow_name: Option<&str>,
    ) -> Result<String, GlueError> {
        if self.triggers.contains_key(name) {
            return Err(already_exists(format!("Trigger {name} already exists.")));
        }
        let t = Trigger {
            name: name.to_string(),
            trigger_type: trigger_type.to_string(),
            state: "CREATED".to_string(),
            description: description.to_string(),
            schedule: schedule.map(|s| s.to_string()),
            actions,
            predicate,
            workflow_name: workflow_name.map(|s| s.to_string()),
        };
        self.triggers.insert(name.to_string(), t);
        Ok(name.to_string())
    }

    pub fn get_trigger(&self, name: &str) -> Result<&Trigger, GlueError> {
        self.triggers
            .get(name)
            .ok_or_else(|| not_found(format!("Trigger {name} not found.")))
    }

    pub fn get_triggers(&self) -> Vec<&Trigger> {
        self.triggers.values().collect()
    }

    pub fn batch_get_triggers(&self, names: &[String]) -> (Vec<&Trigger>, Vec<String>) {
        let mut found = Vec::new();
        let mut not_found_names = Vec::new();
        for name in names {
            if let Some(t) = self.triggers.get(name) {
                found.push(t);
            } else {
                not_found_names.push(name.clone());
            }
        }
        (found, not_found_names)
    }

    pub fn list_triggers(&self) -> Vec<String> {
        self.triggers.keys().cloned().collect()
    }

    pub fn delete_trigger(&mut self, name: &str) -> Result<String, GlueError> {
        self.triggers
            .remove(name)
            .ok_or_else(|| not_found(format!("Trigger {name} not found.")))?;
        Ok(name.to_string())
    }

    pub fn start_trigger(&mut self, name: &str) -> Result<String, GlueError> {
        let t = self
            .triggers
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Trigger {name} not found.")))?;
        t.state = "ACTIVATED".to_string();
        Ok(name.to_string())
    }

    pub fn stop_trigger(&mut self, name: &str) -> Result<String, GlueError> {
        let t = self
            .triggers
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Trigger {name} not found.")))?;
        t.state = "DEACTIVATED".to_string();
        Ok(name.to_string())
    }

    pub fn update_trigger(
        &mut self,
        name: &str,
        description: Option<&str>,
        schedule: Option<Option<&str>>,
        actions: Option<serde_json::Value>,
        predicate: Option<serde_json::Value>,
    ) -> Result<&Trigger, GlueError> {
        let t = self
            .triggers
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Trigger {name} not found.")))?;
        if let Some(d) = description {
            t.description = d.to_string();
        }
        if let Some(s) = schedule {
            t.schedule = s.map(|v| v.to_string());
        }
        if let Some(a) = actions {
            t.actions = Some(a);
        }
        if let Some(p) = predicate {
            t.predicate = Some(p);
        }
        Ok(self.triggers.get(name).unwrap())
    }

    // ─── Workflow operations ───

    pub fn create_workflow(
        &mut self,
        name: &str,
        description: &str,
        default_run_properties: HashMap<String, String>,
        max_concurrent_runs: Option<i32>,
    ) -> Result<String, GlueError> {
        if self.workflows.contains_key(name) {
            return Err(already_exists(format!("Workflow {name} already exists.")));
        }
        let now = Utc::now();
        let w = Workflow {
            name: name.to_string(),
            description: description.to_string(),
            default_run_properties,
            created_on: now,
            last_modified_on: now,
            max_concurrent_runs,
            runs: Vec::new(),
        };
        self.workflows.insert(name.to_string(), w);
        Ok(name.to_string())
    }

    pub fn get_workflow(&self, name: &str) -> Result<&Workflow, GlueError> {
        self.workflows
            .get(name)
            .ok_or_else(|| not_found(format!("Workflow {name} not found.")))
    }

    pub fn delete_workflow(&mut self, name: &str) -> Result<String, GlueError> {
        // Moto behavior: delete_workflow is idempotent (no error on missing)
        self.workflows.remove(name);
        Ok(name.to_string())
    }

    pub fn list_workflows(&self) -> Vec<String> {
        self.workflows.keys().cloned().collect()
    }

    pub fn update_workflow(
        &mut self,
        name: &str,
        description: Option<&str>,
        default_run_properties: Option<HashMap<String, String>>,
        max_concurrent_runs: Option<i32>,
    ) -> Result<String, GlueError> {
        let w = self
            .workflows
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Workflow {name} not found.")))?;
        if let Some(d) = description {
            w.description = d.to_string();
        }
        if let Some(p) = default_run_properties {
            w.default_run_properties = p;
        }
        if let Some(m) = max_concurrent_runs {
            w.max_concurrent_runs = Some(m);
        }
        w.last_modified_on = Utc::now();
        Ok(name.to_string())
    }

    pub fn start_workflow_run(&mut self, name: &str) -> Result<String, GlueError> {
        let w = self
            .workflows
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Workflow {name} not found.")))?;
        let run_id = format!("wr_{}", Uuid::new_v4());
        let run = WorkflowRun {
            workflow_run_id: run_id.clone(),
            name: name.to_string(),
            started_on: Utc::now(),
            completed_on: None,
            status: "RUNNING".to_string(),
            run_properties: w.default_run_properties.clone(),
        };
        w.runs.push(run);
        Ok(run_id)
    }

    pub fn stop_workflow_run(&mut self, name: &str, run_id: &str) -> Result<(), GlueError> {
        let w = self
            .workflows
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Workflow {name} not found.")))?;
        let run = w
            .runs
            .iter_mut()
            .find(|r| r.workflow_run_id == run_id)
            .ok_or_else(|| not_found(format!("WorkflowRun {run_id} not found.")))?;
        run.status = "STOPPED".to_string();
        run.completed_on = Some(Utc::now());
        Ok(())
    }

    pub fn get_workflow_run(&self, name: &str, run_id: &str) -> Result<&WorkflowRun, GlueError> {
        let w = self.get_workflow(name)?;
        w.runs
            .iter()
            .find(|r| r.workflow_run_id == run_id)
            .ok_or_else(|| not_found(format!("WorkflowRun {run_id} not found.")))
    }

    pub fn get_workflow_runs(&self, name: &str) -> Result<Vec<&WorkflowRun>, GlueError> {
        let w = self.get_workflow(name)?;
        Ok(w.runs.iter().collect())
    }

    pub fn get_workflow_run_properties(
        &self,
        name: &str,
        run_id: &str,
    ) -> Result<&HashMap<String, String>, GlueError> {
        let run = self.get_workflow_run(name, run_id)?;
        Ok(&run.run_properties)
    }

    pub fn put_workflow_run_properties(
        &mut self,
        name: &str,
        run_id: &str,
        run_properties: HashMap<String, String>,
    ) -> Result<(), GlueError> {
        let w = self
            .workflows
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Workflow {name} not found.")))?;
        let run = w
            .runs
            .iter_mut()
            .find(|r| r.workflow_run_id == run_id)
            .ok_or_else(|| not_found(format!("WorkflowRun {run_id} not found.")))?;
        run.run_properties = run_properties;
        Ok(())
    }

    pub fn batch_get_workflows(&self, names: &[String]) -> (Vec<&Workflow>, Vec<String>) {
        let mut found = Vec::new();
        let mut missing = Vec::new();
        for name in names {
            if let Some(w) = self.workflows.get(name) {
                found.push(w);
            } else {
                missing.push(name.clone());
            }
        }
        (found, missing)
    }

    pub fn resume_workflow_run(
        &mut self,
        name: &str,
        run_id: &str,
    ) -> Result<(String, Vec<String>), GlueError> {
        let w = self
            .workflows
            .get_mut(name)
            .ok_or_else(|| not_found(format!("Workflow {name} not found.")))?;
        let run = w
            .runs
            .iter_mut()
            .find(|r| r.workflow_run_id == run_id)
            .ok_or_else(|| not_found(format!("WorkflowRun {run_id} not found.")))?;
        run.status = "RUNNING".to_string();
        run.completed_on = None;
        Ok((run_id.to_string(), vec![]))
    }

    // ─── DevEndpoint operations ───

    pub fn create_dev_endpoint(
        &mut self,
        endpoint_name: &str,
        role_arn: &str,
        security_group_ids: Vec<String>,
        subnet_id: &str,
        number_of_nodes: i32,
        number_of_workers: Option<i32>,
        worker_type: Option<String>,
        glue_version: Option<String>,
        public_key: Option<String>,
        public_keys: Vec<String>,
        extra_python_libs_s3_path: Option<String>,
        extra_jars_s3_path: Option<String>,
        arguments: HashMap<String, String>,
    ) -> Result<&DevEndpoint, GlueError> {
        if self.dev_endpoints.contains_key(endpoint_name) {
            return Err(already_exists(format!(
                "DevEndpoint {endpoint_name} already exists."
            )));
        }
        let now = Utc::now();
        let de = DevEndpoint {
            endpoint_name: endpoint_name.to_string(),
            role_arn: role_arn.to_string(),
            security_group_ids,
            subnet_id: subnet_id.to_string(),
            number_of_nodes,
            number_of_workers,
            worker_type,
            glue_version,
            status: "READY".to_string(),
            created_timestamp: now,
            last_modified_timestamp: now,
            public_key,
            public_keys,
            extra_python_libs_s3_path,
            extra_jars_s3_path,
            arguments,
        };
        self.dev_endpoints.insert(endpoint_name.to_string(), de);
        Ok(self.dev_endpoints.get(endpoint_name).unwrap())
    }

    pub fn get_dev_endpoint(&self, name: &str) -> Result<&DevEndpoint, GlueError> {
        self.dev_endpoints
            .get(name)
            .ok_or_else(|| not_found(format!("DevEndpoint {name} not found.")))
    }

    pub fn get_dev_endpoints(&self) -> Vec<&DevEndpoint> {
        self.dev_endpoints.values().collect()
    }

    pub fn delete_dev_endpoint(&mut self, name: &str) -> Result<(), GlueError> {
        self.dev_endpoints
            .remove(name)
            .ok_or_else(|| not_found(format!("DevEndpoint {name} not found.")))?;
        Ok(())
    }

    pub fn list_dev_endpoints(&self) -> Vec<String> {
        self.dev_endpoints.keys().cloned().collect()
    }

    pub fn batch_get_dev_endpoints(&self, names: &[String]) -> (Vec<&DevEndpoint>, Vec<String>) {
        let mut found = Vec::new();
        let mut not_found_names = Vec::new();
        for name in names {
            if let Some(de) = self.dev_endpoints.get(name) {
                found.push(de);
            } else {
                not_found_names.push(name.clone());
            }
        }
        (found, not_found_names)
    }

    pub fn update_dev_endpoint(
        &mut self,
        name: &str,
        public_key: Option<String>,
        add_public_keys: Vec<String>,
        delete_public_keys: Vec<String>,
        add_arguments: HashMap<String, String>,
        delete_arguments: Vec<String>,
        glue_version: Option<String>,
    ) -> Result<(), GlueError> {
        let de = self
            .dev_endpoints
            .get_mut(name)
            .ok_or_else(|| not_found(format!("DevEndpoint {name} not found.")))?;
        if let Some(pk) = public_key {
            de.public_key = Some(pk);
        }
        for pk in add_public_keys {
            if !de.public_keys.contains(&pk) {
                de.public_keys.push(pk);
            }
        }
        de.public_keys.retain(|pk| !delete_public_keys.contains(pk));
        for (k, v) in add_arguments {
            de.arguments.insert(k, v);
        }
        for k in delete_arguments {
            de.arguments.remove(&k);
        }
        if let Some(gv) = glue_version {
            de.glue_version = Some(gv);
        }
        de.last_modified_timestamp = Utc::now();
        Ok(())
    }

    // ─── SecurityConfiguration operations ───

    pub fn create_security_configuration(
        &mut self,
        name: &str,
        encryption_configuration: Option<serde_json::Value>,
    ) -> Result<String, GlueError> {
        if self.security_configurations.contains_key(name) {
            return Err(already_exists(format!(
                "SecurityConfiguration {name} already exists."
            )));
        }
        let sc = SecurityConfiguration {
            name: name.to_string(),
            created_time_stamp: Utc::now(),
            encryption_configuration,
        };
        self.security_configurations.insert(name.to_string(), sc);
        Ok(name.to_string())
    }

    pub fn get_security_configuration(
        &self,
        name: &str,
    ) -> Result<&SecurityConfiguration, GlueError> {
        self.security_configurations
            .get(name)
            .ok_or_else(|| not_found(format!("SecurityConfiguration {name} not found.")))
    }

    pub fn get_security_configurations(&self) -> Vec<&SecurityConfiguration> {
        self.security_configurations.values().collect()
    }

    pub fn delete_security_configuration(&mut self, name: &str) -> Result<(), GlueError> {
        self.security_configurations
            .remove(name)
            .ok_or_else(|| not_found(format!("SecurityConfiguration {name} not found.")))?;
        Ok(())
    }

    // ─── Session operations ───

    pub fn create_session(
        &mut self,
        id: &str,
        description: &str,
        role: &str,
        command: Option<serde_json::Value>,
        glue_version: Option<String>,
        max_capacity: Option<f64>,
        number_of_workers: Option<i32>,
        worker_type: Option<String>,
        idle_timeout: Option<i32>,
        default_arguments: HashMap<String, String>,
        security_configuration: Option<String>,
    ) -> Result<&Session, GlueError> {
        if self.sessions.contains_key(id) {
            return Err(already_exists(format!("Session {id} already exists.")));
        }
        let s = Session {
            id: id.to_string(),
            description: description.to_string(),
            role: role.to_string(),
            command,
            status: "READY".to_string(),
            created_on: Utc::now(),
            glue_version,
            max_capacity,
            number_of_workers,
            worker_type,
            idle_timeout,
            default_arguments,
            security_configuration,
        };
        self.sessions.insert(id.to_string(), s);
        Ok(self.sessions.get(id).unwrap())
    }

    pub fn get_session(&self, id: &str) -> Result<&Session, GlueError> {
        self.sessions
            .get(id)
            .ok_or_else(|| not_found(format!("Session {id} not found.")))
    }

    pub fn list_sessions(&self) -> Vec<&Session> {
        self.sessions.values().collect()
    }

    pub fn delete_session(&mut self, id: &str) -> Result<String, GlueError> {
        self.sessions
            .remove(id)
            .ok_or_else(|| not_found(format!("Session {id} not found.")))?;
        Ok(id.to_string())
    }

    pub fn stop_session(&mut self, id: &str) -> Result<String, GlueError> {
        let s = self
            .sessions
            .get_mut(id)
            .ok_or_else(|| not_found(format!("Session {id} not found.")))?;
        s.status = "STOPPING".to_string();
        Ok(id.to_string())
    }

    // ─── Registry operations ───

    pub fn create_registry(
        &mut self,
        registry_name: &str,
        description: &str,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Registry, GlueError> {
        if self.registries.contains_key(registry_name) {
            return Err(already_exists(format!(
                "Registry {registry_name} already exists."
            )));
        }
        let now = Utc::now();
        let arn = format!("arn:aws:glue:{region}:{account_id}:registry/{registry_name}");
        let r = Registry {
            registry_name: registry_name.to_string(),
            registry_arn: arn,
            description: description.to_string(),
            created_time: now,
            updated_time: now,
            status: "AVAILABLE".to_string(),
            tags,
        };
        self.registries.insert(registry_name.to_string(), r);
        Ok(self.registries.get(registry_name).unwrap())
    }

    pub fn get_registry(&self, registry_name: &str) -> Result<&Registry, GlueError> {
        // Try by name first, then by ARN
        if let Some(r) = self.registries.get(registry_name) {
            return Ok(r);
        }
        // Check if it's an ARN
        for r in self.registries.values() {
            if r.registry_arn == registry_name {
                return Ok(r);
            }
        }
        Err(not_found(format!("Registry {registry_name} not found.")))
    }

    pub fn list_registries(&self) -> Vec<&Registry> {
        self.registries.values().collect()
    }

    pub fn delete_registry(&mut self, registry_name: &str) -> Result<Registry, GlueError> {
        // Find by name or ARN
        let key = if self.registries.contains_key(registry_name) {
            registry_name.to_string()
        } else {
            self.registries
                .iter()
                .find(|(_, r)| r.registry_arn == registry_name)
                .map(|(k, _)| k.clone())
                .ok_or_else(|| not_found(format!("Registry {registry_name} not found.")))?
        };
        let r = self.registries.remove(&key).unwrap();
        Ok(r)
    }

    pub fn update_registry(
        &mut self,
        registry_name: &str,
        description: &str,
    ) -> Result<&Registry, GlueError> {
        let key = if self.registries.contains_key(registry_name) {
            registry_name.to_string()
        } else {
            self.registries
                .iter()
                .find(|(_, r)| r.registry_arn == registry_name)
                .map(|(k, _)| k.clone())
                .ok_or_else(|| not_found(format!("Registry {registry_name} not found.")))?
        };
        let r = self.registries.get_mut(&key).unwrap();
        r.description = description.to_string();
        r.updated_time = Utc::now();
        Ok(self.registries.get(&key).unwrap())
    }

    // ─── Schema operations ───

    pub fn create_schema(
        &mut self,
        schema_name: &str,
        registry_name: &str,
        data_format: &str,
        compatibility: &str,
        description: &str,
        schema_definition: Option<&str>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Schema, GlueError> {
        if self.schemas.contains_key(schema_name) {
            return Err(already_exists(format!(
                "Schema {schema_name} already exists."
            )));
        }
        let reg = self.get_registry(registry_name)?;
        let registry_arn = reg.registry_arn.clone();
        let reg_name = reg.registry_name.clone();

        let now = Utc::now();
        let schema_arn =
            format!("arn:aws:glue:{region}:{account_id}:schema/{reg_name}/{schema_name}");

        let mut versions = Vec::new();
        let (latest_ver, next_ver) = if let Some(def) = schema_definition {
            let sv = SchemaVersion {
                schema_version_id: Uuid::new_v4().to_string(),
                version_number: 1,
                schema_definition: def.to_string(),
                status: "AVAILABLE".to_string(),
                created_time: now,
                metadata: HashMap::new(),
            };
            versions.push(sv);
            (1i64, 2i64)
        } else {
            (0i64, 1i64)
        };

        let s = Schema {
            schema_name: schema_name.to_string(),
            schema_arn,
            registry_name: reg_name,
            registry_arn,
            data_format: data_format.to_string(),
            compatibility: compatibility.to_string(),
            description: description.to_string(),
            schema_status: "AVAILABLE".to_string(),
            created_time: now,
            updated_time: now,
            latest_schema_version: latest_ver,
            next_schema_version: next_ver,
            schema_checkpoint: latest_ver,
            tags,
            versions,
        };
        self.schemas.insert(schema_name.to_string(), s);
        Ok(self.schemas.get(schema_name).unwrap())
    }

    pub fn get_schema(&self, schema_name: &str) -> Result<&Schema, GlueError> {
        if let Some(s) = self.schemas.get(schema_name) {
            return Ok(s);
        }
        for s in self.schemas.values() {
            if s.schema_arn == schema_name {
                return Ok(s);
            }
        }
        Err(not_found(format!("Schema {schema_name} not found.")))
    }

    pub fn delete_schema(&mut self, schema_name: &str) -> Result<Schema, GlueError> {
        let key = if self.schemas.contains_key(schema_name) {
            schema_name.to_string()
        } else {
            self.schemas
                .iter()
                .find(|(_, s)| s.schema_arn == schema_name)
                .map(|(k, _)| k.clone())
                .ok_or_else(|| not_found(format!("Schema {schema_name} not found.")))?
        };
        let s = self.schemas.remove(&key).unwrap();
        Ok(s)
    }

    pub fn update_schema(
        &mut self,
        schema_name: &str,
        compatibility: Option<&str>,
        description: Option<&str>,
    ) -> Result<&Schema, GlueError> {
        let key = if self.schemas.contains_key(schema_name) {
            schema_name.to_string()
        } else {
            self.schemas
                .iter()
                .find(|(_, s)| s.schema_arn == schema_name)
                .map(|(k, _)| k.clone())
                .ok_or_else(|| not_found(format!("Schema {schema_name} not found.")))?
        };
        let s = self.schemas.get_mut(&key).unwrap();
        if let Some(c) = compatibility {
            s.compatibility = c.to_string();
        }
        if let Some(d) = description {
            s.description = d.to_string();
        }
        s.updated_time = Utc::now();
        Ok(self.schemas.get(&key).unwrap())
    }

    pub fn register_schema_version(
        &mut self,
        schema_name: &str,
        schema_definition: &str,
    ) -> Result<&SchemaVersion, GlueError> {
        let key = if self.schemas.contains_key(schema_name) {
            schema_name.to_string()
        } else {
            self.schemas
                .iter()
                .find(|(_, s)| s.schema_arn == schema_name)
                .map(|(k, _)| k.clone())
                .ok_or_else(|| not_found(format!("Schema {schema_name} not found.")))?
        };
        let s = self.schemas.get_mut(&key).unwrap();
        let ver_num = s.next_schema_version;
        let sv = SchemaVersion {
            schema_version_id: Uuid::new_v4().to_string(),
            version_number: ver_num,
            schema_definition: schema_definition.to_string(),
            status: "AVAILABLE".to_string(),
            created_time: Utc::now(),
            metadata: HashMap::new(),
        };
        s.versions.push(sv);
        s.latest_schema_version = ver_num;
        s.next_schema_version = ver_num + 1;
        s.schema_checkpoint = ver_num;
        s.updated_time = Utc::now();
        Ok(s.versions.last().unwrap())
    }

    pub fn get_schema_version(
        &self,
        schema_name: &str,
        version_number: Option<i64>,
        schema_version_id: Option<&str>,
    ) -> Result<&SchemaVersion, GlueError> {
        // Find by schema_version_id across all schemas
        if let Some(svid) = schema_version_id {
            for s in self.schemas.values() {
                for v in &s.versions {
                    if v.schema_version_id == svid {
                        return Ok(v);
                    }
                }
            }
            return Err(not_found(format!("SchemaVersion {svid} not found.")));
        }

        let s = self.get_schema(schema_name)?;
        let ver = version_number.unwrap_or(s.latest_schema_version);
        s.versions
            .iter()
            .find(|v| v.version_number == ver)
            .ok_or_else(|| not_found(format!("SchemaVersion {ver} not found.")))
    }

    pub fn get_schema_by_definition(
        &self,
        schema_name: &str,
        schema_definition: &str,
    ) -> Result<&SchemaVersion, GlueError> {
        let s = self.get_schema(schema_name)?;
        s.versions
            .iter()
            .find(|v| v.schema_definition == schema_definition)
            .ok_or_else(|| not_found("Schema version not found for definition.".to_string()))
    }

    pub fn put_schema_version_metadata(
        &mut self,
        schema_version_id: &str,
        key: &str,
        value: &str,
    ) -> Result<(), GlueError> {
        for s in self.schemas.values_mut() {
            for v in &mut s.versions {
                if v.schema_version_id == schema_version_id {
                    v.metadata.insert(key.to_string(), value.to_string());
                    return Ok(());
                }
            }
        }
        Err(not_found(format!(
            "SchemaVersion {schema_version_id} not found."
        )))
    }

    pub fn list_schemas(&self, registry_name: Option<&str>) -> Vec<&Schema> {
        self.schemas
            .values()
            .filter(|s| {
                if let Some(rn) = registry_name {
                    s.registry_name == rn || s.registry_arn == rn
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn delete_schema_versions(
        &mut self,
        schema_name: &str,
        versions_range: &str,
    ) -> Vec<(i64, Option<GlueError>)> {
        let key = if self.schemas.contains_key(schema_name) {
            schema_name.to_string()
        } else if let Some(k) = self
            .schemas
            .iter()
            .find(|(_, s)| s.schema_arn == schema_name)
            .map(|(k, _)| k.clone())
        {
            k
        } else {
            return vec![];
        };

        // Parse version range like "1-5" or "1" or "LATEST"
        let ver_numbers = parse_version_range(versions_range, self.schemas.get(&key));
        let s = self.schemas.get_mut(&key).unwrap();
        let mut results = Vec::new();
        for ver in &ver_numbers {
            let before = s.versions.len();
            s.versions.retain(|v| v.version_number != *ver);
            if s.versions.len() < before {
                results.push((*ver, None));
            } else {
                results.push((*ver, Some(not_found(format!("Version {ver} not found.")))));
            }
        }
        results
    }

    pub fn check_schema_version_validity(
        &self,
        data_format: &str,
        schema_definition: &str,
    ) -> bool {
        // Simple validation: non-empty definition is considered valid
        let _ = data_format;
        !schema_definition.is_empty()
    }

    pub fn query_schema_version_metadata(
        &self,
        schema_version_id: &str,
    ) -> Result<&HashMap<String, String>, GlueError> {
        for s in self.schemas.values() {
            for v in &s.versions {
                if v.schema_version_id == schema_version_id {
                    return Ok(&v.metadata);
                }
            }
        }
        Err(not_found(format!(
            "SchemaVersion {schema_version_id} not found."
        )))
    }

    pub fn remove_schema_version_metadata(
        &mut self,
        schema_version_id: &str,
        metadata_key: &str,
    ) -> Result<(), GlueError> {
        for s in self.schemas.values_mut() {
            for v in &mut s.versions {
                if v.schema_version_id == schema_version_id {
                    v.metadata.remove(metadata_key);
                    return Ok(());
                }
            }
        }
        Err(not_found(format!(
            "SchemaVersion {schema_version_id} not found."
        )))
    }

    pub fn get_schema_versions_diff(
        &self,
        schema_name: &str,
        first_version: i64,
        second_version: i64,
        _diff_type: &str,
    ) -> Result<String, GlueError> {
        let s = self.get_schema(schema_name)?;
        let v1 = s
            .versions
            .iter()
            .find(|v| v.version_number == first_version)
            .ok_or_else(|| not_found(format!("Version {first_version} not found.")))?;
        let v2 = s
            .versions
            .iter()
            .find(|v| v.version_number == second_version)
            .ok_or_else(|| not_found(format!("Version {second_version} not found.")))?;
        // Simple diff: return a JSON string showing both definitions
        let diff = format!(
            "{{\"v{}\": {}, \"v{}\": {}}}",
            first_version,
            serde_json::to_string(&v1.schema_definition).unwrap_or_default(),
            second_version,
            serde_json::to_string(&v2.schema_definition).unwrap_or_default()
        );
        Ok(diff)
    }

    // ─── ML Transform operations ───

    pub fn create_ml_transform(
        &mut self,
        name: &str,
        description: Option<&str>,
        role: &str,
        glue_version: Option<&str>,
        max_capacity: Option<f64>,
        max_retries: Option<i32>,
        timeout: Option<i32>,
        number_of_workers: Option<i32>,
        worker_type: Option<&str>,
        parameters: Option<serde_json::Value>,
        input_record_tables: Vec<serde_json::Value>,
    ) -> Result<String, GlueError> {
        if self.ml_transforms.values().any(|t| t.name == name) {
            return Err(already_exists(format!(
                "MLTransform {name} already exists."
            )));
        }
        let transform_id = format!("tfm_{}", Uuid::new_v4());
        let now = Utc::now();
        let t = MLTransform {
            transform_id: transform_id.clone(),
            name: name.to_string(),
            description: description.unwrap_or("").to_string(),
            role: role.to_string(),
            glue_version: glue_version.map(|s| s.to_string()),
            max_capacity,
            max_retries,
            timeout,
            number_of_workers,
            worker_type: worker_type.map(|s| s.to_string()),
            parameters,
            input_record_tables,
            status: "READY".to_string(),
            created_on: now,
            last_modified_on: now,
        };
        self.ml_transforms.insert(transform_id.clone(), t);
        Ok(transform_id)
    }

    pub fn get_ml_transform(&self, transform_id: &str) -> Result<&MLTransform, GlueError> {
        self.ml_transforms
            .get(transform_id)
            .ok_or_else(|| not_found(format!("MLTransform {transform_id} not found.")))
    }

    pub fn get_ml_transforms(&self) -> Vec<&MLTransform> {
        self.ml_transforms.values().collect()
    }

    pub fn list_ml_transforms(&self) -> Vec<String> {
        self.ml_transforms.keys().cloned().collect()
    }

    pub fn delete_ml_transform(&mut self, transform_id: &str) -> Result<String, GlueError> {
        self.ml_transforms
            .remove(transform_id)
            .ok_or_else(|| not_found(format!("MLTransform {transform_id} not found.")))?;
        Ok(transform_id.to_string())
    }

    pub fn update_ml_transform(
        &mut self,
        transform_id: &str,
        description: Option<&str>,
        role: Option<&str>,
        glue_version: Option<&str>,
        max_capacity: Option<f64>,
        max_retries: Option<i32>,
        timeout: Option<i32>,
        number_of_workers: Option<i32>,
        worker_type: Option<&str>,
        parameters: Option<serde_json::Value>,
    ) -> Result<String, GlueError> {
        let t = self
            .ml_transforms
            .get_mut(transform_id)
            .ok_or_else(|| not_found(format!("MLTransform {transform_id} not found.")))?;
        if let Some(d) = description {
            t.description = d.to_string();
        }
        if let Some(r) = role {
            t.role = r.to_string();
        }
        if let Some(gv) = glue_version {
            t.glue_version = Some(gv.to_string());
        }
        if let Some(mc) = max_capacity {
            t.max_capacity = Some(mc);
        }
        if let Some(mr) = max_retries {
            t.max_retries = Some(mr);
        }
        if let Some(to) = timeout {
            t.timeout = Some(to);
        }
        if let Some(nw) = number_of_workers {
            t.number_of_workers = Some(nw);
        }
        if let Some(wt) = worker_type {
            t.worker_type = Some(wt.to_string());
        }
        if let Some(p) = parameters {
            t.parameters = Some(p);
        }
        t.last_modified_on = Utc::now();
        Ok(transform_id.to_string())
    }

    // ─── Search/Filter operations ───

    pub fn search_tables(&self, search_text: Option<&str>) -> Vec<&Table> {
        self.tables
            .values()
            .filter(|t| {
                if let Some(text) = search_text {
                    if text.is_empty() {
                        return true;
                    }
                    t.name.contains(text)
                        || t.description.contains(text)
                        || t.database_name.contains(text)
                } else {
                    true
                }
            })
            .collect()
    }

    // ─── Resource policy operations ───

    pub fn put_resource_policy(&mut self, policy_in_json: &str) -> String {
        let now = Utc::now();
        let hash = format!("{:x}", md5_hash(policy_in_json));
        let rp = ResourcePolicy {
            policy_in_json: policy_in_json.to_string(),
            policy_hash: hash.clone(),
            create_time: self
                .resource_policy
                .as_ref()
                .map(|p| p.create_time)
                .unwrap_or(now),
            update_time: now,
        };
        self.resource_policy = Some(rp);
        hash
    }

    pub fn get_resource_policy(&self) -> Result<&ResourcePolicy, GlueError> {
        self.resource_policy
            .as_ref()
            .ok_or_else(|| not_found("Resource policy not found.".to_string()))
    }

    pub fn delete_resource_policy(&mut self) -> Result<(), GlueError> {
        self.resource_policy = None;
        Ok(())
    }

    // ─── Data catalog encryption settings ───

    pub fn put_data_catalog_encryption_settings(
        &mut self,
        encryption_at_rest: Option<serde_json::Value>,
        connection_password_encryption: Option<serde_json::Value>,
    ) {
        self.data_catalog_encryption_settings = Some(DataCatalogEncryptionSettings {
            encryption_at_rest,
            connection_password_encryption,
        });
    }

    pub fn get_data_catalog_encryption_settings(&self) -> Option<&DataCatalogEncryptionSettings> {
        self.data_catalog_encryption_settings.as_ref()
    }

    // ─── Tag operations ───

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        let entry = self.tags.entry(arn.to_string()).or_default();
        entry.extend(tags);
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: &[String]) {
        if let Some(entry) = self.tags.get_mut(arn) {
            for key in tag_keys {
                entry.remove(key);
            }
        }
    }

    pub fn get_tags(&self, arn: &str) -> HashMap<String, String> {
        self.tags.get(arn).cloned().unwrap_or_default()
    }
}

fn table_to_json_snapshot(tbl: &Table) -> serde_json::Value {
    serde_json::json!({
        "Name": tbl.name,
        "DatabaseName": tbl.database_name,
        "Description": tbl.description,
        "Owner": tbl.owner,
        "TableType": tbl.table_type,
        "VersionId": tbl.version_id,
    })
}

fn md5_hash(input: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

fn parse_version_range(range: &str, schema: Option<&Schema>) -> Vec<i64> {
    let trimmed = range.trim();
    if trimmed == "LATEST" {
        return schema
            .map(|s| vec![s.latest_schema_version])
            .unwrap_or_default();
    }
    if let Some((start, end)) = trimmed.split_once('-') {
        if let (Ok(s), Ok(e)) = (start.trim().parse::<i64>(), end.trim().parse::<i64>()) {
            return (s..=e).collect();
        }
    }
    if let Ok(v) = trimmed.parse::<i64>() {
        return vec![v];
    }
    vec![]
}
