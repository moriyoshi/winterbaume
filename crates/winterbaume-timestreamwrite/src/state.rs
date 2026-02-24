use std::collections::HashMap;

use chrono::Utc;
use serde_json::Value;
use thiserror::Error;

use crate::types::{
    BatchLoadTask, Database, MagneticStoreWriteProperties, Record, RetentionProperties, Table,
};

/// In-memory state for the Timestream Write service.
#[derive(Debug, Default)]
pub struct TimestreamWriteState {
    /// Databases keyed by database name.
    pub databases: HashMap<String, Database>,
    /// Tables keyed by (database_name, table_name).
    pub tables: HashMap<(String, String), Table>,
    /// Batch load tasks keyed by task_id.
    pub batch_load_tasks: HashMap<String, BatchLoadTask>,
}

/// Error type for Timestream Write operations.
#[derive(Debug, Error)]
pub enum TimestreamWriteError {
    #[error("Database {database_name} already exists in account {account_id} and region {region}")]
    DatabaseAlreadyExists {
        database_name: String,
        account_id: String,
        region: String,
    },

    #[error("Table {table_name} already exists in database {database_name}")]
    TableAlreadyExists {
        database_name: String,
        table_name: String,
    },

    #[error(
        "Database {database_name} contains tables. Delete all tables before deleting the database."
    )]
    DatabaseNotEmpty { database_name: String },

    #[error(
        "Task {task_id} cannot be resumed because its status is {task_status}. Only FAILED tasks can be resumed."
    )]
    TaskCannotBeResumed {
        task_id: String,
        task_status: String,
    },

    #[error("The database {database_name} does not exist.")]
    DatabaseNotFound { database_name: String },

    #[error("The table {table_name} does not exist in database {database_name}.")]
    TableNotFound {
        database_name: String,
        table_name: String,
    },

    #[error("The resource {resource_arn} does not exist.")]
    ResourceNotFound { resource_arn: String },

    #[error("The task {task_id} does not exist.")]
    TaskNotFound { task_id: String },
}

impl TimestreamWriteState {
    // --- Database operations ---

    pub fn create_database(
        &mut self,
        database_name: &str,
        account_id: &str,
        region: &str,
        kms_key_id: Option<&str>,
    ) -> Result<&Database, TimestreamWriteError> {
        if self.databases.contains_key(database_name) {
            return Err(TimestreamWriteError::DatabaseAlreadyExists {
                database_name: database_name.to_string(),
                account_id: account_id.to_string(),
                region: region.to_string(),
            });
        }

        let arn = format!("arn:aws:timestream:{region}:{account_id}:database/{database_name}");

        let now = Utc::now();
        let database = Database {
            database_name: database_name.to_string(),
            arn,
            kms_key_id: kms_key_id.map(|s| s.to_string()),
            table_count: 0,
            creation_time: now,
            last_updated_time: now,
            tags: HashMap::new(),
        };

        self.databases.insert(database_name.to_string(), database);
        Ok(self.databases.get(database_name).unwrap())
    }

    pub fn describe_database(
        &self,
        database_name: &str,
    ) -> Result<&Database, TimestreamWriteError> {
        self.databases
            .get(database_name)
            .ok_or_else(|| TimestreamWriteError::DatabaseNotFound {
                database_name: database_name.to_string(),
            })
    }

    pub fn delete_database(&mut self, database_name: &str) -> Result<(), TimestreamWriteError> {
        if !self.databases.contains_key(database_name) {
            return Err(TimestreamWriteError::DatabaseNotFound {
                database_name: database_name.to_string(),
            });
        }
        // Check if there are tables in the database
        let has_tables = self.tables.keys().any(|(db, _)| db == database_name);
        if has_tables {
            return Err(TimestreamWriteError::DatabaseNotEmpty {
                database_name: database_name.to_string(),
            });
        }
        self.databases.remove(database_name);
        Ok(())
    }

    pub fn list_databases(&self) -> Vec<&Database> {
        let mut dbs: Vec<&Database> = self.databases.values().collect();
        dbs.sort_by(|a, b| a.database_name.cmp(&b.database_name));
        dbs
    }

    pub fn update_database(
        &mut self,
        database_name: &str,
        kms_key_id: &str,
    ) -> Result<&Database, TimestreamWriteError> {
        let db = self.databases.get_mut(database_name).ok_or_else(|| {
            TimestreamWriteError::DatabaseNotFound {
                database_name: database_name.to_string(),
            }
        })?;
        db.kms_key_id = Some(kms_key_id.to_string());
        db.last_updated_time = Utc::now();
        Ok(db)
    }

    // --- Table operations ---

    pub fn create_table(
        &mut self,
        database_name: &str,
        table_name: &str,
        account_id: &str,
        region: &str,
        retention_properties: Option<RetentionProperties>,
        magnetic_store_write_properties: Option<MagneticStoreWriteProperties>,
    ) -> Result<&Table, TimestreamWriteError> {
        if !self.databases.contains_key(database_name) {
            return Err(TimestreamWriteError::DatabaseNotFound {
                database_name: database_name.to_string(),
            });
        }

        let key = (database_name.to_string(), table_name.to_string());
        if self.tables.contains_key(&key) {
            return Err(TimestreamWriteError::TableAlreadyExists {
                database_name: database_name.to_string(),
                table_name: table_name.to_string(),
            });
        }

        let arn = format!(
            "arn:aws:timestream:{region}:{account_id}:database/{database_name}/table/{table_name}"
        );
        let now = Utc::now();
        let table = Table {
            table_name: table_name.to_string(),
            database_name: database_name.to_string(),
            arn,
            table_status: "ACTIVE".to_string(),
            retention_properties: retention_properties.unwrap_or_default(),
            magnetic_store_write_properties: magnetic_store_write_properties.unwrap_or_default(),
            creation_time: now,
            last_updated_time: now,
            tags: HashMap::new(),
            records: Vec::new(),
        };

        self.tables.insert(key.clone(), table);

        // Update table count
        if let Some(db) = self.databases.get_mut(database_name) {
            db.table_count += 1;
        }

        Ok(self.tables.get(&key).unwrap())
    }

    pub fn describe_table(
        &self,
        database_name: &str,
        table_name: &str,
    ) -> Result<&Table, TimestreamWriteError> {
        if !self.databases.contains_key(database_name) {
            return Err(TimestreamWriteError::DatabaseNotFound {
                database_name: database_name.to_string(),
            });
        }
        let key = (database_name.to_string(), table_name.to_string());
        self.tables
            .get(&key)
            .ok_or_else(|| TimestreamWriteError::TableNotFound {
                database_name: database_name.to_string(),
                table_name: table_name.to_string(),
            })
    }

    pub fn delete_table(
        &mut self,
        database_name: &str,
        table_name: &str,
    ) -> Result<(), TimestreamWriteError> {
        if !self.databases.contains_key(database_name) {
            return Err(TimestreamWriteError::DatabaseNotFound {
                database_name: database_name.to_string(),
            });
        }
        let key = (database_name.to_string(), table_name.to_string());
        match self.tables.remove(&key) {
            Some(_) => {
                if let Some(db) = self.databases.get_mut(database_name) {
                    db.table_count -= 1;
                }
                Ok(())
            }
            None => Err(TimestreamWriteError::TableNotFound {
                database_name: database_name.to_string(),
                table_name: table_name.to_string(),
            }),
        }
    }

    pub fn list_tables(&self, database_name: &str) -> Result<Vec<&Table>, TimestreamWriteError> {
        if !self.databases.contains_key(database_name) {
            return Err(TimestreamWriteError::DatabaseNotFound {
                database_name: database_name.to_string(),
            });
        }
        let mut tables: Vec<&Table> = self
            .tables
            .values()
            .filter(|t| t.database_name == database_name)
            .collect();
        tables.sort_by(|a, b| a.table_name.cmp(&b.table_name));
        Ok(tables)
    }

    pub fn update_table(
        &mut self,
        database_name: &str,
        table_name: &str,
        retention_properties: Option<RetentionProperties>,
        magnetic_store_write_properties: Option<MagneticStoreWriteProperties>,
    ) -> Result<&Table, TimestreamWriteError> {
        if !self.databases.contains_key(database_name) {
            return Err(TimestreamWriteError::DatabaseNotFound {
                database_name: database_name.to_string(),
            });
        }
        let key = (database_name.to_string(), table_name.to_string());
        let table =
            self.tables
                .get_mut(&key)
                .ok_or_else(|| TimestreamWriteError::TableNotFound {
                    database_name: database_name.to_string(),
                    table_name: table_name.to_string(),
                })?;

        if let Some(rp) = retention_properties {
            table.retention_properties = rp;
        }
        if let Some(mswp) = magnetic_store_write_properties {
            table.magnetic_store_write_properties = mswp;
        }
        table.last_updated_time = Utc::now();
        Ok(table)
    }

    // --- WriteRecords ---

    pub fn write_records(
        &mut self,
        database_name: &str,
        table_name: &str,
        records: Vec<Record>,
    ) -> Result<i32, TimestreamWriteError> {
        if !self.databases.contains_key(database_name) {
            return Err(TimestreamWriteError::DatabaseNotFound {
                database_name: database_name.to_string(),
            });
        }
        let key = (database_name.to_string(), table_name.to_string());
        let table =
            self.tables
                .get_mut(&key)
                .ok_or_else(|| TimestreamWriteError::TableNotFound {
                    database_name: database_name.to_string(),
                    table_name: table_name.to_string(),
                })?;

        let count = records.len() as i32;
        table.records.extend(records);
        Ok(count)
    }

    // --- Tag operations ---

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), TimestreamWriteError> {
        // Try databases first
        for db in self.databases.values_mut() {
            if db.arn == resource_arn {
                db.tags.extend(tags);
                return Ok(());
            }
        }
        // Try tables
        for table in self.tables.values_mut() {
            if table.arn == resource_arn {
                table.tags.extend(tags);
                return Ok(());
            }
        }
        Err(TimestreamWriteError::ResourceNotFound {
            resource_arn: resource_arn.to_string(),
        })
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), TimestreamWriteError> {
        // Try databases first
        for db in self.databases.values_mut() {
            if db.arn == resource_arn {
                for key in tag_keys {
                    db.tags.remove(key);
                }
                return Ok(());
            }
        }
        // Try tables
        for table in self.tables.values_mut() {
            if table.arn == resource_arn {
                for key in tag_keys {
                    table.tags.remove(key);
                }
                return Ok(());
            }
        }
        Err(TimestreamWriteError::ResourceNotFound {
            resource_arn: resource_arn.to_string(),
        })
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<&HashMap<String, String>, TimestreamWriteError> {
        for db in self.databases.values() {
            if db.arn == resource_arn {
                return Ok(&db.tags);
            }
        }
        for table in self.tables.values() {
            if table.arn == resource_arn {
                return Ok(&table.tags);
            }
        }
        Err(TimestreamWriteError::ResourceNotFound {
            resource_arn: resource_arn.to_string(),
        })
    }

    // --- BatchLoadTask operations ---

    pub fn create_batch_load_task(
        &mut self,
        task_id: String,
        database_name: &str,
        table_name: &str,
        data_source_configuration: Value,
        report_configuration: Value,
        data_model_configuration: Option<Value>,
    ) -> &BatchLoadTask {
        let now = Utc::now();
        let task = BatchLoadTask {
            task_id: task_id.clone(),
            task_status: "CREATED".to_string(),
            database_name: database_name.to_string(),
            table_name: table_name.to_string(),
            data_source_configuration,
            report_configuration,
            data_model_configuration,
            creation_time: now,
            last_updated_time: now,
            error_message: None,
        };
        self.batch_load_tasks.insert(task_id.clone(), task);
        self.batch_load_tasks.get(&task_id).unwrap()
    }

    pub fn describe_batch_load_task(
        &self,
        task_id: &str,
    ) -> Result<&BatchLoadTask, TimestreamWriteError> {
        self.batch_load_tasks
            .get(task_id)
            .ok_or_else(|| TimestreamWriteError::TaskNotFound {
                task_id: task_id.to_string(),
            })
    }

    pub fn list_batch_load_tasks(&self, task_status_filter: Option<&str>) -> Vec<&BatchLoadTask> {
        let mut tasks: Vec<&BatchLoadTask> = self
            .batch_load_tasks
            .values()
            .filter(|t| {
                task_status_filter
                    .map(|s| t.task_status == s)
                    .unwrap_or(true)
            })
            .collect();
        tasks.sort_by(|a, b| a.task_id.cmp(&b.task_id));
        tasks
    }

    pub fn resume_batch_load_task(&mut self, task_id: &str) -> Result<(), TimestreamWriteError> {
        let task = self.batch_load_tasks.get_mut(task_id).ok_or_else(|| {
            TimestreamWriteError::TaskNotFound {
                task_id: task_id.to_string(),
            }
        })?;
        if task.task_status != "FAILED" {
            return Err(TimestreamWriteError::TaskCannotBeResumed {
                task_id: task_id.to_string(),
                task_status: task.task_status.clone(),
            });
        }
        task.task_status = "IN_PROGRESS".to_string();
        task.last_updated_time = Utc::now();
        Ok(())
    }
}
