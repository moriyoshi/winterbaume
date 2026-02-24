use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct KeyspacesState {
    /// Keyspaces keyed by name.
    pub keyspaces: HashMap<String, Keyspace>,
    /// Tables keyed by (keyspace_name, table_name).
    pub tables: HashMap<(String, String), Table>,
    /// User-defined types keyed by (keyspace_name, type_name).
    pub types: HashMap<(String, String), UserDefinedType>,
}

/// Domain-specific error enum. Contains no HTTP status codes or AWS error type strings.
#[derive(Debug, Error)]
pub enum KeyspacesError {
    #[error("Resource not found: {resource_type} {name}")]
    NotFound {
        resource_type: &'static str,
        name: String,
    },
    #[error("Resource already exists: {resource_type} {name}")]
    AlreadyExists {
        resource_type: &'static str,
        name: String,
    },
    #[error("{message}")]
    Validation { message: String },
    #[error("Resource {name} has a conflict: {detail}")]
    Conflict { name: String, detail: String },
}

impl KeyspacesState {
    // ---- Keyspace CRUD ----

    pub fn create_keyspace(
        &mut self,
        name: &str,
        replication_strategy: &str,
        replication_regions: Vec<String>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<String, KeyspacesError> {
        if self.keyspaces.contains_key(name) {
            return Err(KeyspacesError::AlreadyExists {
                resource_type: "Keyspace",
                name: name.to_string(),
            });
        }
        let arn = format!("arn:aws:cassandra:{region}:{account_id}:/keyspace/{name}/");
        let ks = Keyspace {
            name: name.to_string(),
            arn: arn.clone(),
            replication_strategy: replication_strategy.to_string(),
            replication_regions,
            tags,
            creation_timestamp: Utc::now(),
            status: "ACTIVE".to_string(),
        };
        self.keyspaces.insert(name.to_string(), ks);
        Ok(arn)
    }

    pub fn get_keyspace(&self, name: &str) -> Result<&Keyspace, KeyspacesError> {
        self.keyspaces
            .get(name)
            .ok_or_else(|| KeyspacesError::NotFound {
                resource_type: "Keyspace",
                name: name.to_string(),
            })
    }

    pub fn delete_keyspace(&mut self, name: &str) -> Result<(), KeyspacesError> {
        if self.keyspaces.remove(name).is_none() {
            return Err(KeyspacesError::NotFound {
                resource_type: "Keyspace",
                name: name.to_string(),
            });
        }
        // Also remove all tables in this keyspace.
        self.tables.retain(|(ks, _), _| ks != name);
        // Also remove all types in this keyspace.
        self.types.retain(|(ks, _), _| ks != name);
        Ok(())
    }

    pub fn update_keyspace(
        &mut self,
        name: &str,
        replication_strategy: &str,
        replication_regions: Vec<String>,
    ) -> Result<String, KeyspacesError> {
        let ks = self
            .keyspaces
            .get_mut(name)
            .ok_or_else(|| KeyspacesError::NotFound {
                resource_type: "Keyspace",
                name: name.to_string(),
            })?;
        ks.replication_strategy = replication_strategy.to_string();
        ks.replication_regions = replication_regions;
        Ok(ks.arn.clone())
    }

    pub fn list_keyspaces(&self) -> Vec<&Keyspace> {
        let mut ks: Vec<_> = self.keyspaces.values().collect();
        ks.sort_by_key(|k| &k.name);
        ks
    }

    // ---- Table CRUD ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_table(
        &mut self,
        keyspace_name: &str,
        table_name: &str,
        schema: SchemaDefinition,
        capacity_mode: &str,
        read_capacity_units: Option<i64>,
        write_capacity_units: Option<i64>,
        encryption_type: &str,
        kms_key_identifier: Option<String>,
        point_in_time_recovery: bool,
        ttl_status: &str,
        default_time_to_live: Option<i32>,
        comment: &str,
        client_side_timestamps: bool,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<String, KeyspacesError> {
        // Verify keyspace exists.
        if !self.keyspaces.contains_key(keyspace_name) {
            return Err(KeyspacesError::NotFound {
                resource_type: "Keyspace",
                name: keyspace_name.to_string(),
            });
        }
        let key = (keyspace_name.to_string(), table_name.to_string());
        if self.tables.contains_key(&key) {
            return Err(KeyspacesError::AlreadyExists {
                resource_type: "Table",
                name: format!("{keyspace_name}/{table_name}"),
            });
        }
        let arn = format!(
            "arn:aws:cassandra:{region}:{account_id}:/keyspace/{keyspace_name}/table/{table_name}"
        );
        let table = Table {
            keyspace_name: keyspace_name.to_string(),
            table_name: table_name.to_string(),
            arn: arn.clone(),
            schema_definition: schema,
            capacity_mode: capacity_mode.to_string(),
            read_capacity_units,
            write_capacity_units,
            encryption_type: encryption_type.to_string(),
            kms_key_identifier,
            point_in_time_recovery_enabled: point_in_time_recovery,
            ttl_status: ttl_status.to_string(),
            default_time_to_live,
            comment: comment.to_string(),
            client_side_timestamps_enabled: client_side_timestamps,
            tags,
            creation_timestamp: Utc::now(),
            status: "ACTIVE".to_string(),
        };
        self.tables.insert(key, table);
        Ok(arn)
    }

    pub fn get_table(
        &self,
        keyspace_name: &str,
        table_name: &str,
    ) -> Result<&Table, KeyspacesError> {
        let key = (keyspace_name.to_string(), table_name.to_string());
        self.tables
            .get(&key)
            .ok_or_else(|| KeyspacesError::NotFound {
                resource_type: "Table",
                name: format!("{keyspace_name}/{table_name}"),
            })
    }

    pub fn delete_table(
        &mut self,
        keyspace_name: &str,
        table_name: &str,
    ) -> Result<(), KeyspacesError> {
        let key = (keyspace_name.to_string(), table_name.to_string());
        if self.tables.remove(&key).is_none() {
            return Err(KeyspacesError::NotFound {
                resource_type: "Table",
                name: format!("{keyspace_name}/{table_name}"),
            });
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_table(
        &mut self,
        keyspace_name: &str,
        table_name: &str,
        capacity_mode: Option<&str>,
        read_capacity_units: Option<i64>,
        write_capacity_units: Option<i64>,
        encryption_type: Option<&str>,
        kms_key_identifier: Option<String>,
        point_in_time_recovery: Option<bool>,
        ttl_status: Option<&str>,
        default_time_to_live: Option<i32>,
        client_side_timestamps: Option<bool>,
    ) -> Result<String, KeyspacesError> {
        let key = (keyspace_name.to_string(), table_name.to_string());
        let table = self
            .tables
            .get_mut(&key)
            .ok_or_else(|| KeyspacesError::NotFound {
                resource_type: "Table",
                name: format!("{keyspace_name}/{table_name}"),
            })?;
        if let Some(cm) = capacity_mode {
            table.capacity_mode = cm.to_string();
        }
        if let Some(rcu) = read_capacity_units {
            table.read_capacity_units = Some(rcu);
        }
        if let Some(wcu) = write_capacity_units {
            table.write_capacity_units = Some(wcu);
        }
        if let Some(et) = encryption_type {
            table.encryption_type = et.to_string();
        }
        if kms_key_identifier.is_some() {
            table.kms_key_identifier = kms_key_identifier;
        }
        if let Some(pitr) = point_in_time_recovery {
            table.point_in_time_recovery_enabled = pitr;
        }
        if let Some(ts) = ttl_status {
            table.ttl_status = ts.to_string();
        }
        if let Some(dttl) = default_time_to_live {
            table.default_time_to_live = Some(dttl);
        }
        if let Some(cst) = client_side_timestamps {
            table.client_side_timestamps_enabled = cst;
        }
        Ok(table.arn.clone())
    }

    pub fn list_tables(&self, keyspace_name: &str) -> Result<Vec<&Table>, KeyspacesError> {
        if !self.keyspaces.contains_key(keyspace_name) {
            return Err(KeyspacesError::NotFound {
                resource_type: "Keyspace",
                name: keyspace_name.to_string(),
            });
        }
        let mut tables: Vec<_> = self
            .tables
            .values()
            .filter(|t| t.keyspace_name == keyspace_name)
            .collect();
        tables.sort_by_key(|t| &t.table_name);
        Ok(tables)
    }

    pub fn restore_table(
        &mut self,
        source_keyspace_name: &str,
        source_table_name: &str,
        target_keyspace_name: &str,
        target_table_name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<String, KeyspacesError> {
        // Verify source table exists.
        let source_key = (
            source_keyspace_name.to_string(),
            source_table_name.to_string(),
        );
        let source = self
            .tables
            .get(&source_key)
            .ok_or_else(|| KeyspacesError::NotFound {
                resource_type: "Table",
                name: format!("{source_keyspace_name}/{source_table_name}"),
            })?
            .clone();

        // Verify target keyspace exists.
        if !self.keyspaces.contains_key(target_keyspace_name) {
            return Err(KeyspacesError::NotFound {
                resource_type: "Keyspace",
                name: target_keyspace_name.to_string(),
            });
        }

        let target_key = (
            target_keyspace_name.to_string(),
            target_table_name.to_string(),
        );
        if self.tables.contains_key(&target_key) {
            return Err(KeyspacesError::AlreadyExists {
                resource_type: "Table",
                name: format!("{target_keyspace_name}/{target_table_name}"),
            });
        }

        let arn = format!(
            "arn:aws:cassandra:{region}:{account_id}:/keyspace/{target_keyspace_name}/table/{target_table_name}"
        );
        let table = Table {
            keyspace_name: target_keyspace_name.to_string(),
            table_name: target_table_name.to_string(),
            arn: arn.clone(),
            schema_definition: source.schema_definition,
            capacity_mode: source.capacity_mode,
            read_capacity_units: source.read_capacity_units,
            write_capacity_units: source.write_capacity_units,
            encryption_type: source.encryption_type,
            kms_key_identifier: source.kms_key_identifier,
            point_in_time_recovery_enabled: source.point_in_time_recovery_enabled,
            ttl_status: source.ttl_status,
            default_time_to_live: source.default_time_to_live,
            comment: source.comment,
            client_side_timestamps_enabled: source.client_side_timestamps_enabled,
            tags: source.tags,
            creation_timestamp: Utc::now(),
            status: "ACTIVE".to_string(),
        };
        self.tables.insert(target_key, table);
        Ok(arn)
    }

    // ---- Type CRUD ----

    pub fn create_type(
        &mut self,
        keyspace_name: &str,
        type_name: &str,
        field_definitions: Vec<FieldDefinition>,
    ) -> Result<String, KeyspacesError> {
        if !self.keyspaces.contains_key(keyspace_name) {
            return Err(KeyspacesError::NotFound {
                resource_type: "Keyspace",
                name: keyspace_name.to_string(),
            });
        }
        let key = (keyspace_name.to_string(), type_name.to_string());
        if self.types.contains_key(&key) {
            return Err(KeyspacesError::AlreadyExists {
                resource_type: "Type",
                name: format!("{keyspace_name}/{type_name}"),
            });
        }
        // Types don't have a separate ARN field in the API, but GetType returns keyspaceArn.
        // The response uses keyspaceArn, not a type-specific ARN.
        let result = format!("{keyspace_name}.{type_name}");
        let udt = UserDefinedType {
            keyspace_name: keyspace_name.to_string(),
            type_name: type_name.to_string(),
            field_definitions,
            creation_timestamp: Utc::now(),
            status: "ACTIVE".to_string(),
        };
        self.types.insert(key, udt);
        Ok(result)
    }

    pub fn get_type(
        &self,
        keyspace_name: &str,
        type_name: &str,
    ) -> Result<&UserDefinedType, KeyspacesError> {
        let key = (keyspace_name.to_string(), type_name.to_string());
        self.types
            .get(&key)
            .ok_or_else(|| KeyspacesError::NotFound {
                resource_type: "Type",
                name: format!("{keyspace_name}/{type_name}"),
            })
    }

    pub fn delete_type(
        &mut self,
        keyspace_name: &str,
        type_name: &str,
    ) -> Result<(), KeyspacesError> {
        let key = (keyspace_name.to_string(), type_name.to_string());
        if self.types.remove(&key).is_none() {
            return Err(KeyspacesError::NotFound {
                resource_type: "Type",
                name: format!("{keyspace_name}/{type_name}"),
            });
        }
        Ok(())
    }

    pub fn list_types(&self, keyspace_name: &str) -> Result<Vec<&UserDefinedType>, KeyspacesError> {
        if !self.keyspaces.contains_key(keyspace_name) {
            return Err(KeyspacesError::NotFound {
                resource_type: "Keyspace",
                name: keyspace_name.to_string(),
            });
        }
        let mut types: Vec<_> = self
            .types
            .values()
            .filter(|t| t.keyspace_name == keyspace_name)
            .collect();
        types.sort_by_key(|t| &t.type_name);
        Ok(types)
    }

    // ---- Tag operations ----

    /// Get tags for a resource by ARN. Returns empty map if no tags found.
    pub fn get_tags_for_resource(
        &self,
        arn: &str,
    ) -> Result<HashMap<String, String>, KeyspacesError> {
        // Find resource by ARN.
        for ks in self.keyspaces.values() {
            if ks.arn == arn {
                return Ok(ks.tags.clone());
            }
        }
        for table in self.tables.values() {
            if table.arn == arn {
                return Ok(table.tags.clone());
            }
        }
        Err(KeyspacesError::NotFound {
            resource_type: "Resource",
            name: arn.to_string(),
        })
    }

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), KeyspacesError> {
        // Find resource by ARN and merge tags.
        for ks in self.keyspaces.values_mut() {
            if ks.arn == arn {
                ks.tags.extend(tags);
                return Ok(());
            }
        }
        for table in self.tables.values_mut() {
            if table.arn == arn {
                table.tags.extend(tags);
                return Ok(());
            }
        }
        Err(KeyspacesError::NotFound {
            resource_type: "Resource",
            name: arn.to_string(),
        })
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: &[String]) -> Result<(), KeyspacesError> {
        // Find resource by ARN and remove tags.
        for ks in self.keyspaces.values_mut() {
            if ks.arn == arn {
                for key in tag_keys {
                    ks.tags.remove(key);
                }
                return Ok(());
            }
        }
        for table in self.tables.values_mut() {
            if table.arn == arn {
                for key in tag_keys {
                    table.tags.remove(key);
                }
                return Ok(());
            }
        }
        Err(KeyspacesError::NotFound {
            resource_type: "Resource",
            name: arn.to_string(),
        })
    }
}
